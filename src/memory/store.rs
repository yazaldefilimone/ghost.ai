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

	pub fn add(&mut self, entry: Entry) -> usize {
		let len = self.entries.len();
		self.entries.push(entry);
		len
	}

	pub fn add_many(&mut self, entries: Vec<Entry>) {
		self.entries.extend(entries);
	}

	pub fn add_embedding(&mut self, id: usize, embedding: Vec<f32>) {
		if let Some(entry) = self.entries.get_mut(id) {
			entry.embedding = Some(embedding);
		}
	}

	pub fn remove(&mut self, index: usize) {
		self.entries.remove(index);
	}

	pub fn get(&self, index: usize) -> Option<&Entry> {
		self.entries.get(index)
	}

	pub fn all(&self) -> &[Entry] {
		&self.entries
	}

	pub fn get_first(&self) -> Option<&Entry> {
		self.entries.first()
	}

	pub fn find_similar(&self, entry: &Entry, threshold: f32) -> Vec<&Entry> {
		self
			.entries
			.iter()
			.filter(|e| cosine_similarity(e.embedding(), entry.embedding()) > threshold)
			.collect()
	}

	pub fn format_entries_as_context(&self) -> String {
		self
			.entries
			.iter()
			.map(|entry| {
				let app = entry.app_name.as_deref().unwrap_or("Unknown");
				let window = entry.window_title.as_deref().unwrap_or("Unknown");
				let timestamp = entry.timestamp.to_rfc3339();

				format!(
					"<context>\nApp: {}\nWindow: {}\nTime: {}\n\n{}\n</context>",
					app,
					window,
					timestamp,
					entry.extracted_text.trim()
				)
			})
			.collect::<Vec<_>>()
			.join("\n\n")
	}
}
