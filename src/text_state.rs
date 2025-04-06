#[derive(Debug, Clone, Default)]
pub struct TextState {
	typed: String,
	suggestion: String,
	// suggestions: Vec<String>,
	// suggestion_index: usize,
	// cursor: usize, // todo
}

impl TextState {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn add_character(&mut self, character: char) {
		self.typed.push(character);
		self.clear_suggestion();
	}
	pub fn add_text(&mut self, text: &str) {
		self.typed.push_str(text);
		self.clear_suggestion();
	}
	pub fn add_suggestion(&mut self, suggestion: String) {
		self.suggestion = suggestion;
	}

	pub fn backspace(&mut self) -> usize {
		if self.has_suggestion() {
			self.clear_suggestion()
		} else {
			self.typed.pop();
			1
		}
	}

	pub fn apply_suggestion(&self) -> String {
		format!("{}{}", self.typed, self.suggestion)
	}

	pub fn get_typed(&self) -> &str {
		&self.typed
	}

	pub fn get_history(&self) -> &str {
		&self.typed
	}
	pub fn apply_typed(&mut self) -> String {
		let typed = self.apply_suggestion();
		self.clear();
		typed
	}

	pub fn current_suggestion(&self) -> Option<&str> {
		if self.has_suggestion() {
			Some(&self.suggestion)
		} else {
			None
		}
	}

	// pub fn set_suggestions(&mut self, new: Vec<String>) {
	// 	self.suggestions = new;
	// 	self.suggestion_index = 0;

	// 	if let Some(first) = self.suggestions.first() {
	// 		self.suggestion = first.clone();
	// 	} else {
	// 		self.suggestion.clear();
	// 	}
	// }

	// pub fn next_suggestion(&mut self) -> Option<&str> {
	// 	if self.suggestions.is_empty() {
	// 		return None;
	// 	}
	// 	self.suggestion_index = (self.suggestion_index + 1) % self.suggestions.len();
	// 	self.suggestion = self.suggestions[self.suggestion_index].clone();
	// 	Some(&self.suggestion)
	// }

	pub fn clear(&mut self) {
		self.typed.clear();
		let _ = self.clear_suggestion();
		// self.suggestions.clear();
		// self.suggestion_index = 0;
	}

	pub fn has_suggestion(&self) -> bool {
		!self.suggestion.is_empty()
	}

	fn clear_suggestion(&mut self) -> usize {
		let len = self.suggestion.len();
		self.suggestion.clear();
		len
	}

	// pub fn cursor_left(&mut self) {
	// 	if self.cursor > 0 {
	// 		self.cursor = self.cursor.saturating_sub(1)
	// 	}
	// }
	// pub fn cursor_right(&mut self) {
	// 	if self.cursor < self.typed.len() {
	// 		self.cursor = self.cursor.saturating_add(1)
	// 	}
	// }
}
