#[derive(Default)]
pub struct TextBuffer {
	content: String,
	cursor: usize,
}

#[allow(dead_code)]
impl TextBuffer {
	pub fn new() -> Self {
		Self { content: String::new(), cursor: 0 }
	}

	pub fn insert(&mut self, c: char) {
		self.content.insert(self.cursor, c);
		self.cursor += 1;
	}

	pub fn insert_str(&mut self, s: &str) {
		self.content.insert_str(self.cursor, s);
		self.cursor += s.len();
	}

	pub fn backspace(&mut self) {
		if self.cursor > 0 {
			self.cursor -= 1;
			self.content.remove(self.cursor);
		}
	}

	pub fn move_cursor_left(&mut self) {
		if self.cursor > 0 {
			self.cursor -= 1;
		}
	}

	pub fn move_cursor_right(&mut self) {
		if self.cursor < self.content.len() {
			self.cursor += 1;
		}
	}

	pub fn get_text(&self) -> String {
		let left = &self.content[..self.cursor];
		let right = &self.content[self.cursor..];
		format!("{}<cursor>{}", left, right)
	}

	pub fn debug(&self) {
		let left = &self.content[..self.cursor];
		let right = &self.content[self.cursor..];
		println!(">> {}<cursor>{}", left, right);
	}
}
