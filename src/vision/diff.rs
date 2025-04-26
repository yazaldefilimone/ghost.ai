#![allow(dead_code)]
use blake3;
use image::DynamicImage;

use crate::constants::VISION_DIFF_TOLERANCE;

/// compares two images using hashing (exact content check).
pub fn frames_hash_differs(left: &DynamicImage, right: &DynamicImage) -> bool {
	let left_hash = blake3::hash(left.to_rgba8().as_raw());
	let right_hash = blake3::hash(right.to_rgba8().as_raw());
	left_hash != right_hash
}

/// compares two images pixel-by-pixel and returns true if they're visually similar.
pub fn frames_visually_similar(left: &DynamicImage, right: &DynamicImage) -> bool {
	let left_image = left.to_rgba8();
	let right_image = right.to_rgba8();

	if left_image.dimensions() != right_image.dimensions() {
		return false;
	}

	let left_raw = left_image.as_raw();
	let right_raw = right_image.as_raw();

	let total_pixels = left_raw.len();
	let diff_pixels = left_raw.iter().zip(right_raw).filter(|(a, b)| a != b).count();

	let diff_ratio = diff_pixels as f32 / total_pixels as f32;
	// crate::vision_info!("diff ratio: {:.2}%", diff_ratio * 100.0);
	diff_ratio <= VISION_DIFF_TOLERANCE
}
