use rig::providers;

use crate::{llm_error, settings::provider::LlmModel};

use super::{google::GoogleProvider, provider::Provider};
use crate::llm::{
	anthropic::AnthropicProvider, groq::GroqProvider, ollama::OllamaProvider, openai::OpenAiProvider,
	together::TogetherProvider,
};

pub fn build_provider(
	model: &LlmModel,
	temperature: Option<f64>,
	max_tokens: Option<u64>,
) -> Provider {
	match model {
		LlmModel::Anthropic { model, api_key } => {
			let api_key = api_key.as_ref().unwrap_or_else(|| {
				llm_error!("Anthropic API key not found");
				std::process::exit(1);
			});
			let client = providers::anthropic::ClientBuilder::new(api_key).build();
			let mut agent = client.agent(model.as_str());
			if let Some(temperature) = temperature {
				agent = agent.temperature(temperature)
			};
			if let Some(max_tokens) = max_tokens {
				agent = agent.max_tokens(max_tokens);
			};
			let agent = agent.build();
			Provider::Anthropic(AnthropicProvider { agent })
		}
		LlmModel::Google { model, api_key } => {
			let api_key = api_key.as_ref().unwrap_or_else(|| {
				llm_error!("Google API key not found");
				std::process::exit(1);
			});
			let client = providers::gemini::Client::new(api_key);
			let mut agent = client.agent(model.as_str());
			if let Some(temperature) = temperature {
				agent = agent.temperature(temperature)
			};
			if let Some(max_tokens) = max_tokens {
				agent = agent.max_tokens(max_tokens);
			};
			let agent = agent.build();
			Provider::Google(GoogleProvider { agent })
		}
		LlmModel::OpenAI { model, api_key } => {
			let api_key = api_key.as_ref().unwrap_or_else(|| {
				llm_error!("OpenAI API key not found");
				std::process::exit(1);
			});
			let client = providers::openai::Client::new(api_key);
			let mut agent = client.agent(model.as_str());
			if let Some(temperature) = temperature {
				agent = agent.temperature(temperature)
			};
			if let Some(max_tokens) = max_tokens {
				agent = agent.max_tokens(max_tokens);
			};
			let agent = agent.build();
			Provider::OpenAI(OpenAiProvider { agent })
		}
		LlmModel::Groq { model, api_key } => {
			let api_key = api_key.as_ref().unwrap_or_else(|| {
				llm_error!("Groq API key not found");
				std::process::exit(1);
			});
			let client = providers::groq::Client::new(api_key);
			let mut agent = client.agent(model.as_str());
			if let Some(temperature) = temperature {
				agent = agent.temperature(temperature);
			};
			if let Some(max_tokens) = max_tokens {
				agent = agent.max_tokens(max_tokens);
			};
			let agent = agent.build();
			Provider::Groq(GroqProvider { agent })
		}
		LlmModel::Ollama { model } => {
			let client = providers::ollama::Client::new();
			let mut agent = client.agent(model.0.as_str());
			if let Some(temperature) = temperature {
				agent = agent.temperature(temperature);
			};
			if let Some(max_tokens) = max_tokens {
				agent = agent.max_tokens(max_tokens);
			};
			let agent = agent.build();
			Provider::Ollama(OllamaProvider { agent })
		}
		LlmModel::Together { model, api_key } => {
			let api_key = api_key.as_ref().unwrap_or_else(|| {
				llm_error!("Together API key not found");
				std::process::exit(1);
			});
			let client = providers::together::Client::new(api_key);
			let mut agent = client.agent(model.as_str());
			if let Some(temperature) = temperature {
				agent = agent.temperature(temperature);
			};
			if let Some(max_tokens) = max_tokens {
				agent = agent.max_tokens(max_tokens);
			};
			let agent = agent.build();
			Provider::Together(TogetherProvider { agent })
		}
	}
}
