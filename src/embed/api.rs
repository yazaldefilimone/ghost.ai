use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct EmbeddingRequest<'a> {
	model: &'a str,
	prompt: &'a str,
}
impl<'a> EmbeddingRequest<'a> {
	fn new(model: &'a str, prompt: &'a str) -> Self {
		Self { model, prompt }
	}
}

#[derive(Deserialize)]
struct EmbeddingResponse {
	embedding: Vec<f32>,
}

impl EmbeddingResponse {
	fn new(embedding: Vec<f32>) -> Self {
		Self { embedding }
	}
}

type Result<T> = std::result::Result<T, reqwest::Error>;

const EMBEDDING_URL: &str = "http://localhost:11434/api/embeddings";

pub async fn generate_embedding(text: &str) -> Result<Vec<f32>> {
	let client = reqwest::Client::new();

	let embed_model = EmbeddingRequest::new("nomic-embed-text", text);

	let response = client.post(EMBEDDING_URL).json(&embed_model).send().await?;

	let json: EmbeddingResponse = response.json().await?;

	Ok(json.embedding)
}
