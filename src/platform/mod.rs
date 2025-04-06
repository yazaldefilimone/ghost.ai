// #[cfg(target_os = "linux")]
// mod linux;
// #[cfg(target_os = "linux")]
// pub use linux::*;

// #[cfg(target_os = "windows")]
// mod windows;
// #[cfg(target_os = "windows")]
// pub use windows::*;

#[cfg(target_os = "macos")]
mod macos;

pub fn frontmost_application_name() -> Option<String> {
	#[cfg(target_os = "macos")]
	{
		macos::frontmost_application_name()
	}

	#[cfg(target_os = "windows")]
	{
		return None;
	};

	#[cfg(target_os = "linux")]
	{
		return None;
	};
}
