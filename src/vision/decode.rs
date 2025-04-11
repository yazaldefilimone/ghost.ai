use super::DynamicImage;
use base64::{engine::general_purpose, Engine as _};
use std::io::Cursor;

pub fn image_to_base64(image: &DynamicImage) -> String {
	println!("[vision.decode] converting image to base64");
	let mut buffer = Cursor::new(Vec::new());

	if let Err(err) = image.write_to(&mut buffer, image::ImageFormat::Png) {
		eprintln!("[vision.decode] failed encoding image: {}", err);
		std::process::exit(1);
	}
	general_purpose::STANDARD.encode(buffer.get_ref())
}
