use std::sync::Arc;

use futures::StreamExt;
use rig::completion::Prompt;
use rig::providers::ollama::Client;
use rig::streaming::{stream_to_stdout, StreamingChoice, StreamingPrompt};
use tokio::sync::RwLock;

use crate::memory::store::Store;

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
const MODEL: &str = "mistral:7b-instruct";

fn ask_preamble(prompt: &str) -> String {
	r#"
	You are a highly intelligent and helpful assistant acting as the user's second brain.

<context>
{history}
</context>

Your goal is to help the user recall anything they’ve seen or read on their screen. The <context> contains raw OCR-extracted text from screenshots of windows the user interacted with.

Guidelines:
- Assume the user is trying to remember something they recently saw.
- The extracted text may be noisy or contain UI elements (e.g. tooltips, icons, buttons); focus on meaningful content such as paragraphs, labels, conversations, or documents.
- Ignore elements that seem like part of the UI unless they look important or unique.
- If the user asks a question, try to match relevant parts of the context and respond naturally.
- Do not repeat or reformat the context unless necessary for clarity.
- You are not just a summarizer — you are a memory engine.

Always respond based on what’s present in the <context>. If unsure, say so clearly.
"#
	.to_string()
}

fn create_preamble(history: String) -> String {
	format!(
		r#"
You are a highly intelligent and helpful assistant acting as the user's second brain.

<context>
{history}
</context>

Your goal is to help the user recall anything they’ve seen or read on their screen. The <context> contains raw OCR-extracted text from screenshots of windows the user interacted with.

Guidelines:
- Assume the user is trying to remember something they recently saw.
- The extracted text may be noisy or contain UI elements (e.g. tooltips, icons, buttons); focus on meaningful content such as paragraphs, labels, conversations, or documents.
- Ignore elements that seem like part of the UI unless they look important or unique.
- If the user asks a question, try to match relevant parts of the context and respond naturally.
- Do not repeat or reformat the context unless necessary for clarity.
- You are not just a summarizer — you are a memory engine.

Always respond based on what’s present in the <context>. If unsure, say so clearly.
"#
	)
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn generate_completion(prompt: &str, store: Arc<RwLock<Store>>) -> Result<String> {
	let client = Client::new();
	let store = store.read().await;
	let agent = client.agent(MODEL).preamble(&create_preamble(store.format_entries_as_context()));
	let result = agent.build().prompt(prompt).await?;
	// println!("[llm] result: {result}");
	Ok(result.trim().to_string())
}

pub async fn stream_completion<F>(
	prompt: &str,
	store: Arc<RwLock<Store>>,
	mut on_chunk: F,
) -> Result<()>
where
	F: FnMut(String) + Send + 'static,
{
	let client = Client::new();
	let store = store.read().await;
	let entries_text = store.format_entries_as_context();
	// println!("[llm] entries_text: {entries_text}");
	let agent = client.agent(MODEL).preamble(&create_preamble(entries_text));

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

pub async fn ask_stream(prompt: &str, store: Arc<RwLock<Store>>) -> Result<()> {
	let store = store.read().await;
	let entries_text = store.format_entries_as_context();
	let client = Client::new();
	let agent = client.agent(MODEL).preamble(&create_preamble(entries_text)).temperature(0.5).build();
	// let prompt_cursor = format!("{prompt}");
	let mut stream = agent.stream_prompt(prompt).await?;
	stream_to_stdout(agent, &mut stream).await?;
	Ok(())
}

use std::io::{self, Write};

pub async fn chat_loop(store: Arc<RwLock<Store>>) -> Result<()> {
	let stdin = io::stdin();
	let mut stdout = io::stdout();
	let mut input = String::new();

	loop {
		print!(">>> ");
		stdout.flush()?;
		input.clear();
		stdin.read_line(&mut input)?;
		let prompt = input.trim();

		if prompt == "exit" || prompt == "quit" {
			break;
		}

		// print!("ghost: ");
		stdout.flush()?;
		ask_stream(prompt, store.clone()).await?;
		println!();
	}

	Ok(())
}
