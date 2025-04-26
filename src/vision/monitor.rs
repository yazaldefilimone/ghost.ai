#![allow(dead_code)]

use xcap::{Monitor, XCapError};

use crate::vision_error;

pub fn lookup_monitors() -> Vec<Monitor> {
	Monitor::all().unwrap_or_else(|err| {
		vision_error!("{}", explain_xcap_error(&err));
		std::process::exit(1);
	})
}

pub fn lookup_primary_monitor() -> Option<Monitor> {
	lookup_monitors().into_iter().find(|m| m.is_primary().unwrap_or(false))
}

pub fn explain_xcap_error(err: &XCapError) -> String {
	use XCapError::*;

	match err {
		Error(msg) => format!("internal error: {}", msg),
		StdSyncPoisonError(msg) => format!("thread sync error: {}", msg),

		#[cfg(target_os = "linux")]
		XcbError(_) => "xcb failed to communicate with X server".into(),

		#[cfg(target_os = "linux")]
		XcbConnError(_) => "xcb connection to X server failed".into(),

		#[cfg(target_os = "linux")]
		ImageImageError(_) => "failed to process image data".into(),

		#[cfg(target_os = "linux")]
		StdStringFromUtf8Error(_) => "utf-8 conversion failed (invalid string)".into(),

		#[cfg(target_os = "linux")]
		DbusError(_) => "something went wrong with session bus in D-Bus".into(),

		#[cfg(target_os = "linux")]
		StdIOError(_) => "problem reading or writing data, probably a stdio error".into(),

		#[cfg(target_os = "linux")]
		StdTimeSystemTimeError(_) => "system time error".into(),

		#[cfg(target_os = "linux")]
		LibwayshotError(_) => "wayshot: screenshot backend failed".into(),

		#[cfg(target_os = "macos")]
		Objc2CoreGraphicsCGError(code) => format!("core graphics failed (code: {:?})", code),

		#[cfg(target_os = "windows")]
		WindowsCoreError(_) => "windows API error".into(),

		#[cfg(target_os = "windows")]
		Utf16Error(_) => "UTF-16 conversion error".into(),
	}
}
