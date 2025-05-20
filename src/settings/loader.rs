use once_cell::sync::OnceCell;
use std::fs;

use super::schema::Settings;
use crate::{config_error, utils::config_file_path};

static SETTINGS: OnceCell<Settings> = OnceCell::new();

pub fn set_settings(settings: Settings) {
	SETTINGS.set(settings).unwrap();
}

pub fn current_settings() -> &'static Settings {
	SETTINGS.get().unwrap_or_else(|| {
		config_error!("settings not initialized");
		std::process::exit(1);
	})
}

pub fn read_settings() -> Option<Settings> {
	let path = config_file_path();

	let content = fs::read_to_string(&path)
		.map_err(|err| {
			config_error!("failed to read '{}': {}", path.display(), err);
		})
		.ok()?;

	if content.trim().is_empty() {
		return None;
	}
	match toml::from_str::<Settings>(&content) {
		Ok(settings) => Some(settings),
		Err(err) => {
			config_error!("failed to parse '{}': {}", path.display(), err);
			std::process::exit(1);
		}
	}
}

pub fn load_settings_or_default() -> Settings {
	read_settings().unwrap_or_else(|| {
		let default = Settings::default();
		write_settings(&default);
		default
	})
}

pub fn write_settings(settings: &Settings) {
	let path = config_file_path();

	let toml_string = match toml::to_string(settings) {
		Ok(s) => s,
		Err(err) => {
			config_error!("failed to serialize settings: {}", err);
			return;
		}
	};

	if let Err(err) = fs::write(&path, toml_string) {
		config_error!("failed to save settings to '{}': {}", path.display(), err);
	}
}
