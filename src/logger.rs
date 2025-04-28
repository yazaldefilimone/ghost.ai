use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
	#[serde(rename = "info")]
	Info,
	#[serde(rename = "warn")]
	Warn,
	#[serde(rename = "error")]
	Error,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Module {
	#[serde(rename = "vision")]
	Vision,
	#[serde(rename = "typer")]
	Typer,
	#[serde(rename = "embed")]
	Embed,
	#[serde(rename = "llm")]
	Llm,
	#[serde(rename = "keyboard")]
	Keyboard,
	#[serde(rename = "config")]
	Config,
	#[serde(rename = "help")]
	Help,
}

impl Display for Severity {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Severity::Info => write!(f, "{}", "info".cyan()),
			Severity::Warn => write!(f, "{}", "warn".yellow()),
			Severity::Error => write!(f, "{}", "error".red()),
		}
	}
}

impl Display for Module {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Module::Vision => write!(f, "vision"),
			Module::Embed => write!(f, "embed"),
			Module::Llm => write!(f, "llm"),
			Module::Config => write!(f, "config"),
			Module::Help => write!(f, "help"),
			Module::Typer => write!(f, "typer"),
			Module::Keyboard => write!(f, "board"),
		}
	}
}

pub fn log(severity: Severity, module: Module, msg: String) {
	// todo
	if severity == Severity::Error || (severity == Severity::Info && module == Module::Config) {
		let now = chrono::Local::now().format("%H:%M:%S").to_string().dimmed();
		let formatted = format!("{} [{}] {}", now, module, severity);
		println!("{}: {}", formatted, msg);
	}
}

#[macro_export]
macro_rules! vision_info {
	($($arg:tt)*) => {
		$crate::logger::log($crate::logger::Severity::Info, $crate::logger::Module::Vision, format!( $($arg)* ))
	};
}

#[macro_export]
macro_rules! vision_warn {
	($($arg:tt)*) => {
		$crate::logger::log($crate::logger::Severity::Warn, $crate::logger::Module::Vision,format!( $($arg)* ))
	};
}

#[macro_export]
macro_rules! vision_error {
	($($arg:tt)*) => {
		$crate::logger::log($crate::logger::Severity::Error, $crate::logger::Module::Vision,format!( $($arg)* ))
	};
}

#[macro_export]
macro_rules! llm_info {
	($($arg:tt)*) => {
		$crate::logger::log($crate::logger::Severity::Info, $crate::logger::Module::Llm,format!( $($arg)* ))
	};
}
#[macro_export]
macro_rules! llm_warn {
	($($arg:tt)*) => {
		$crate::logger::log($crate::logger::Severity::Warn, $crate::logger::Module::Llm,format!( $($arg)* ))
	};
}
#[macro_export]
macro_rules! llm_error {
	($($arg:tt)*) => {
		$crate::logger::log($crate::logger::Severity::Error, $crate::logger::Module::Llm,format!( $($arg)* ))
	};
}

#[macro_export]
macro_rules! embed_info {
	($($arg:tt)*) => {
		$crate::logger::log($crate::logger::Severity::Info, $crate::logger::Module::Embed,format!( $($arg)* ))
	};
}

#[macro_export]
macro_rules! embed_warn {
	($($arg:tt)*) => {
		$crate::logger::log($crate::logger::Severity::Warn, $crate::logger::Module::Embed,format!( $($arg)* ))
	};
}

#[macro_export]
macro_rules! embed_error {
	($($arg:tt)*) => {
		$crate::logger::log($crate::logger::Severity::Error, $crate::logger::Module::Embed,format!( $($arg)* ))
	};
}

#[macro_export]
macro_rules! config_info {
	($($arg:tt)*) => {
		$crate::logger::log($crate::logger::Severity::Info, $crate::logger::Module::Config,format!( $($arg)* ))
	};
}

#[macro_export]
macro_rules! config_warn {
	($($arg:tt)*) => {
		$crate::logger::log($crate::logger::Severity::Warn, $crate::logger::Module::Config,format!( $($arg)* ))
	};
}

#[macro_export]
macro_rules! config_error {
	($($arg:tt)*) => {
		$crate::logger::log($crate::logger::Severity::Error, $crate::logger::Module::Config,format!( $($arg)* ))
	};
}

#[macro_export]
macro_rules! help_info {
	($($arg:tt)*) => {
		$crate::logger::log($crate::logger::Severity::Info, $crate::logger::Module::Help,format!( $($arg)* ))
	};
}

#[macro_export]
macro_rules! help_warn {
	($($arg:tt)*) => {
		$crate::logger::log($crate::logger::Severity::Warn, $crate::logger::Module::Help,format!( $($arg)* ))
	};
}

#[macro_export]
macro_rules! help_error {
	($($arg:tt)*) => {
		$crate::logger::log($crate::logger::Severity::Error, $crate::logger::Module::Help,format!( $($arg)* ))
	};
}

#[macro_export]
macro_rules! typer_info {
	($($arg:tt)*) => {
		$crate::logger::log($crate::logger::Severity::Info, $crate::logger::Module::Typer,format!( $($arg)* ))
	};
}

#[macro_export]
macro_rules! typer_warn {
	($($arg:tt)*) => {
		$crate::logger::log($crate::logger::Severity::Warn, $crate::logger::Module::Typer,format!( $($arg)* ))
	};
}

#[macro_export]
macro_rules! typer_error {
	($($arg:tt)*) => {
		$crate::logger::log($crate::logger::Severity::Error, $crate::logger::Module::Typer,format!( $($arg)* ))
	};
}

#[macro_export]
macro_rules! keyboard_info {
	($($arg:tt)*) => {
		$crate::logger::log($crate::logger::Severity::Info, $crate::logger::Module::Keyboard,format!( $($arg)* ))
	};
}

#[macro_export]
macro_rules! keyboard_warn {
	($($arg:tt)*) => {
		$crate::logger::log($crate::logger::Severity::Warn, $crate::logger::Module::Keyboard,format!( $($arg)* ))
	};
}

#[macro_export]
macro_rules! keyboard_error {
	($($arg:tt)*) => {
		$crate::logger::log($crate::logger::Severity::Error, $crate::logger::Module::Keyboard,format!( $($arg)* ))
	};
}
