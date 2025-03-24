use std::time::Instant;

#[derive(Debug)]
pub struct Engine {
	pub user_input: String,
	pub suggestion: Option<String>,
	pub cursor_pos: usize,
	pub in_suggestion: bool,
	pub last_event_time: Instant,
}

impl Engine {
	pub fn new() -> Self {
		Self {
			user_input: String::new(),
			suggestion: None,
			cursor_pos: 0,
			in_suggestion: false,
			last_event_time: Instant::now(),
		}
	}

	pub fn push_char(&mut self, c: char) {
		self.user_input.insert(self.cursor_pos, c);
		self.cursor_pos += 1;
		self.last_event_time = Instant::now();
	}

	pub fn backspace(&mut self) {
		if self.cursor_pos > 0 {
			self.cursor_pos -= 1;
			self.user_input.remove(self.cursor_pos);
		}

		// Se apagou algo da sugestão
		if self.in_suggestion {
			if let Some(sug) = &self.suggestion {
				if !self.user_input.ends_with(sug) {
					// Sugestão foi modificada ou rejeitada
					self.suggestion = None;
					self.in_suggestion = false;
				}
			}
		}

		self.last_event_time = Instant::now();
	}

	pub fn apply_suggestion(&mut self, text: &str) {
		self.user_input.insert_str(self.cursor_pos, text);
		self.suggestion = Some(text.to_string());
		self.in_suggestion = true;
		self.cursor_pos += text.len();
	}

	pub fn reset(&mut self) {
		self.user_input.clear();
		self.cursor_pos = 0;
		self.in_suggestion = false;
		self.suggestion = None;
	}
}
