use xcap::Window;

pub struct WindowMetadata {
	pub title: String,
	pub app_name: String,
}

impl WindowMetadata {
	pub fn new(window: &Window) -> Self {
		let title = window.title().unwrap_or("unknown".into());
		let app_name = window.app_name().unwrap_or("unknown".into());
		Self { title, app_name }
	}
}

pub fn list_windows() -> Vec<Window> {
	println!("[vision] listing windows");
	match Window::all() {
		Ok(windows) => windows,
		Err(e) => {
			eprintln!("[vision] failed to list windows: {:?}", e);
			std::process::exit(1)
		}
	}
}

pub fn get_focused_window() -> Option<Window> {
	let windows = list_windows();
	println!("[vision] find focused window");
	windows.into_iter().find(|w| w.is_focused().unwrap_or(false))
}

pub fn get_window_metadata(window: &Window) -> WindowMetadata {
	WindowMetadata::new(window)
}
