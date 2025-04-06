use std::{
	sync::{atomic::Ordering, Arc},
	thread,
};

use rdev::{Event, EventType, Key, KeyboardState};
use tokio::sync::mpsc::{error::TrySendError, Sender};

use crate::{runtime::state::SharedState, IS_PAUSED};

use super::typer::Typer;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub async fn listen(state: Arc<SharedState>, trigger_tx: Sender<()>) {
	thread::spawn(move || {
		let result = rdev::listen(move |event| {
			if let Err(err) = keyboard_event(&state, &trigger_tx, event) {
				eprintln!("[keyboard] failed to handle keyboard event, reason: '{}'", err);
			}
		});
		if result.is_err() {
			eprintln!("[keyboard] failed to listen for keyboard events");
		}
	});
}

fn keyboard_event(state: &Arc<SharedState>, trigger_tx: &Sender<()>, event: Event) -> Result<()> {
	if IS_PAUSED.load(Ordering::SeqCst) {
		return Ok(());
	}
	let mut text_state = state.text_state.lock().unwrap();
	let mut keyboard = state.keyboard.lock().unwrap();
	let mut memory = state.memory.lock().unwrap();
	if let EventType::KeyPress(key) = event.event_type {
		match key {
			Key::Tab => {
				if text_state.get_typed().is_empty() {
					return Ok(());
				}
				if text_state.has_suggestion() {
					let typed = text_state.apply_typed();
					memory.insert_content(typed)
				}
				match trigger_tx.try_send(()) {
					Err(TrySendError::Full(_)) => {
						eprintln!("[keybard] waiting for llm to finish");
					}
					Err(TrySendError::Closed(_)) => {
						eprintln!("[keybard] trigger channel closed");
					}
					Ok(_) => {
						println!("[keybard] tab: '{}'", text_state.get_typed());
					}
				}
			}
			Key::Backspace => {
				let times = text_state.backspace();
				let mut typer = Typer::new();
				typer.backspace(times);
			}
			Key::Return | Key::KpReturn => {
				let typed = text_state.apply_typed();
				memory.insert_content(typed)
			}
			key => {
				if let Some(character) = keyboard.add(&EventType::KeyPress(key)) {
					text_state.add_text(character.as_str());
				}
			}
		}
	}
	Ok(())
}
