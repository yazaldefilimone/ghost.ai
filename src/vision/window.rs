#![allow(dead_code)]
use crate::{vision_error, vision_info};
use xcap::Window;

pub struct WindowMetadata {
	pub title: String,
	pub app_name: String,
}

impl WindowMetadata {
	pub fn from(window: &Window) -> Self {
		Self {
			title: window.title().unwrap_or_else(|_e| "unknown".into()),
			app_name: window.app_name().unwrap_or_else(|_e| "unknown".into()),
		}
	}
}

pub fn lookup_windows() -> Vec<Window> {
	vision_info!("listing windows...");
	Window::all().unwrap_or_else(|err| {
		vision_error!("can't list windows: {err}");
		std::process::exit(1);
	})
}

pub fn lookup_focused_window() -> Option<Window> {
	vision_info!("looking for focused window...");
	lookup_windows().into_iter().find(|w| w.is_focused().unwrap_or(false))
}

pub fn extract_metadata(window: &Window) -> WindowMetadata {
	WindowMetadata::from(window)
}
