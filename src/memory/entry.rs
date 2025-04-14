use std::fmt::Display;

use crate::vision::FrameCaptured;
use chrono::{DateTime, Utc};
use image::DynamicImage;

#[derive(Debug, Clone)]
pub struct Entry {
	pub timestamp: DateTime<Utc>,
	pub window_title: Option<String>,
	pub app_name: Option<String>,
	pub extracted_text: String,
	pub embedding: Option<Vec<f32>>,
	pub frame: FrameCaptured, // only kept in-memory
}

impl Entry {
	pub fn new(frame: FrameCaptured) -> Self {
		Self {
			timestamp: Utc::now(),
			window_title: None,
			app_name: None,
			extracted_text: String::new(),
			embedding: None,
			frame,
		}
	}

	pub fn with_window_title(mut self, title: impl Into<String>) -> Self {
		self.window_title = Some(title.into());
		self
	}

	pub fn with_app_name(mut self, name: impl Into<String>) -> Self {
		self.app_name = Some(name.into());
		self
	}

	pub fn with_text(mut self, text: impl Into<String>) -> Self {
		self.extracted_text = text.into();
		self
	}

	pub fn with_vector_text(mut self, text: Vec<String>) -> Self {
		self.extracted_text = text.join("\n");
		self
	}
	pub fn with_embedding(mut self, embedding: Vec<f32>) -> Self {
		self.embedding = Some(embedding);
		self
	}

	pub fn embedding(&self) -> &[f32] {
		self.embedding.as_deref().unwrap_or_default()
	}

	pub fn dynamic_image(&self) -> DynamicImage {
		self.frame.clone().into()
	}
}

impl Display for Entry {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		// app_name and window_title
		writeln!(f, "app_name: {}", self.app_name.as_deref().unwrap_or("?"))?;
		writeln!(f, "window_title: {}", self.window_title.as_deref().unwrap_or("?"))?;
		Ok(())
	}
}
