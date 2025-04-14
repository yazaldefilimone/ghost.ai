use std::sync::Arc;

// use rig::{providers::ollama, streaming::StreamingPrompt};
use tokio::sync::{mpsc::Receiver, RwLock};

use crate::{
	channels::{Signal, SignalReceiver},
	llm,
	memory::store::Store,
	runtime::{state::SharedState, typer::Typer},
	IS_PAUSED,
};

pub async fn complete_suggestion(
	state: Arc<SharedState>,
	store: Arc<RwLock<Store>>,

	mut trigger_rx: Receiver<()>,
) {
	// println!("[ai] thinking...");
	// tokio::spawn(async move {
	while (trigger_rx.recv().await).is_some() {
		// println!("[ai] thinking...");
		IS_PAUSED.store(true, std::sync::atomic::Ordering::SeqCst);
		let (typed, history) = {
			let text_state = state.text_state.lock().unwrap();
			let memory = state.memory.lock().unwrap();
			let typed = text_state.get_typed().to_owned();
			(typed, memory.to_string())
		};
		// println!("[ai] typed: {}", typed);
		match llm::generate_completion(&typed, store.clone()).await {
			Ok(suggestion) => {
				// println!("[ai] suggestion: {}", suggestion);
				// todo: move typer for global... don't allocate for each event
				let mut typer = Typer::new();
				typer.type_text(suggestion.as_str());
			}
			Err(_) => {
				eprintln!("[ai] failed to generate suggestion");
			}
		}
		IS_PAUSED.store(false, std::sync::atomic::Ordering::SeqCst);
		// state.unmute();
	}
	// });
}

pub async fn stream_suggestion(
	state: Arc<SharedState>,
	store: Arc<RwLock<Store>>,
	mut trigger_rx: SignalReceiver,
) {
	tokio::spawn(async move {
		while let Some(signal) = trigger_rx.recv().await {
			if let Signal::Llm = signal {
				// println!("[ai] thinking...");
				IS_PAUSED.store(true, std::sync::atomic::Ordering::SeqCst);
				let typed = {
					let text_state = state.text_state.lock().unwrap();
					let typed = text_state.get_typed().to_owned();
					typed
				};

				let result = llm::stream_completion(&typed, store.clone(), move |chunk| {
					let mut typer = Typer::new();
					typer.type_text(chunk.as_str());
				})
				.await;

				if result.is_err() {
					eprintln!("[ai] failed to stream completion");
				}
				IS_PAUSED.store(false, std::sync::atomic::Ordering::SeqCst);
			}
		}
	});
}
