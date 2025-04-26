use tokio::sync::mpsc;

use crate::vision::FrameCaptured;

#[derive(Debug)]
pub struct Extract {
	pub app_name: String,
	pub window_title: String,
	pub frame: FrameCaptured,
}

impl Extract {
	pub fn new(app_name: String, window_title: String, frame: FrameCaptured) -> Self {
		Self { app_name, window_title, frame }
	}
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Signal {
	ForceCapture,
	// SoftCapture,
	Extract(Extract),
	// Embed(usize),
	Type(String),
	Backspace(usize),
	// Quit,
}

pub const BUFFER_SIZE: usize = 10;

pub type SignalSender = mpsc::Sender<Signal>;
pub type SignalReceiver = mpsc::Receiver<Signal>;
