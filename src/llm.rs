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
const MODEL: &str = "qwen2.5:3b";
const PREAMBLE: &str = r#"
You are an intelligent text completion assistant.

Your task is to help users complete their sentences in a natural, concise, and helpful way.

Specifications:
- Continue the sentence **exactly from the last word** typed by the user.
- **Never modify, correct, or repeat** the input — only append.
- Use the **same language** as the user's input.
- Adapt to the user's writing style and intent.
- Do not include punctuation unless it fits naturally.
- Your output will be **automatically injected into the input field**, so your reply must be directly usable.
- If unsure, give a **neutral and helpful continuation**.

Important:
- Your response must be **only the continuation string**. No formatting, explanations, or additional context.
- This is not a conversation — it’s real-time typing assistance.

Examples:
User: I’m going to talk to João about the
Assistant: meeting tomorrow.

User: Hoje quero escrever um post sobre o que aconteceu com
Assistant: a Ana na reunião.

User: Let me quickly finish this and then I’ll
Assistant: go grab lunch.
"#;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn generate_completion(prompt: &str) -> Result<String> {
	let client = Client::new();
	let agent = client.agent(MODEL).preamble(PREAMBLE);
	let result = agent.build().prompt(prompt).await?;
	println!("[llm] result: {result}");
	Ok(result.trim().to_string())
}

pub async fn stream_completion<F>(prompt: &str, mut on_chunk: F) -> Result<()>
where
	F: FnMut(String) + Send + 'static,
{
	let client = Client::new();
	let agent = client.agent(MODEL).preamble(PREAMBLE);

	let prompt_cursor = format!("{prompt}<cursor>");
	let mut stream = agent.build().stream_prompt(prompt_cursor.as_str()).await?;
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
