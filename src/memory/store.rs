use super::entry::Entry;
use crate::embed::cosine_similarity;

/// In-memory visual/text memory storage.
#[derive(Debug, Clone)]
pub struct Store {
	entries: Vec<Entry>,
}

impl Store {
	pub fn new() -> Self {
		Self { entries: Vec::new() }
	}

	pub fn add(&mut self, entry: Entry) {
		self.entries.push(entry);
	}

	pub fn all(&self) -> &[Entry] {
		&self.entries
	}

	pub fn find_similar(&self, entry: &Entry, threshold: f32) -> Vec<&Entry> {
		self
			.entries
			.iter()
			.filter(|e| cosine_similarity(e.embedding(), entry.embedding()) > threshold)
			.collect()
	}
}
