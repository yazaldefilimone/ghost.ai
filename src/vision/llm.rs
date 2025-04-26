#![allow(unused_imports, dead_code)]
use rig::completion::{Prompt, PromptError};
use rig::providers::ollama;

use super::decode::frame_to_base64;
use super::FrameCaptured;

const MODEL: &str = "mistral:7b-instruct"; // change to better model for image
const TEMPERATURE: f64 = 0.75;

const PREAMBLE: &str = "";

type LlmResult<T> = Result<T, PromptError>;

// todo: support settings models
pub async fn describe_visible_text(_frame: &FrameCaptured) -> LlmResult<String> {
	todo!()
	// let client = ollama::Client::new();
	// let agent = client.agent(MODEL).temperature(TEMPERATURE).preamble(PREAMBLE.trim()).build();
	// let base64 = frame_to_base64(frame);
	// let response = agent.prompt(base64).await?;
	// Ok(response)
}
