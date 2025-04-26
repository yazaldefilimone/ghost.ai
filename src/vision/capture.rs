use super::FrameCaptured;
use crate::vision_error;
use xcap::{Monitor, Window};

#[allow(dead_code)]
pub fn capture_monitor(monitor: &Monitor) -> FrameCaptured {
	monitor.capture_image().unwrap_or_else(|_err| {
		let name = monitor.name().unwrap_or_default();
		vision_error!("can't capture monitor '{}'", name);
		std::process::exit(1);
	})
}

pub fn capture_window(window: &Window) -> FrameCaptured {
	window.capture_image().unwrap_or_else(|_err| {
		let name = window.app_name().unwrap_or_default();
		vision_error!("can't capture window '{}'", name);
		std::process::exit(1);
	})
}
