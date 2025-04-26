pub mod api;

#[allow(dead_code)]
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
	let dot = a.iter().zip(b).map(|(x, y)| x * y).sum::<f32>();
	let norm_a = a.iter().map(|x| x * x).sum::<f32>().sqrt();
	let norm_b = b.iter().map(|x| x * x).sum::<f32>().sqrt();
	dot / (norm_a * norm_b)
}
