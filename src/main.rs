use std::sync::{Arc, Mutex};

use ai::llm::{self};
use engine::{parser::parse_key, state::KeyboardState, Engine};
use rdev::{listen, simulate, Event, EventType, Key};
use tokio::sync::mpsc;
mod ai;
mod engine;
mod typer;

#[tokio::main]
async fn main() {
	let engine = Arc::new(Mutex::new(Engine::new()));
	let (tx, mut rx) = mpsc::channel::<()>(1);
	let engine_clone = Arc::clone(&engine);

	// Task async para processar sugestões
	tokio::spawn(async move {
		while rx.recv().await.is_some() {
			let prompt = {
				let eng = engine_clone.lock().unwrap();
				if eng.is_empty() {
					continue;
				}
				eng.get_text().to_string()
			};
			println!("prompt: {}", prompt);
			match llm::generate_suggestion(&prompt).await {
				Ok(suggestion) => {
					println!("suggestion: {}", suggestion);
					typer::type_text(&suggestion);
					let mut eng = engine_clone.lock().unwrap();
					eng.apply_suggestion(&suggestion);
				}
				Err(error) => eprintln!("hmmm, something went wrong with the llm: {:?}", error),
			}
		}
	});

	// Escuta de teclado
	if let Err(error) = listen(move |event: Event| {
		static mut KEY_STATE: Option<KeyboardState> = None;
		unsafe {
			let key_state = KEY_STATE.get_or_insert_with(KeyboardState::new);
			match event.event_type {
				EventType::KeyPress(key) => match key {
					Key::Tab => {
						engine.lock().unwrap().in_suggestion = true;
						let _ = tx.try_send(());
					}
					Key::Backspace => {
						let mut eng = engine.lock().unwrap();
						if let Some(removed) = eng.backspace() {
							for _ in 0..removed.len() {
								let _ = simulate(&EventType::KeyPress(Key::Backspace));
							}
						}
					}
					_ => {
						key_state.update(&key, true);
						if let Some(c) = parse_key(key, key_state) {
							key_state.reset();
							engine.lock().unwrap().push_char(c);
						}
					}
				},
				EventType::KeyRelease(key) => {
					key_state.update(&key, false);
				}
				_ => {}
			}
		}
	}) {
		eprintln!("Erro ao escutar eventos: {:?}", error);
	}
}
