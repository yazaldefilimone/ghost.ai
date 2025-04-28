use rdev::{Button, Event, EventType, Key, Keyboard, KeyboardState, ListenError};
use std::thread;

use crate::{
	keyboard_error, keyboard_info,
	settings::loader,
	signal::{Signal, SignalSender},
	text_buffer::TextBuffer,
};

pub async fn start_keyboard_listener(vision: SignalSender, typer: SignalSender) {
	thread::spawn(move || {
		let mut keyboard = Keyboard::new().unwrap();
		let mut buffer = TextBuffer::new();
		let mut pressed: Vec<Key> = Vec::with_capacity(3);
		let result = rdev::listen(move |event| {
			handle_event(event, &mut keyboard, &mut buffer, &mut pressed, &vision, &typer)
		});
		if let Err(err) = result {
			print_listen_error(err);
		}
	});
}

fn handle_event(
	event: Event,
	keyboard: &mut Keyboard,
	buffer: &mut TextBuffer,
	pressed_keys: &mut Vec<Key>,
	vision: &SignalSender,
	typer: &SignalSender,
) {
	match event.event_type {
		EventType::ButtonPress(Button::Left) => {
			// Optional: trigger vision on mouse click
			let _ = vision.try_send(Signal::ForceCapture);
		}

		EventType::KeyRelease(key) => {
			pressed_keys.retain(|k| k != &key);
		}
		EventType::KeyPress(Key::Return) | EventType::KeyPress(Key::KpReturn) => {
			pressed_keys.clear();
		}
		EventType::KeyPress(key) => {
			pressed_keys.push(key);
			let settings = loader::current_settings();
			let shortcuts = &settings.shortcuts;

			if shortcuts.autocomplete_shortcut(pressed_keys) {
				keyboard_info!("triggering autocomplete");
				let _ = typer.try_send(Signal::Type(buffer.get_text()));
				return;
			}

			if shortcuts.look_shortcut(pressed_keys) {
				keyboard_info!("triggering look");
				let _ = vision.try_send(Signal::ForceCapture);
				return;
			}
			match key {
				Key::Backspace => buffer.backspace(),
				Key::LeftArrow => buffer.move_cursor_left(),
				Key::RightArrow => buffer.move_cursor_right(),
				other => {
					if let Some(text) = keyboard.add(&EventType::KeyPress(other)) {
						buffer.insert_str(&text);
					}
				}
			}
		}

		other_event => {
			if let Some(text) = keyboard.add(&other_event) {
				buffer.insert_str(&text);
			}
		}
	}
}

fn print_listen_error(err: ListenError) {
	use ListenError::*;
	let message = match err {
		EventTapError => "can't create event listener. check accessibility permissions.",
		LoopSourceError => "internal event loop error.",
		MissingDisplayError => "DISPLAY not set. Are you in a graphical session?",
		KeyboardError => "can't access keyboard. check permissions.",
		RecordContextEnablingError => "failed to enable record context.",
		RecordContextError => "record context error.",
		XRecordExtensionError => "XRecord extension not available.",
		KeyHookError(code) => return println!("keyboard hook failed (error code {code})."),
		MouseHookError(code) => return println!("mouse hook failed (error code {code})."),
		err => return println!("someting went wrong, please report this bug: {err:?}"),
	};
	keyboard_error!("{message}");
}
