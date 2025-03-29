#[derive(Debug, Clone)]
pub struct Context {
	pub current_input: String,
	pub recent_context: Vec<String>,
	pub max_words: usize,

	pub max_chars: usize,
}

impl Context {
	pub fn new() -> Self {
		Self { current_input: "".into(), recent_context: Vec::new(), max_words: 10, max_chars: 100 }
	}

	pub fn context_block(&self) -> String {
		if self.recent_context.is_empty() {
			String::new()
		} else {
			let mut block = String::from("Histórico recente:\n");
			for line in &self.recent_context {
				block.push_str("- ");
				block.push_str(line.trim());
				block.push('\n');
			}
			block.trim().to_string()
		}
	}

	pub fn clean_input(&self) -> String {
		format!("{} <CURSOR>", self.current_input.trim())
	}

	pub fn is_valid_suggestion(&self, suggestion: &str) -> bool {
		let word_count = suggestion.split_whitespace().count();
		let char_count = suggestion.chars().count();

		word_count <= self.max_words && char_count <= self.max_chars
	}

	pub fn add_context_line(&mut self, line: String) {
		self.recent_context.push(line);
	}

	pub fn replace_input(&mut self, new_input: String) {
		if !self.current_input.is_empty() {
			self.add_context_line(self.current_input.clone());
		}
		self.current_input = new_input;
	}
}
