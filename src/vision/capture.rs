use xcap::{Monitor, Window};

use super::FrameCaptured;

pub fn capture_monitor(monitor: &Monitor) -> FrameCaptured {
	match monitor.capture_image() {
		Ok(image) => image,
		Err(err) => {
			eprintln!("[vision] failed seeing monitor: '{}'", monitor.name().unwrap_or("unknown".into()));
			std::process::exit(1);
		}
	}
}

pub fn capture_window(window: &Window) -> FrameCaptured {
	match window.capture_image() {
		Ok(image) => image,
		Err(err) => {
			eprintln!("[vision] failed seeing window: '{}'", window.title().unwrap_or("unknown".into()));
			std::process::exit(1)
		}
	}
}
