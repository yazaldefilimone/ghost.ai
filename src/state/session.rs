use std::time::Instant;

use crate::engine::Engine;

pub struct Session {
	pub engine: Engine,
	pub last_suggestion: String,
	pub in_autocomplete: bool,
	pub last_input_time: Instant,
}

impl Session {
	pub fn new() -> Self {
		let engine = Engine::new();
		let last_suggestion = "auto complete com ghost.ai....".to_string();
		let in_autocomplete = false;
		let last_input_time = Instant::now();
		Self { engine, last_suggestion, in_autocomplete, last_input_time }
	}

	pub fn is_empty(&self) -> bool {
		self.engine.user_input.is_empty()
	}

	pub fn set_autocomplete(&mut self) {
		self.in_autocomplete = true;
	}
	pub fn push_char(&mut self, c: char) {
		self.engine.push_char(c);
		self.last_input_time = Instant::now();
		// self.in_autocomplete = false;
	}

	pub fn backspace(&mut self) -> Option<String> {
		if self.in_autocomplete {
			self.engine.in_suggestion = true;
			self.engine.apply_suggestion(self.last_suggestion.as_str());
			let text = self.engine.backspace();
			self.engine.in_suggestion = false;
			self.in_autocomplete = false;
			return Some(text);
		}
		None
	}

	pub fn get_suggestion(&mut self) -> String {
		self.in_autocomplete = true;
		self.last_suggestion.clone()
	}
	pub fn reset(&mut self) {
		self.engine.reset();
		self.last_suggestion.clear();
		self.in_autocomplete = false;
	}

	pub fn mark_autocomplete(&mut self, suggestion: &str) {
		self.last_suggestion = suggestion.to_string();
		self.in_autocomplete = true;
	}

	pub fn clear_autocomplete(&mut self) {
		self.in_autocomplete = false;
	}

	pub fn is_paused(&self, threshold_secs: u64) -> bool {
		self.last_input_time.elapsed().as_secs() >= threshold_secs
	}

	pub fn last_input_time(&self) -> Instant {
		self.last_input_time
	}
}
