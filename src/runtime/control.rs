use rdev::Key;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

pub struct Control {
	pub is_muted: AtomicBool,
	pub simulated_keys: Mutex<VecDeque<Key>>,
}

impl Control {
	pub fn new() -> Self {
		Self {
			is_muted: AtomicBool::new(false),
			simulated_keys: Mutex::new(VecDeque::with_capacity(10)),
		}
	}

	pub fn mute(&self) {
		self.is_muted.store(true, Ordering::SeqCst);
	}

	pub fn unmute(&self) {
		self.is_muted.store(false, Ordering::SeqCst);
	}

	pub fn is_muted(&self) -> bool {
		self.is_muted.load(Ordering::SeqCst)
	}

	pub fn push_simulated(&self, key: Key) {
		let mut sim = self.simulated_keys.lock().unwrap();
		if sim.len() >= 10 {
			sim.pop_front();
		}
		sim.push_back(key);
	}

	pub fn was_simulated(&self, key: &Key) -> bool {
		let sim = self.simulated_keys.lock().unwrap();
		sim.contains(key)
	}

	pub fn clear(&self) {
		self.simulated_keys.lock().unwrap().clear();
	}
}
