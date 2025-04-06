#[cfg(target_os = "macos")]
use std::process::Command;

#[cfg(target_os = "macos")]
pub fn frontmost_application_name() -> Option<String> {
	let mut command = Command::new("osascript");
	let output = command
		.arg("-e")
		.arg("tell application \"System Events\" to get name of first process whose frontmost is true")
		.output()
		.ok()?;
	Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}
