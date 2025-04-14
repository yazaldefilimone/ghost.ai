use crate::{memory_store::MemoryStore, text_state::TextState};
use rdev::Keyboard;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

pub struct SharedState {
	pub memory: Arc<Mutex<MemoryStore>>,
	pub text_state: Arc<Mutex<TextState>>,
	pub keyboard: Arc<Mutex<Keyboard>>,
	pub muted: AtomicBool,
}

impl SharedState {
	pub fn new() -> Self {
		Self {
			memory: Arc::new(Mutex::new(MemoryStore::new())),
			text_state: Arc::new(Mutex::new(TextState::new())),
			keyboard: Arc::new(Mutex::new(Keyboard::new().unwrap())),
			muted: AtomicBool::new(false),
		}
	}

	pub fn mute(&self) {
		self.muted.store(true, Ordering::SeqCst);
	}

	pub fn unmute(&self) {
		self.muted.store(false, Ordering::SeqCst);
	}

	pub fn set_muted(&self, muted: bool) {
		self.muted.store(muted, Ordering::SeqCst);
	}

	pub fn is_muted(&self) -> bool {
		self.muted.load(Ordering::SeqCst)
	}
	pub fn with_muted<F: FnOnce()>(&self, f: F) {
		self.set_muted(true);
		f();
		self.set_muted(false);
	}
}
