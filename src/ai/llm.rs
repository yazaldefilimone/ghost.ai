use rig::completion::Prompt;
use rig::providers::ollama;
use std::error::Error;

pub async fn prompt(prompt: &str) -> Result<String, Box<dyn Error>> {
	let agent = ollama::Client::new()
		.agent("llama3.2")
		.preamble("Be precise and concise.")
		.temperature(0.5)
		.build();
	let response = agent.prompt(prompt).await?;
	Ok(response)
}
pub async fn generate_suggestion(input: &str) -> Result<String, Box<dyn Error>> {
	let full_prompt = format!(
		r#"You are ghost.ia, a personal typing copilot. User is currently typing: "{}"
    Suggest the next part of this sentence, clearly and concisely:"#,
		input
	);

	let agent = ollama::Client::new()
		.agent("llama3")
		.preamble(
			"Your job is to extend user thoughts, clearly, concisely and naturally. Avoid repetition.",
		)
		.temperature(0.3)
		.build();
	let response = agent.prompt(full_prompt).await?;
	Ok(response.trim().to_string())
}
