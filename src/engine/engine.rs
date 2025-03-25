#[derive(Debug, Default)]
pub struct Engine {
	pub user_input: String,         // Texto real digitado
	pub suggestion: Option<String>, // Sugestão da IA aplicada
	pub cursor_pos: usize,          // Posição do cursor no texto
	pub in_suggestion: bool,        // Se está com sugestão ativa
}

impl Engine {
	pub fn new() -> Self {
		let user_input = String::new();
		let suggestion = None;
		let cursor_pos = 0;
		let in_suggestion = false;

		Self { user_input, suggestion, cursor_pos, in_suggestion }
	}

	pub fn push_char(&mut self, c: char) {
		if self.in_suggestion {
			self.cancel_suggestion();
		}
		self.user_input.insert(self.cursor_pos, c);
		self.cursor_pos += 1;
		// self.last_event_time = Instant::now();
	}

	pub fn backspace(&mut self) -> Option<String> {
		if self.cursor_pos == 0 {
			return None;
		}
		self.cursor_pos -= 1;
		self.user_input.remove(self.cursor_pos);
		// self.last_event_time = Instant::now();
		if self.in_suggestion {
			if let Some(sug) = &self.suggestion {
				if self.user_input.ends_with(sug) {
					return Some(sug.clone());
				} else {
					self.cancel_suggestion();
				}
			}
		}
		None
	}

	pub fn apply_suggestion(&mut self, text: &str) {
		let text = format!("  {text}");
		self.user_input.insert_str(self.cursor_pos, &text);
		self.cursor_pos += text.len();
		self.suggestion = Some(text.to_string());
		self.in_suggestion = true;
	}

	pub fn cancel_suggestion(&mut self) {
		if let Some(sug) = &self.suggestion {
			if self.cursor_pos >= sug.len() && self.user_input.ends_with(sug) {
				let start = self.cursor_pos - sug.len();
				self.user_input.replace_range(start..self.cursor_pos, "");
				self.cursor_pos = start;
			}
		}
		self.suggestion = None;
		self.in_suggestion = false;
	}

	pub fn reset(&mut self) {
		self.user_input.clear();
		self.cursor_pos = 0;
		self.in_suggestion = false;
		self.suggestion = None;
	}

	pub fn is_empty(&self) -> bool {
		self.user_input.is_empty()
	}

	pub fn get_text(&self) -> &str {
		&self.user_input
	}

	// pub fn is_paused(&self, secs: u64) -> bool {
	// self.last_event_time.elapsed().as_secs() >= secs
	// }

	pub fn has_suggestion(&self) -> bool {
		self.in_suggestion && self.suggestion.is_some()
	}

	pub fn get_suggestion(&self) -> Option<&str> {
		self.suggestion.as_deref()
	}
}
