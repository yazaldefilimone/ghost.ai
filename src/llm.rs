use futures::StreamExt;
use rig::completion::Prompt;
use rig::providers::ollama::Client;
use rig::streaming::{StreamingChoice, StreamingPrompt};

/*
		NAME                           SIZE
		qwen2.5:3b                     1.9 GB
		deepseek-r1:1.5b               1.1 GB
		nomic-embed-text:latest        274 MB
		gemma:2b                       1.7 GB
		tinyllama:1.1b-chat            637 MB
		llama3.2:latest                2.0 GB
		llama3.1:latest                4.7 GB
*/
const MODEL: &str = "llama3.1:latest";

fn create_preamble(history: String) -> String {
	r#"
	You are an autocomplete assistant.
RULES:
- Only complete the user's sentence.
- NEVER repeat or rewrite what was already typed.
- Your entire output will be inserted directly after the last word.
- If it is unclear how to continue, output nothing.
"#
	.to_string()
}
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn generate_completion(prompt: &str, history: String) -> Result<String> {
	let client = Client::new();
	let agent = client.agent(MODEL).preamble(&create_preamble(history));
	let result = agent.build().prompt(prompt).await?;
	println!("[llm] result: {result}");
	Ok(result.trim().to_string())
}

pub async fn stream_completion<F>(prompt: &str, history: String, mut on_chunk: F) -> Result<()>
where
	F: FnMut(String) + Send + 'static,
{
	let client = Client::new();
	let agent = client.agent(MODEL).preamble(&create_preamble(history));

	// let prompt_cursor = format!("{prompt}");
	let mut stream = agent.build().stream_prompt(prompt).await?;
	while let Some(chunk_result) = stream.next().await {
		match chunk_result {
			Ok(StreamingChoice::Message(text)) => {
				on_chunk(text.to_owned());
			}
			Ok(_) => {}
			Err(reason) => {
				eprintln!("[llm] failed to stream completion, reason: '{}'", reason);
				break;
			}
		}
	}
	Ok(())
}
