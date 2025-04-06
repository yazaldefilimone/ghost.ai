use std::cmp;

use crate::embed;

#[derive(Clone)]
pub struct MemoryEntry {
	pub content: String,
	pub embedding: Vec<f32>,
	// pub timestamp: u64,
}

pub struct MemoryStore {
	pub entries: Vec<MemoryEntry>,
}

impl MemoryStore {
	pub fn new() -> Self {
		Self { entries: Vec::new() }
	}

	pub fn insert(&mut self, content: String, embedding: Vec<f32>) {
		self.entries.push(MemoryEntry { content, embedding });
	}

	pub fn insert_entry(&mut self, entry: MemoryEntry) {
		self.entries.push(entry);
	}

	pub fn insert_content(&mut self, content: String) {
		self.entries.push(MemoryEntry { content, embedding: Vec::new() });
	}

	/// find the top-k most similar entries to the given embedding.
	pub fn find_similar(&self, query: &[f32], k: usize) -> Vec<String> {
		let scored_iter = self.entries.iter();

		let scored_map = scored_iter.map(|entry| {
			let score = embed::cosine_similarity(query, &entry.embedding);
			(score, entry.content.clone())
		});
		let mut scored: Vec<_> = scored_map.collect();

		scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(cmp::Ordering::Equal));

		scored.into_iter().take(k).map(|(_, c)| c).collect()
	}
}

impl std::fmt::Display for MemoryEntry {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.content)
	}
}

impl std::fmt::Display for MemoryStore {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.entries.iter().map(|e| e.to_string()).collect::<Vec<_>>().join("\n-  "))
	}
}
