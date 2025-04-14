use std::sync::Arc;

use tokio::sync::RwLock;

use crate::{
	channels::{Signal, SignalReceiver, SignalSender},
	get_text_engine,
	memory::{entry::Entry, store::Store},
	vision::{capture::capture_window, window::get_focused_window},
};

pub async fn run(
	mut vision_receiver: SignalReceiver,
	embed_sender: SignalSender,
	store: Arc<RwLock<Store>>,
) {
	while let Some(signal) = vision_receiver.recv().await {
		match signal {
			Signal::ForceCapture => {
				if let Some(window) = get_focused_window() {
					let image = capture_window(&window);

					let mut entry = Entry::new(image.clone())
						.with_window_title(window.title().unwrap_or_default())
						.with_app_name(window.app_name().unwrap_or_default());

					let extracted_lines = get_text_engine().recognize(image.into());
					entry = entry.with_vector_text(extracted_lines);

					// println!("[vision] new entry: {}", entry);

					let index = store.write().await.add(entry);

					if let Err(err) = embed_sender.try_send(Signal::Embed(index)) {
						eprintln!("[vision] failed to send embed signal: {err}");
					}
				} else {
					eprintln!("[vision] no focused window found");
				}
			}
			Signal::SoftCapture => {
				// Futuramente: capturar só se houver scroll, mouse move etc.
				println!("[vision] soft capture triggered");
			}
			_ => {}
		}
	}
}
