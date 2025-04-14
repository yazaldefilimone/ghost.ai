use xcap::Monitor;

pub fn list_all_monitors() -> Vec<Monitor> {
	// println!("[vision] listing monitors");
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
	// println!("[vision] find primary monitor");
	monitors.into_iter().find(|m| m.is_primary().unwrap_or(false))
}
