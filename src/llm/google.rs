use std::sync::Arc;

use futures::StreamExt;
use rig::agent::Agent;
use rig::completion::Prompt;
use rig::providers::gemini::{self};
use rig::streaming::{StreamingChoice, StreamingPrompt};
use sea_orm::DbConn;

use crate::database::Model;
use crate::llm::provider::LlmResult;
use crate::llm_error;
use crate::utils::format_model_as_context;

use super::prompt::{complete_prompt, question_prompt};

pub struct GoogleProvider {
	pub agent: Agent<gemini::completion::CompletionModel>,
}

impl GoogleProvider {
	pub async fn complete(&self, text: &str, db: Arc<DbConn>) -> LlmResult<String> {
		let context = format_model_as_context(&Model::get_all(&db).await);
		let prompt = complete_prompt(text, context);
		let response = self.agent.prompt(prompt).await?;
		Ok(response.trim().to_string())
	}

	pub async fn complete_stream<F>(
		&self,
		text: &str,
		db: Arc<DbConn>,
		mut handle_chunk: F,
	) -> LlmResult<()>
	where
		F: FnMut(String) + Send + 'static,
	{
		let context = format_model_as_context(&Model::get_all(&db).await);
		let prompt = complete_prompt(text, context);

		let mut stream = self.agent.stream_prompt(&prompt).await?;

		while let Some(chunk) = stream.next().await {
			match chunk {
				Ok(StreamingChoice::Message(text)) => handle_chunk(text),
				Ok(_) => {}
				Err(err) => {
					llm_error!("failed to stream completion: {err}");
					break;
				}
			}
		}

		Ok(())
	}

	pub async fn ask_question<F>(
		&self,
		text: &str,
		db: Arc<DbConn>,
		mut handle_chunk: F,
	) -> LlmResult<()>
	where
		F: FnMut(String) + Send + 'static,
	{
		let context = format_model_as_context(&Model::get_all(&db).await);
		let prompt = question_prompt(text, context);

		let mut stream = self.agent.stream_prompt(&prompt).await?;

		while let Some(chunk) = stream.next().await {
			match chunk {
				Ok(StreamingChoice::Message(text)) => handle_chunk(text),
				Ok(_) => {}
				Err(err) => {
					llm_error!("failed to respond to question: {err}");
					break;
				}
			}
		}

		Ok(())
	}
}
