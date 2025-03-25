#[derive(Debug, Default, Clone)]
pub struct KeyboardState {
	pub shift: bool,
	pub capslock: bool,
	pub alt: bool,
	pub ctrl: bool,
}

impl KeyboardState {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn update(&mut self, key: &rdev::Key, pressed: bool) {
		match key {
			rdev::Key::ShiftLeft | rdev::Key::ShiftRight => self.shift = pressed,
			rdev::Key::ControlLeft | rdev::Key::ControlRight => self.ctrl = pressed,
			rdev::Key::Alt | rdev::Key::AltGr => self.alt = pressed,
			rdev::Key::CapsLock if pressed => {
				self.capslock = !self.capslock;
			}
			_ => {}
		}
	}

	pub fn is_uppercase(&self) -> bool {
		self.shift ^ self.capslock
	}

	pub fn reset(&mut self) {
		self.shift = false;
		self.capslock = false;
		self.alt = false;
		self.ctrl = false;
	}
}
