use std::path::PathBuf;

use image::DynamicImage;
use ocrs::{ImageSource, OcrEngine, OcrEngineParams};
use rten::Model;

use crate::{
	constants::{DETECTION_MODEL, RECOGNITION_MODEL},
	vision_info,
};

pub struct TextEngine {
	inner: OcrEngine,
}

impl TextEngine {
	pub fn new() -> Self {
		let detection = Model::load_file(Self::model_path(DETECTION_MODEL)).unwrap();
		let recognition = Model::load_file(Self::model_path(RECOGNITION_MODEL)).unwrap();
		let engine = OcrEngine::new(OcrEngineParams {
			detection_model: Some(detection),
			recognition_model: Some(recognition),
			..Default::default()
		})
		.unwrap();

		Self { inner: engine }
	}

	pub fn recognize(&self, image: DynamicImage) -> Vec<String> {
		let rgb = image.into_rgb8();
		let source = ImageSource::from_bytes(rgb.as_raw(), rgb.dimensions()).unwrap();
		let input = self.inner.prepare_input(source).unwrap();

		vision_info!("detecting words...");
		let words = self.inner.detect_words(&input).unwrap();

		vision_info!("grouping lines...");
		let lines = self.inner.find_text_lines(&input, &words);

		vision_info!("recognizing text...");
		let result = self.inner.recognize_text(&input, &lines).unwrap();
		result
			.into_iter()
			.flatten()
			.filter(|line| line.to_string().len() > 1)
			.map(|line| line.to_string())
			.collect()
	}

	fn model_path(file: &str) -> PathBuf {
		let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
		path.push(file);
		path
	}
}

impl Default for TextEngine {
	fn default() -> Self {
		Self::new()
	}
}
