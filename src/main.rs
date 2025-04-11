#![allow(dead_code, unused_variables)]

// use runtime::state::SharedState;
// use tokio::{
// 	io::{self, AsyncBufReadExt, BufReader},
// 	sync::mpsc,
// };

mod app;
mod channels;
mod console;
mod embed;
mod llm;
mod memory;
mod memory_store;
mod platform;
mod runtime;
mod tasks;
mod text_state;
mod tracker;
mod vision;
// mod watcher;

// use arboard::Clipboard;
use once_cell::sync::Lazy;
use std::sync::{
	atomic::{
		AtomicBool,
		// Ordering
	},
	// 	Arc,
};

pub static IS_PAUSED: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));
pub fn show_help() {
	println!("commands: .mute / .unmute / .quit");
	println!("ready. start typing...");
	println!()
}

#[tokio::main]
async fn main() {
	app::bootstrap().await;
}

// async fn main() {
// 	let engine = vision::ocr::Engine::new();
// 	// let Some(primary_monitor) = vision::monitor::get_primary_monitor() else {
// 	// 	println!("[vision] failed to get primary monitor");
// 	// 	std::process::exit(1);
// 	// };
// 	//
// 	// let frame = vision::capture::capture_monitor(&primary_monitor);

// 	let Some(focused_window) = vision::monitor::get_focused_window() else {
// 		println!("[vision] failed to get primary monitor");
// 		std::process::exit(1);
// 	};
// 	let frame = vision::capture::capture_window(&focused_window);

// 	let frame_data = engine.recognize(frame.into());
// 	println!("frame_data {}", frame_data.join("\n"));
// 	let summary = vision::llm::summarize(frame_data.join("\n").as_str()).await.unwrap();
// 	// println!("image_info {}", image_info);
// 	println!("summary {}", summary);
// 	// let (trigger_tx, trigger_rx) = mpsc::channel::<()>(1);
// 	// let state = Arc::new(SharedState::new());
// 	// let ai_state = Arc::clone(&state);
// 	// let listen_state = Arc::clone(&state);

// 	// // runtime::ai::complete_suggestion(ai_state, trigger_rx).await;
// 	// runtime::ai::stream_suggestion(ai_state, trigger_rx).await;
// 	// runtime::listen(listen_state, trigger_tx).await;

// 	// let stdin = BufReader::new(io::stdin());
// 	// let mut lines = stdin.lines();
// 	// show_help();
// 	// while let Ok(Some(line)) = lines.next_line().await {
// 	// 	match line.trim() {
// 	// 		".mute" | ".m" => {
// 	// 			IS_PAUSED.store(true, Ordering::SeqCst);
// 	// 			println!("muted, type '.unmute' to unmute");
// 	// 		}
// 	// 		".unmute" | ".u" => {
// 	// 			IS_PAUSED.store(false, Ordering::SeqCst);
// 	// 			println!("unmuted, type '.mute' to mute");
// 	// 		}
// 	// 		".quit" | ".q" => exit(0),
// 	// 		".help" | ".h" => show_help(),
// 	// 		_ => {}
// 	// 	}
// 	// }
// }
