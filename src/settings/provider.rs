use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "provider", rename_all = "lowercase")]
pub enum LlmModel {
	#[serde(rename = "openai")]
	OpenAI {
		#[serde(skip_serializing_if = "Option::is_none")]
		api_key: Option<String>,
		model: OpenAIModel,
	},

	#[serde(rename = "ollama")]
	Ollama { model: OllamaModel },

	#[serde(rename = "anthropic")]
	Anthropic {
		#[serde(skip_serializing_if = "Option::is_none")]
		api_key: Option<String>,
		model: AnthropicModel,
	},
	#[serde(rename = "google")]
	Google {
		#[serde(skip_serializing_if = "Option::is_none")]
		api_key: Option<String>,
		model: GoogleModel,
	},
	#[serde(rename = "groq")]
	Groq {
		#[serde(skip_serializing_if = "Option::is_none")]
		api_key: Option<String>,
		model: GroqModel,
	},

	#[serde(rename = "together")]
	Together {
		#[serde(skip_serializing_if = "Option::is_none")]
		api_key: Option<String>,
		model: TogetherModel,
	},
}

impl Default for LlmModel {
	fn default() -> Self {
		Self::Ollama { model: OllamaModel::default() }
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OpenAIModel {
	#[serde(rename = "o1")]
	O1,
	#[serde(rename = "o1-preview")]
	O1Preview,
	#[serde(rename = "o1-mini")]
	O1Mini,
	#[serde(rename = "gpt-4o")]
	Gpt4o,
	#[serde(rename = "gpt-4o-mini")]
	Gpt4oMini,
	#[serde(rename = "gpt-4o-realtime-preview")]
	Gpt4oRealtimePreview,
	#[serde(rename = "gpt-4-turbo")]
	Gpt4Turbo,
	#[serde(rename = "gpt-4")]
	Gpt4,
	#[serde(rename = "gpt-4-32k")]
	Gpt432k,
	#[serde(rename = "gpt-4-32k-0613")]
	Gpt432k0613,
	#[serde(rename = "gpt-3.5-turbo")]
	Gpt35Turbo,
	#[serde(rename = "gpt-3.5-turbo-instruct")]
	Gpt35TurboInstruct,
	#[serde(rename = "gpt-3.5-turbo-16k")]
	Gpt35Turbo16k,
}

impl Default for OpenAIModel {
	fn default() -> Self {
		Self::Gpt4oMini
	}
}

impl OpenAIModel {
	pub fn as_str(&self) -> &'static str {
		match self {
			Self::O1 => "o1",
			Self::O1Preview => "o1-preview",
			Self::O1Mini => "o1-mini",
			Self::Gpt4o => "gpt-4o",
			Self::Gpt4oMini => "gpt-4o-mini",
			Self::Gpt4oRealtimePreview => "gpt-4o-realtime-preview",
			Self::Gpt4Turbo => "gpt-4-turbo",
			Self::Gpt4 => "gpt-4",
			Self::Gpt432k => "gpt-4-32k",
			Self::Gpt432k0613 => "gpt-4-32k-0613",
			Self::Gpt35Turbo => "gpt-3.5-turbo",
			Self::Gpt35TurboInstruct => "gpt-3.5-turbo-instruct",
			Self::Gpt35Turbo16k => "gpt-3.5-turbo-16k",
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AnthropicModel {
	#[serde(rename = "claude-3-7-sonnet-latest")]
	Claude37Sonnet,
	#[serde(rename = "claude-3-5-sonnet-latest")]
	Claude35Sonnet,
	#[serde(rename = "claude-3-5-haiku-latest")]
	Claude35Haiku,
	#[serde(rename = "claude-3-opus-latest")]
	Claude3Opus,
	#[serde(rename = "claude-3-sonnet-20240229")]
	Claude3Sonnet,
	#[serde(rename = "claude-3-haiku-20240307")]
	Claude3Haiku,
}

impl Default for AnthropicModel {
	fn default() -> Self {
		Self::Claude35Haiku
	}
}

impl AnthropicModel {
	pub fn as_str(&self) -> &'static str {
		match self {
			Self::Claude37Sonnet => "claude-3-7-sonnet-latest",
			Self::Claude35Sonnet => "claude-3-5-sonnet-latest",
			Self::Claude35Haiku => "claude-3-5-haiku-latest",
			Self::Claude3Opus => "claude-3-opus-latest",
			Self::Claude3Sonnet => "claude-3-sonnet-20240229",
			Self::Claude3Haiku => "claude-3-haiku-20240307",
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum GoogleModel {
	#[serde(rename = "gemini-2.0-flash")]
	Gemini20Flash,
	#[serde(rename = "gemini-1.5-flash")]
	Gemini15Flash,
	#[serde(rename = "gemini-1.5-pro")]
	Gemini15Pro,
	#[serde(rename = "gemini-1.5-pro-8b")]
	Gemini15Pro8b,
	#[serde(rename = "gemini-1.0-pro")]
	Gemini10Pro,
}

impl Default for GoogleModel {
	fn default() -> Self {
		Self::Gemini10Pro
	}
}

impl GoogleModel {
	pub fn as_str(&self) -> &'static str {
		match self {
			Self::Gemini20Flash => "gemini-2.0-flash",
			Self::Gemini15Flash => "gemini-1.5-flash",
			Self::Gemini15Pro => "gemini-1.5-pro",
			Self::Gemini15Pro8b => "gemini-1.5-pro-8b",
			Self::Gemini10Pro => "gemini-1.0-pro",
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(transparent)]
pub struct OllamaModel(pub String);

impl Default for OllamaModel {
	fn default() -> Self {
		Self("mistral:7b-instruct".into())
	}
}

impl From<String> for OllamaModel {
	fn from(s: String) -> Self {
		Self(s)
	}
}

impl From<OllamaModel> for String {
	fn from(val: OllamaModel) -> Self {
		val.0
	}
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum GroqModel {
	#[serde(rename = "deepseek-r1-distill-llama-70b")]
	DeepseekR1DistillLlama70B,

	#[serde(rename = "gemma2-9b-it")]
	Gemma2_9BIt,

	#[serde(rename = "llama-3.1-8b-instant")]
	Llama3_1_8bInstant,

	#[serde(rename = "llama-3.2-11b-vision-preview")]
	Llama3_2_11bVisionPreview,

	#[serde(rename = "llama-3.2-1b-preview")]
	Llama3_2_1bPreview,

	#[serde(rename = "llama-3.2-3b-preview")]
	Llama3_2_3bPreview,

	#[serde(rename = "llama-3.2-90b-vision-preview")]
	Llama3_2_90bVisionPreview,

	#[serde(rename = "llama-3.2-70b-specdec")]
	Llama3_2_70bSpecdec,

	#[serde(rename = "llama-3.2-70b-versatile")]
	Llama3_2_70bVersatile,

	#[serde(rename = "llama-guard-3-8b")]
	LlamaGuard3_8b,

	#[serde(rename = "llama3-70b-8192")]
	Llama3_70b_8192,

	#[serde(rename = "llama3-8b-8192")]
	Llama3_8b_8192,

	#[serde(rename = "mixtral-8x7b-32768")]
	Mixtral8x7b_32768,
}

impl Default for GroqModel {
	fn default() -> Self {
		Self::Mixtral8x7b_32768
	}
}

impl GroqModel {
	pub fn as_str(&self) -> &'static str {
		match self {
			Self::DeepseekR1DistillLlama70B => "deepseek-r1-distill-llama-70b",
			Self::Gemma2_9BIt => "gemma2-9b-it",
			Self::Llama3_1_8bInstant => "llama-3.1-8b-instant",
			Self::Llama3_2_11bVisionPreview => "llama-3.2-11b-vision-preview",
			Self::Llama3_2_1bPreview => "llama-3.2-1b-preview",
			Self::Llama3_2_3bPreview => "llama-3.2-3b-preview",
			Self::Llama3_2_90bVisionPreview => "llama-3.2-90b-vision-preview",
			Self::Llama3_2_70bSpecdec => "llama-3.2-70b-specdec",
			Self::Llama3_2_70bVersatile => "llama-3.2-70b-versatile",
			Self::LlamaGuard3_8b => "llama-guard-3-8b",
			Self::Llama3_70b_8192 => "llama3-70b-8192",
			Self::Llama3_8b_8192 => "llama3-8b-8192",
			Self::Mixtral8x7b_32768 => "mixtral-8x7b-32768",
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(non_camel_case_types)]
pub enum TogetherModel {
	#[serde(rename = "meta-llama/Meta-Llama-3-70B-Instruct")]
	Llama3_70B_Instruct,

	#[serde(rename = "meta-llama/Meta-Llama-3-8B-Instruct")]
	Llama3_8B_Instruct,

	#[serde(rename = "mistralai/Mixtral-8x7B-Instruct-v0.1")]
	Mixtral_8x7B_Instruct,

	#[serde(rename = "mistralai/Mistral-7B-Instruct-v0.3")]
	Mistral_7B_Instruct,

	#[serde(rename = "databricks/dbrx-instruct")]
	DBRX_Instruct,

	#[serde(rename = "Qwen/Qwen2.5-72B-Instruct-Turbo")]
	Qwen2_5_72B_Instruct_Turbo,

	#[serde(rename = "HuggingFaceH4/zephyr-7b-beta")]
	Zephyr_7B_Beta,

	#[serde(rename = "NousResearch/Hermes-2-Theta-Llama-3-70B")]
	Hermes2_Llama3_70B,

	#[serde(rename = "deepseek-ai/deepseek-llm-67b-chat")]
	DeepSeek_67B_Chat,

	#[serde(rename = "openchat/openchat-3.5-1210")]
	OpenChat_3_5,
}

impl Default for TogetherModel {
	fn default() -> Self {
		Self::Qwen2_5_72B_Instruct_Turbo
	}
}

impl TogetherModel {
	pub fn as_str(&self) -> &'static str {
		match self {
			Self::Llama3_70B_Instruct => "meta-llama/Meta-Llama-3-70B-Instruct",
			Self::Llama3_8B_Instruct => "meta-llama/Meta-Llama-3-8B-Instruct",
			Self::Mixtral_8x7B_Instruct => "mistralai/Mixtral-8x7B-Instruct-v0.1",
			Self::Mistral_7B_Instruct => "mistralai/Mistral-7B-Instruct-v0.3",
			Self::DBRX_Instruct => "databricks/dbrx-instruct",
			Self::Qwen2_5_72B_Instruct_Turbo => "Qwen/Qwen2.5-72B-Instruct-Turbo",
			Self::Zephyr_7B_Beta => "HuggingFaceH4/zephyr-7b-beta",
			Self::Hermes2_Llama3_70B => "NousResearch/Hermes-2-Theta-Llama-3-70B",
			Self::DeepSeek_67B_Chat => "deepseek-ai/deepseek-llm-67b-chat",
			Self::OpenChat_3_5 => "openchat/openchat-3.5-1210",
		}
	}
}
