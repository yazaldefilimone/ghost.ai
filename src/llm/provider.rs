pub type LlmResult<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

use std::sync::Arc;

use sea_orm::DbConn;

use super::{anthropic, google, groq, ollama, openai, together};

pub enum Provider {
	Anthropic(anthropic::AnthropicProvider),
	Google(google::GoogleProvider),
	OpenAI(openai::OpenAiProvider),
	Groq(groq::GroqProvider),
	Ollama(ollama::OllamaProvider),
	Together(together::TogetherProvider),
}

impl Provider {
	pub async fn complete(&self, text: &str, db: Arc<DbConn>) -> LlmResult<String> {
		match self {
			Provider::Anthropic(provider) => provider.complete(text, db).await,
			Provider::Google(provider) => provider.complete(text, db).await,
			Provider::OpenAI(provider) => provider.complete(text, db).await,
			Provider::Groq(provider) => provider.complete(text, db).await,
			Provider::Ollama(provider) => provider.complete(text, db).await,
			Provider::Together(provider) => provider.complete(text, db).await,
		}
	}

	pub async fn complete_stream<F>(
		&self,
		text: &str,
		db: Arc<DbConn>,
		handle_chunk: F,
	) -> LlmResult<()>
	where
		F: FnMut(String) + Send + 'static,
	{
		match self {
			Provider::Anthropic(provider) => provider.complete_stream(text, db, handle_chunk).await,
			Provider::Google(provider) => provider.complete_stream(text, db, handle_chunk).await,
			Provider::OpenAI(provider) => provider.complete_stream(text, db, handle_chunk).await,
			Provider::Groq(provider) => provider.complete_stream(text, db, handle_chunk).await,
			Provider::Ollama(provider) => provider.complete_stream(text, db, handle_chunk).await,
			Provider::Together(provider) => provider.complete_stream(text, db, handle_chunk).await,
		}
	}

	#[allow(dead_code)]
	pub async fn ask_question<F>(&self, text: &str, db: Arc<DbConn>, handle_chunk: F) -> LlmResult<()>
	where
		F: FnMut(String) + Send + 'static,
	{
		match self {
			Provider::Anthropic(provider) => provider.ask_question(text, db, handle_chunk).await,
			Provider::Google(provider) => provider.ask_question(text, db, handle_chunk).await,
			Provider::OpenAI(provider) => provider.ask_question(text, db, handle_chunk).await,
			Provider::Groq(provider) => provider.ask_question(text, db, handle_chunk).await,
			Provider::Ollama(provider) => provider.ask_question(text, db, handle_chunk).await,
			Provider::Together(provider) => provider.ask_question(text, db, handle_chunk).await,
		}
	}
}
