use xcap::Monitor;
use xcap::Window;

pub fn list_all_monitors() -> Vec<Monitor> {
	println!("[vision] listing monitors");
	match Monitor::all() {
		Ok(monitors) => monitors,
		Err(e) => {
			eprintln!("[vision] failed to list monitors: {:?}", e);
			std::process::exit(1)
		}
	}
}

pub fn get_primary_monitor() -> Option<Monitor> {
	let monitors = list_all_monitors();
	println!("[vision] find primary monitor");
	monitors.into_iter().find(|m| m.is_primary().unwrap_or(false))
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
