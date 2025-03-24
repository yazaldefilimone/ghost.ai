use enigo::{Enigo, Keyboard, Settings};
use rdev::{listen, Event, EventType, Key};
use std::{
	sync::{Arc, Mutex},
	thread,
	time::Duration,
};

mod ai;
mod engine;
mod state;
mod typer;
fn key_to_char(key: Key) -> Option<char> {
	match key {
		Key::Num0 => Some('0'),
		Key::Num1 => Some('1'),
		Key::Num2 => Some('2'),
		Key::Num3 => Some('3'),
		Key::Num4 => Some('4'),
		Key::Num5 => Some('5'),
		Key::Num6 => Some('6'),
		Key::Num7 => Some('7'),
		Key::Num8 => Some('8'),
		Key::Num9 => Some('9'),
		Key::KeyA => Some('a'),
		Key::KeyB => Some('b'),
		Key::KeyC => Some('c'),
		Key::KeyD => Some('d'),
		Key::KeyE => Some('e'),
		Key::KeyF => Some('f'),
		Key::KeyG => Some('g'),
		Key::KeyH => Some('h'),
		Key::KeyI => Some('i'),
		Key::KeyJ => Some('j'),
		Key::KeyK => Some('k'),
		Key::KeyL => Some('l'),
		Key::KeyM => Some('m'),
		Key::KeyN => Some('n'),
		Key::KeyO => Some('o'),
		Key::KeyP => Some('p'),
		Key::KeyQ => Some('q'),
		Key::KeyR => Some('r'),
		Key::KeyS => Some('s'),
		Key::KeyT => Some('t'),
		Key::KeyU => Some('u'),
		Key::KeyV => Some('v'),
		Key::KeyW => Some('w'),
		Key::KeyX => Some('x'),
		Key::KeyY => Some('y'),
		Key::KeyZ => Some('z'),
		Key::Space => Some(' '),

		_ => None,
	}
}

fn main() {
	let buffer = Arc::new(Mutex::new(String::new()));
	let buffer_clone = Arc::clone(&buffer);

	thread::spawn(move || loop {
		thread::sleep(Duration::from_secs(2));

		let text = {
			let mut buf = buffer_clone.lock().unwrap();
			if buf.is_empty() {
				continue;
			}
			let out = buf.clone();
			buf.clear();
			out
		};
		println!("📤 texto capturado: {}", text);
		let mut enigo = Enigo::new(&Settings::default()).unwrap();
		enigo.text(&text).unwrap();
	});

	// Captura global
	if let Err(error) = listen(move |event: Event| {
		if let EventType::KeyPress(key) = event.event_type {
			if let Some(c) = key_to_char(key) {
				buffer.lock().unwrap().push(c);
			}
		}
	}) {
		println!("Erro ao escutar eventos: {:?}", error)
	}
}
