use ocrs::{ImageSource, OcrEngine, OcrEngineParams};
use rten::Model;
use std::path::PathBuf;

fn file_path(path: &str) -> PathBuf {
	let mut abs_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
	abs_path.push(path);
	abs_path
}

const DETECTION_MODEL: &str = "models/text-detection.rten";
const RECOGNITION_MODEL: &str = "models/text-recognition.rten";
pub struct TextEngine {
	inner: OcrEngine,
}

impl TextEngine {
	pub fn new() -> Self {
		let detection_model = Model::load_file(file_path(DETECTION_MODEL)).unwrap();
		let recognition_model = Model::load_file(file_path(RECOGNITION_MODEL)).unwrap();
		let inner = OcrEngine::new(OcrEngineParams {
			detection_model: Some(detection_model),
			recognition_model: Some(recognition_model),
			..Default::default()
		})
		.unwrap();
		Self { inner }
	}

	pub fn recognize(&self, image: image::DynamicImage) -> Vec<String> {
		let rgb = image.into_rgb8();
		let img_source = ImageSource::from_bytes(rgb.as_raw(), rgb.dimensions()).unwrap();
		let ocr_input = self.inner.prepare_input(img_source).unwrap();
		// println!("[vision] detect words...");
		let word_rects = self.inner.detect_words(&ocr_input).unwrap();
		let line_rects = self.inner.find_text_lines(&ocr_input, &word_rects);
		// println!("[vision] recognize text...");
		let line_texts = self.inner.recognize_text(&ocr_input, &line_rects).unwrap();
		let filtered_line_texts = line_texts.into_iter().flatten().filter(|l| l.to_string().len() > 1);
		filtered_line_texts.map(|l| l.to_string()).collect()
	}
}

impl Default for TextEngine {
	fn default() -> Self {
		Self::new()
	}
}
