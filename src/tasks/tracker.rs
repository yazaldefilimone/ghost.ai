use std::{sync::Arc, thread};

use rdev::{Button, EventType, Key, Keyboard, KeyboardState};

use crate::{
	channels::{Signal, SignalSender},
	runtime::state::SharedState,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub async fn run(vision_sender: SignalSender, llm_sender: SignalSender, state: Arc<SharedState>) {
	thread::spawn(move || {
		let mut keyboard = rdev::Keyboard::new().unwrap();
		let result = rdev::listen(move |event| {
			keyboard_event(event, &mut keyboard, &vision_sender, &llm_sender, &state);
		});
		if result.is_err() {
			eprintln!("[keyboard] failed to listen for keyboard events");
		}
	});
}

fn keyboard_event(
	event: rdev::Event,
	board: &mut Keyboard,
	vision_sender: &SignalSender,
	llm_sender: &SignalSender,
	state: &SharedState,
) {
	let mut text_state = state.text_state.lock().unwrap();
	match event.event_type {
		EventType::ButtonPress(mouse_btn) => match mouse_btn {
			Button::Left => {
				if let Err(err) = vision_sender.try_send(Signal::ForceCapture) {
					println!("failed to send vision signal: {}", err);
				}
				// println!("left click");
			}
			Button::Right => {
				// println!("right click");
			}
			Button::Middle => {
				// println!("middle click");
			}
			Button::Unknown(btn) => {}
		},
		EventType::Wheel { delta_y, delta_x } => {
			// println!("wheel {} x {}", delta_y, delta_x);
		}
		EventType::MouseMove { x, y } => {
			// println!("mouse move {}, {}", x, y);
		}
		EventType::KeyPress(Key::Tab) => {
			// println!("tab!");
			if let Err(err) = llm_sender.try_send(Signal::Llm) {
				println!("failed to send llm signal: {}", err);
			}
		}
		event => {
			if let Some(character) = board.add(&event) {
				// println!("character: {}", character);
				text_state.add_text(character.as_str());
			}
		}
	};
}
