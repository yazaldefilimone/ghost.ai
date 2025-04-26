use crate::vision_error;

use super::FrameCaptured;
use base64::{engine::general_purpose, Engine as _};
use image::ImageFormat;
use std::io::Cursor;

#[allow(dead_code)]
pub fn frame_to_base64(frame: &FrameCaptured) -> String {
	let mut buffer = Cursor::new(Vec::new());
	if let Err(err) = frame.write_to(&mut buffer, ImageFormat::Png) {
		vision_error!("can't encode frame, reason: {err}");
		std::process::exit(1);
	}
	general_purpose::STANDARD.encode(buffer.get_ref())
}
