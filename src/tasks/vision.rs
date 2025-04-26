use std::sync::Arc;

use chrono::Utc;
use sea_orm::{ActiveValue::Set, DbConn};

use crate::{
	database::{ActiveModel, Model},
	get_text_engine,
	security::contains_skip_security_pattern,
	settings::loader,
	signal::{Extract, Signal, SignalReceiver, SignalSender},
	vision::{capture, window},
	vision_error, vision_info,
};

pub async fn start_vision_listener(
	mut receiver: SignalReceiver,
	extract_sender: SignalSender,
	db: Arc<DbConn>,
) {
	let settings = loader::current_settings();

	while let Some(Signal::ForceCapture) = receiver.recv().await {
		vision_info!("capture requested");
		let Some(window) = window::lookup_focused_window() else {
			vision_info!("no focused window");
			continue;
		};

		let app = window.app_name().unwrap_or_default();
		if settings.vision.skip_app(&app) {
			vision_info!("skipping app: '{}'", app);
			continue;
		}

		let title = window.title().unwrap_or_default();
		if contains_skip_security_pattern(&title) {
			vision_info!("skipping sensitive: '{} - {}'", app, title);
			continue;
		}

		let already_seen = Model::find_by_window_title_and_app(&db, &app, &title).await;
		if !already_seen.is_empty() {
			vision_info!("duplicate skipped: '{} - {}'", app, title);
			continue;
		}

		let frame = capture::capture_window(&window);
		let extract = Extract::new(app, title, frame);
		if let Err(err) = extract_sender.try_send(Signal::Extract(extract)) {
			vision_error!("can't send to extract: {err}");
		}
	}
}

pub async fn start_extract_listener(mut extract_receiver: SignalReceiver, db: Arc<DbConn>) {
	while let Some(Signal::Extract(entry)) = extract_receiver.recv().await {
		vision_info!("extracting text...");

		let frame = entry.frame.clone();
		let lines = get_text_engine().recognize(frame.into());
		let text = lines.join("\n");
		let now = Utc::now().naive_utc();

		let record = ActiveModel {
			app_name: Set(Some(entry.app_name)),
			window_title: Set(Some(entry.window_title)),
			extracted_text: Set(text),
			created_at: Set(now),
			updated_at: Set(now),
			..Default::default()
		};
		let _ = Model::insert_entry(&db, record).await;
		vision_info!("memoory saved.");
	}
}
