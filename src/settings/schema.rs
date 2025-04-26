use serde::{Deserialize, Serialize};

use super::provider::LlmModel;
use super::shortcuts::Shortcuts;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Settings {
	#[serde(default)]
	pub name: String,
	#[serde(default)]
	pub language: Language,
	#[serde(default)]
	pub debug: bool,

	#[serde(default)]
	pub embed: LlmModel,
	#[serde(default)]
	pub autocomplete: Autocomplete,

	#[serde(default)]
	pub chat: Chat,

	#[serde(default)]
	pub hear: Hear,

	#[serde(default)]
	pub vision: Vision,
	#[serde(default)]
	pub shortcuts: Shortcuts,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Language {
	#[serde(rename = "en")]
	English,
	#[serde(rename = "fr")]
	French,
	#[serde(rename = "de")]
	German,
	#[serde(rename = "es")]
	Spanish,
	#[serde(rename = "it")]
	Italian,
	#[serde(rename = "pt")]
	Portuguese,
	#[serde(rename = "ru")]
	Russian,
}

impl Default for Language {
	fn default() -> Self {
		Self::English
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FeatureToggle {
	pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Chat {
	pub enabled: bool,
	#[serde(flatten)]
	pub model: LlmModel,
}

impl Default for Chat {
	fn default() -> Self {
		Self { enabled: true, model: LlmModel::default() }
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Hear {
	pub enabled: bool,
	pub model: String,
}

impl Default for Hear {
	fn default() -> Self {
		Self { enabled: false, model: "whisper".into() }
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Vision {
	pub skip_app: Vec<String>,
	pub skip_patterns: Vec<String>,
	pub security_skip: bool,
}

impl Vision {
	pub fn skip_app(&self, app_name: &str) -> bool {
		self.skip_app.iter().any(|app| app.to_lowercase() == app_name.to_lowercase())
	}
}

impl Default for Vision {
	fn default() -> Self {
		Self { skip_app: vec!["code".into(), "zed".into()], skip_patterns: vec![], security_skip: true }
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Hotkeys {
	capture: String,
}

impl Default for Hotkeys {
	fn default() -> Self {
		Self { capture: "cmd+shift+c".into() }
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Autocomplete {
	#[serde(flatten)]
	pub model: LlmModel,
	pub stream: bool,
	pub skip_app: Vec<String>,
	pub discard_behavior: DiscardBehavior,
}

impl Default for Autocomplete {
	fn default() -> Self {
		Self {
			model: LlmModel::default(),
			stream: true,
			skip_app: vec!["code".into(), "zed".into()],
			discard_behavior: DiscardBehavior::default(),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DiscardBehavior {
	#[serde(rename = "full")]
	Full,
	#[serde(rename = "inline")]
	Inline,
}

impl Default for DiscardBehavior {
	fn default() -> Self {
		Self::Full
	}
}
