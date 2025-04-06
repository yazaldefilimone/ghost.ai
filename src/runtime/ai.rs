use std::sync::{Arc, Mutex};

use tokio::sync::mpsc::Receiver;

use crate::{
	llm,
	runtime::{state::SharedState, typer::Typer},
	IS_PAUSED,
};

pub async fn complete_suggestion(state: Arc<SharedState>, mut trigger_rx: Receiver<()>) {
	// println!("[ai] thinking...");
	// tokio::spawn(async move {
	while (trigger_rx.recv().await).is_some() {
		println!("[ai] thinking...");
		IS_PAUSED.store(true, std::sync::atomic::Ordering::SeqCst);
		let (typed, history) = {
			let text_state = state.text_state.lock().unwrap();
			let memory = state.memory.lock().unwrap();
			let typed = text_state.get_typed().to_owned();
			(typed, memory.to_string())
		};
		println!("[ai] typed: {}", typed);
		match llm::generate_completion(&typed, history).await {
			Ok(suggestion) => {
				println!("[ai] suggestion: {}", suggestion);
				// todo: move typer for global... don't allocate for each event
				let mut typer = Typer::new();
				typer.type_text(suggestion.as_str());
				let mut text_state = state.text_state.lock().unwrap();
				text_state.add_suggestion(suggestion);
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

pub async fn stream_suggestion(state: Arc<SharedState>, mut trigger_rx: Receiver<()>) {
	tokio::spawn(async move {
		while trigger_rx.recv().await.is_some() {
			println!("[ai] thinking...");
			IS_PAUSED.store(true, std::sync::atomic::Ordering::SeqCst);
			let (typed, history) = {
				let text_state = state.text_state.lock().unwrap();
				let memory = state.memory.lock().unwrap();
				let typed = text_state.get_typed().to_owned();
				(typed, memory.to_string())
			};

			let suggestion_acc = Arc::new(Mutex::new(String::new()));
			let acc_clone = suggestion_acc.clone(); // clone pra usar no closure

			let result = llm::stream_completion(&typed, history, move |chunk| {
				if let Ok(mut acc) = acc_clone.lock() {
					acc.push_str(chunk.as_str());
				}
				let mut typer = Typer::new();
				typer.type_text(chunk.as_str());
			})
			.await;

			if result.is_ok() {
				if let Ok(acc) = suggestion_acc.lock() {
					let mut text_state = state.text_state.lock().unwrap();
					text_state.add_suggestion(acc.to_owned());
				}
			} else {
				eprintln!("[ai] failed to stream completion");
			}
			IS_PAUSED.store(false, std::sync::atomic::Ordering::SeqCst);
		}
	});
}
