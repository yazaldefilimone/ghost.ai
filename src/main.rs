use ai::{ctx::Context, llm};
use rdev::{listen, Event, EventType, Key, Keyboard, KeyboardState};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

mod ai;
mod engine;
mod typer;

#[tokio::main]
async fn main() {
	let ctx = Arc::new(Mutex::new(Context::new()));
	let buffer = Arc::new(Mutex::new(String::new()));
	let keyboard = Arc::new(Mutex::new(Keyboard::new().unwrap()));

	let (tx, mut rx) = mpsc::channel::<()>(10);

	// Task de sugestão
	let ctx_for_task = Arc::clone(&ctx);
	tokio::spawn(async move {
		while let Some(_) = rx.recv().await {
			println!("[TASK] Recebido trigger");
			let context_data = {
				let ctx_locked = ctx_for_task.lock().unwrap();
				ctx_locked.clone()
			};

			match llm::generate_suggestion(&context_data).await {
				Ok(suggestion) => {
					println!("[TASK] Sugestão: {}", suggestion);

					typer::type_text(&suggestion);
					let mut ctx_locked = ctx_for_task.lock().unwrap();
					ctx_locked.current_input.push_str(&suggestion);
				}
				Err(error) => {
					eprintln!("[TASK] Erro ao gerar sugestão: {:?}", error);
				}
			}
		}

		println!("[TASK] Canal fechado. Encerrando.");
	});

	// Captura de eventos de teclado
	let buffer_clone = Arc::clone(&buffer);
	let ctx_clone = Arc::clone(&ctx);
	let keyboard_clone = Arc::clone(&keyboard);
	let tx_clone = tx.clone();

	if let Err(error) = listen(move |event: Event| {
		let mut keyboard = keyboard_clone.lock().unwrap();

		match event.event_type {
			EventType::KeyPress(key) => match key {
				Key::Tab => {
					println!("[KEY] TAB pressionado");
					if let (Ok(buf), Ok(mut ctx_guard)) = (buffer_clone.lock(), ctx_clone.lock()) {
						ctx_guard.replace_input(buf.clone());

						let tx_clone = tx_clone.clone();
						tokio::spawn(async move {
							if let Err(err) = tx_clone.send(()).await {
								eprintln!("[KEY] Erro ao enviar trigger: {:?}", err);
							}
						});
					}
				}
				Key::Return | Key::KpReturn => {
					if let (Ok(mut buf), Ok(mut ctx_guard)) = (buffer_clone.lock(), ctx_clone.lock()) {
						println!("[retun] current: {}, buf: {}", ctx_guard.current_input, buf);
						ctx_guard.replace_input(buf.clone());
						buf.clear();
					}
				}
				Key::Backspace => {
					if let Ok(mut buf) = buffer_clone.lock() {
						buf.pop();
					}
				}
				_ => {
					if let Some(text) = keyboard.add(&EventType::KeyPress(key)) {
						if let Ok(mut buf) = buffer_clone.lock() {
							buf.push_str(&text);
						}
					}
				}
			},
			_ => {
				if let Some(text) = keyboard.add(&event.event_type) {
					if let Ok(mut buf) = buffer_clone.lock() {
						buf.push_str(&text);
					}
				}
			}
		}
	}) {
		eprintln!("Erro ao escutar eventos: {:?}", error);
	}
}
