use std::sync::Arc;

use tokio::sync::mpsc::Receiver;

use crate::{
	llm,
	runtime::{state::SharedState, typer::Typer},
};

pub async fn complete_suggestion(state: Arc<SharedState>, mut trigger_rx: Receiver<()>) {
	// println!("[ai] thinking...");
	// tokio::spawn(async move {
	while (trigger_rx.recv().await).is_some() {
		println!("[ai] thinking...");
		let typed = {
			let text_state = state.text_state.lock().unwrap();
			let history = text_state.get_history();
			text_state.get_typed().to_owned()
		};
		println!("[ai] typed: {}", typed);
		match llm::generate_completion(&typed).await {
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
		// state.unmute();
	}
	// });
}

pub async fn stream_suggestion(state: Arc<SharedState>, mut trigger_rx: Receiver<()>) {
	tokio::spawn(async move {
		// state.mute();
		while trigger_rx.recv().await.is_some() {
			println!("[ai] thinking...");
			let typed = {
				let text_state = state.text_state.lock().unwrap();
				text_state.get_typed().to_owned()
			};
			// let mut suggestion_acc = String::new();
			let result = llm::stream_completion(&typed, |chunk| {
				// todo: move typer for global... don't allocate for each event
				let mut typer = Typer::new();
				typer.type_text(chunk.as_str());
				// suggestion_acc.push_str(chunk.as_str());
			})
			.await;
			if result.is_ok() {
				// let mut text_state = state.text_state.lock().unwrap();
				// text_state.add_suggestion(sugg estion_acc);}
			} else {
				eprintln!("[ai] failed to stream completion");
			}
		}
		// state.unmute();
	});
}
