use rig::{
	completion::{Prompt, PromptError},
	// message::{ContentFormat, Image, ImageMediaType},
	providers::ollama,
};

// use super::{decode, DynamicImage};

/*
NAME                           SIZE
mistral:7b-instruct            4.1 GB
llama3.2-vision:latest         7.9 GB
llava:latest                   4.7 GB
deepseek-coder:latest          776 MB
qwen2.5:3b                     1.9 GB
deepseek-r1:1.5b               1.1 GB
nomic-embed-text:latest        274 MB
gemma:2b                       1.7 GB
tinyllama:1.1b-chat            637 MB
llama3.2:latest                2.0 GB
llava-llama3:latest            5.5 GB
llama3.1:latest                4.7 GB
*/
const MODEL: &str = "mistral:7b-instruct"; // llama3.2:latest // llama3.1:latest
const TEMPERATURE: f64 = 0.5;
const PREAMBLE: &str = "
given the text below identifies message... normally comes with the name pattern(+ username)\nparagraphs(message) returns all messages
";

type ImageResult<T> = Result<T, PromptError>;

pub async fn summarize(image_info: &str) -> ImageResult<String> {
	println!("[vision.llm] infer from image");
	let client = ollama::Client::new();
	let agent = client.agent(MODEL).temperature(TEMPERATURE).preamble(PREAMBLE.trim()).build();
	let response = agent.prompt(image_info).await?;
	Ok(response)
}

// pub async fn summarize_image_context(image: String) -> ImageResult<String> {
// 	let client = ollama::Client::new();
// 	let agent = client.agent(MODEL).temperature(TEMPERATURE).preamble(PREAMBLE).build();
// 	let image = Image {
// 		data: image,
// 		media_type: Some(ImageMediaType::JPEG),
// 		format: Some(ContentFormat::Base64),
// 		..Default::default()
// 	};
// 	let response = agent.summarize(image).await?;
// 	Ok(response)
// }

// pub async fn detect_visual_entities(image: String) -> ImageResult<String> {
// 	let client = ollama::Client::new();
// 	let agent = client.agent(MODEL).temperature(TEMPERATURE).preamble(PREAMBLE).build();
// 	let image = Image {
// 		data: image,
// 		media_type: Some(ImageMediaType::JPEG),
// 		format: Some(ContentFormat::Base64),
// 		..Default::default()
// 	};
// 	let response = agent.detect(image).await?;
// 	Ok(response)
// }
