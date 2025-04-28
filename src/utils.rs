use std::{fmt::Write, fs, io, path::PathBuf};

use rdev::Key;

use crate::{
	constants::{DB_FILE_NAME, MAIN_DIR_NAME, SETTINGS_FILE_NAME},
	database::Model,
};

pub fn create_config_file_path(file_name: &str) -> PathBuf {
	dirs::home_dir()
		.unwrap_or_else(|| PathBuf::from("."))
		.join(".config")
		.join(MAIN_DIR_NAME)
		.join(file_name)
}

pub fn config_file_path() -> PathBuf {
	let path = create_config_file_path(SETTINGS_FILE_NAME);
	let _ = ensure_config_file_exists(&path);
	path
}

pub fn database_file_path() -> PathBuf {
	let path = create_config_file_path(DB_FILE_NAME);
	let _ = ensure_config_file_exists(&path);
	path
}

pub fn ensure_config_file_exists(path: &PathBuf) -> io::Result<()> {
	if path.exists() {
		return Ok(());
	}
	if let Some(parent) = path.parent() {
		fs::create_dir_all(parent)?;
	}
	fs::write(path, b"")
}

pub fn format_model_as_context(models: &[Model]) -> String {
	let mut out = String::with_capacity(models.len() * 256);
	for model in models {
		let app = model.app_name.as_deref().unwrap_or("unknown");
		let window = model.window_title.as_deref().unwrap_or("unknown");
		let content = model.extracted_text.trim();
		writeln!(
			&mut out,
			"<context>
			  App: {app}
				Window Title: {window}
				Content:
				  {content}
			</context>",
		)
		.ok();
	}
	out
}
pub fn key_to_platform_string(key: &Key) -> String {
	let base = default_key_label(key);

	if let Some(patched) = patch_key_label_for_platform(key) {
		patched
	} else {
		base
	}
}

fn default_key_label(key: &Key) -> String {
	match key {
		Key::Alt => "Alt",
		Key::AltGr => "AltGr",
		Key::Backspace => "Backspace",
		Key::CapsLock => "CapsLock",
		Key::ControlLeft | Key::ControlRight => "Ctrl",
		Key::Delete => "Delete",
		Key::DownArrow => "Down",
		Key::End => "End",
		Key::Escape => "Esc",
		Key::F1 => "F1",
		Key::F2 => "F2",
		Key::F3 => "F3",
		Key::F4 => "F4",
		Key::F5 => "F5",
		Key::F6 => "F6",
		Key::F7 => "F7",
		Key::F8 => "F8",
		Key::F9 => "F9",
		Key::F10 => "F10",
		Key::F11 => "F11",
		Key::F12 => "F12",
		Key::Home => "Home",
		Key::LeftArrow => "Left",
		Key::PageDown => "PageDown",
		Key::PageUp => "PageUp",
		Key::RightArrow => "Right",
		Key::ShiftLeft | Key::ShiftRight => "Shift",
		Key::Space => "Space",
		Key::Tab => "Tab",
		Key::UpArrow => "Up",
		Key::PrintScreen => "PrintScreen",
		Key::ScrollLock => "ScrollLock",
		Key::Pause => "Pause",
		Key::NumLock => "NumLock",
		Key::BackQuote => "`",
		Key::Minus => "-",
		Key::Equal => "=",
		Key::KeyQ => "q",
		Key::KeyW => "w",
		Key::KeyE => "e",
		Key::KeyR => "r",
		Key::KeyT => "t",
		Key::KeyY => "y",
		Key::KeyU => "u",
		Key::KeyI => "i",
		Key::KeyO => "o",
		Key::KeyP => "p",
		Key::LeftBracket => "[",
		Key::RightBracket => "]",
		Key::KeyA => "a",
		Key::KeyS => "s",
		Key::KeyD => "d",
		Key::KeyF => "f",
		Key::KeyG => "g",
		Key::KeyH => "h",
		Key::KeyJ => "j",
		Key::KeyK => "k",
		Key::KeyL => "l",
		Key::SemiColon => ";",
		Key::Quote => "'",
		Key::BackSlash => "\\",
		Key::IntlBackslash => "\\",
		Key::KeyZ => "z",
		Key::KeyX => "x",
		Key::KeyC => "c",
		Key::KeyV => "v",
		Key::KeyB => "b",
		Key::KeyN => "n",
		Key::KeyM => "m",
		Key::Comma => ",",
		Key::Dot => ".",
		Key::Slash => "/",
		_ => "",
	}
	.to_string()
}

fn patch_key_label_for_platform(key: &Key) -> Option<String> {
	if cfg!(target_os = "macos") {
		match key {
			Key::MetaLeft | Key::MetaRight => Some("Cmd".to_string()),
			Key::Return => Some("Return".to_string()),
			_ => None,
		}
	} else if cfg!(target_os = "windows") {
		match key {
			Key::MetaLeft | Key::MetaRight => Some("Win".to_string()),
			Key::Return => Some("Enter".to_string()),
			Key::PrintScreen => Some("PrtSc".to_string()),
			_ => None,
		}
	} else {
		// Linux ou outros
		match key {
			Key::MetaLeft | Key::MetaRight => Some("Super".to_string()),
			Key::Return => Some("Enter".to_string()),
			Key::PrintScreen => Some("Print".to_string()),
			_ => None,
		}
	}
}
