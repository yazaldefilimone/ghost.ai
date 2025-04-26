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

pub fn key_to_macos_string(key: &Key) -> String {
	match key {
		Key::Alt => "Option".to_string(),
		Key::AltGr => "AltGr".to_string(),
		Key::Backspace => "Backspace".to_string(),
		Key::CapsLock => "CapsLock".to_string(),
		Key::ControlLeft | Key::ControlRight => "Ctrl".to_string(),
		Key::Delete => "Delete".to_string(),
		Key::DownArrow => "Down".to_string(),
		Key::End => "End".to_string(),
		Key::Escape => "Esc".to_string(),
		Key::F1 => "F1".to_string(),
		Key::F2 => "F2".to_string(),
		Key::F3 => "F3".to_string(),
		Key::F4 => "F4".to_string(),
		Key::F5 => "F5".to_string(),
		Key::F6 => "F6".to_string(),
		Key::F7 => "F7".to_string(),
		Key::F8 => "F8".to_string(),
		Key::F9 => "F9".to_string(),
		Key::F10 => "F10".to_string(),
		Key::F11 => "F11".to_string(),
		Key::F12 => "F12".to_string(),
		Key::Home => "Home".to_string(),
		Key::LeftArrow => "Left".to_string(),
		Key::MetaLeft | Key::MetaRight => "Cmd".to_string(),
		Key::PageDown => "PageDown".to_string(),
		Key::PageUp => "PageUp".to_string(),
		Key::Return => "Return".to_string(),
		Key::RightArrow => "Right".to_string(),
		Key::ShiftLeft | Key::ShiftRight => "Shift".to_string(),
		Key::Space => "Space".to_string(),
		Key::Tab => "Tab".to_string(),
		Key::UpArrow => "Up".to_string(),
		Key::PrintScreen => "PrintScreen".to_string(),
		Key::ScrollLock => "ScrollLock".to_string(),
		Key::Pause => "Pause".to_string(),
		Key::NumLock => "NumLock".to_string(),
		Key::BackQuote => "`".to_string(),
		Key::Minus => "-".to_string(),
		Key::Equal => "=".to_string(),
		Key::KeyQ => "q".to_string(),
		Key::KeyW => "w".to_string(),
		Key::KeyE => "e".to_string(),
		Key::KeyR => "r".to_string(),
		Key::KeyT => "t".to_string(),
		Key::KeyY => "y".to_string(),
		Key::KeyU => "u".to_string(),
		Key::KeyI => "i".to_string(),
		Key::KeyO => "o".to_string(),
		Key::KeyP => "p".to_string(),
		Key::LeftBracket => "[".to_string(),
		Key::RightBracket => "]".to_string(),
		Key::KeyA => "a".to_string(),
		Key::KeyS => "s".to_string(),
		Key::KeyD => "d".to_string(),
		Key::KeyF => "f".to_string(),
		Key::KeyG => "g".to_string(),
		Key::KeyH => "h".to_string(),
		Key::KeyJ => "j".to_string(),
		Key::KeyK => "k".to_string(),
		Key::KeyL => "l".to_string(),
		Key::SemiColon => ";".to_string(),
		Key::Quote => "'".to_string(),
		Key::BackSlash => "\\".to_string(),
		Key::IntlBackslash => "\\".to_string(),
		Key::KeyZ => "z".to_string(),
		Key::KeyX => "x".to_string(),
		Key::KeyC => "c".to_string(),
		Key::KeyV => "v".to_string(),
		Key::KeyB => "b".to_string(),
		Key::KeyN => "n".to_string(),
		Key::KeyM => "m".to_string(),
		Key::Comma => ",".to_string(),
		Key::Dot => ".".to_string(),
		Key::Slash => "/".to_string(),
		_ => "".to_string(),
	}
}
