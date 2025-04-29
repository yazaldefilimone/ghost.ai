use crate::utils::key_to_platform_string;
use rdev::Key;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub struct Shortcuts {
	pub macos: Option<ShortcutMap>,
	pub windows: Option<ShortcutMap>,
	pub linux: Option<ShortcutMap>,
}

impl Shortcuts {
	pub fn get_current(&self) -> &ShortcutMap {
		#[cfg(target_os = "macos")]
		{
			self.macos.as_ref().unwrap()
		}
		#[cfg(target_os = "windows")]
		{
			self.windows.as_ref().unwrap()
		}
		#[cfg(target_os = "linux")]
		{
			self.linux.as_ref().unwrap()
		}
	}
	// todo: bro, what is this? please, refactor :)
	pub fn look_shortcut(&self, pressed: &[Key]) -> bool {
		let look = &self.get_current().look;
		let text = pressed.iter().map(key_to_platform_string).collect::<Vec<_>>().join("+");
		look.to_lowercase().trim() == text.to_lowercase().trim()
	}

	// todo: bro, what is this? please, refactor :)
	pub fn autocomplete_shortcut(&self, pressed: &[Key]) -> bool {
		let autocomplete = &self.get_current().autocomplete;
		let text = pressed.iter().map(key_to_platform_string).collect::<Vec<_>>().join("+");
		autocomplete.to_lowercase().trim() == text.to_lowercase().trim()
	}

	#[allow(dead_code)]
	// todo: bro, what is this? please, refactor :)
	pub fn hear_shortcut(&self, pressed: &[Key]) -> bool {
		let hear = &self.get_current().hear;
		let text = pressed.iter().map(key_to_platform_string).collect::<Vec<_>>().join("+");
		hear.to_lowercase().trim() == text.to_lowercase().trim()
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShortcutMap {
	pub look: String,
	pub autocomplete: String,
	pub hear: String,
}

impl Default for ShortcutMap {
	fn default() -> Self {
		#[cfg(target_os = "macos")]
		{
			return Self { look: "cmd+v".into(), autocomplete: "cmd+a".into(), hear: "cmd+h".into() };
		}
		#[cfg(target_os = "windows")]
		{
			return Self { look: "ctrl+v".into(), autocomplete: "ctrl+a".into(), hear: "ctrl+h".into() };
		}
		#[cfg(target_os = "linux")]
		{
			return Self { look: "ctrl+v".into(), autocomplete: "ctrl+a".into(), hear: "ctrl+h".into() };
		}
	}
}

impl Default for Shortcuts {
	fn default() -> Self {
		let shortcut_map = ShortcutMap::default();
		#[cfg(target_os = "macos")]
		{
			Self { macos: Some(shortcut_map), windows: None, linux: None }
		}
		#[cfg(target_os = "windows")]
		{
			Self { macos: None, windows: Some(shortcut_map), linux: None }
		}

		#[cfg(target_os = "linux")]
		{
			Self { macos: None, windows: None, linux: Some(shortcut_map) }
		}
	}
}
