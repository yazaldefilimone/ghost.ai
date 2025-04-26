use sea_orm::DbConn;
use std::sync::Arc;

use crate::{
	constants::{AUTO_COMPLETE_MAX_TOKENS, TEMPERATURE},
	llm::manager,
	settings::loader,
	signal::{Signal, SignalReceiver},
	typer, typer_error,
};

pub async fn autocomplete_listener(mut receiver: SignalReceiver, db: Arc<DbConn>) {
	let settings = loader::current_settings();
	let use_streaming = settings.autocomplete.stream;
	let llm_model = &settings.autocomplete.model;
	// todo: add temperature and max tokens to settings?
	let llm = manager::build_provider(llm_model, Some(TEMPERATURE), Some(AUTO_COMPLETE_MAX_TOKENS));
	while let Some(Signal::Type(text)) = receiver.recv().await {
		let db = db.clone();
		let text = text.as_str();

		if use_streaming {
			if let Err(e) = llm.complete_stream(text, db, |r| typer::typer(r.as_str())).await {
				typer_error!("failed to stream auto complete, reason: {e}");
			}
			return;
		}
		match llm.complete(text, db).await {
			Ok(response) => typer::typer(response.as_str()),
			Err(e) => typer_error!("failed to auto complete, reason: {e}"),
		}
	}
}

#[allow(dead_code)]
pub async fn backspace_listener(mut receiver: SignalReceiver) {
	while let Some(signal) = receiver.recv().await {
		if let Signal::Backspace(times) = signal {
			typer::backspace(times);
		}
	}
}
