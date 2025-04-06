use std::{
	thread::sleep,
	time::{Duration, Instant},
};

use enigo::{Enigo, Keyboard};
use rdev::{simulate, EventType, Key};

const BACKSPACE_DELAY_MS: u64 = 5;

pub struct Typer {
	enigo: Enigo,
}

impl Typer {
	pub fn new() -> Self {
		let settings = enigo::Settings::default();
		let enigo = Enigo::new(&settings).expect("[typer] failed to initialize Typer");
		Self { enigo }
	}

	pub fn type_text(&mut self, text: &str) {
		if self.enigo.text(text).is_err() {
			eprintln!("[typer] failed to type text");
		}
	}

	pub fn backspace(&mut self, times: usize) {
		if times <= 1 {
			return;
		}
		let delay = Duration::from_millis(15);
		for _ in 1..times {
			if simulate(&EventType::KeyPress(Key::Backspace)).is_err() {
				eprintln!("[typer] failed to backspace");
			}
			sleep(delay);
		}
	}

	pub fn backspace_for(&mut self, duration_ms: u64) {
		let now = Instant::now();
		while now.elapsed().as_millis() < duration_ms as u128 {
			if simulate(&EventType::KeyPress(Key::Backspace)).is_err() {
				eprintln!("[typer] failed to backspace");
			}
			std::thread::sleep(std::time::Duration::from_millis(10));
		}
	}

	pub fn enter(&mut self) {
		if simulate(&EventType::KeyPress(Key::Return)).is_err() {
			eprintln!("[typer] failed to press enter");
		}
	}

	pub fn clear_line(&mut self, len: usize) {
		self.backspace(len);
	}
}
