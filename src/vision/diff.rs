use blake3;
use image::DynamicImage;

const TOLERANCE: f32 = 30.0; // 30%

pub fn hash_differs(left: &DynamicImage, right: &DynamicImage) -> bool {
	let left_hash = blake3::hash(left.to_rgba8().as_raw());
	let right_hash = blake3::hash(right.to_rgba8().as_raw());
	left_hash != right_hash
}

fn windows_similar(left: &DynamicImage, right: &DynamicImage) -> bool {
	println!("[vision.diff] try find similarity between windows");
	let left_image = left.to_rgba8();
	let right_image = right.to_rgba8();
	if left_image.dimensions() != right_image.dimensions() {
		return false;
	}

	let left_raw = left_image.as_raw();
	let right_raw = right_image.as_raw();

	let total = left_raw.len();
	let diffs = left_raw.iter().zip(right_raw).filter(|(a, b)| a != b).count();
	let percent_diff = diffs as f32 / total as f32;

	percent_diff <= TOLERANCE
}
