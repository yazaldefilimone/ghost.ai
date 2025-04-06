#![allow(dead_code, unused_variables)]

use runtime::state::SharedState;
use tokio::{
	io::{self, AsyncBufReadExt, BufReader},
	sync::mpsc,
};

mod embed;
mod llm;
mod memory_store;
mod platform;
mod runtime;
mod text_state;

use once_cell::sync::Lazy;
use std::{
	process::exit,
	sync::{
		atomic::{AtomicBool, Ordering},
		Arc,
	},
};

pub static IS_PAUSED: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));
pub fn show_help() {
	println!("commands: .mute / .unmute / .quit");
	println!("ready. start typing...");
	println!()
}

#[tokio::main]
async fn main() {
	let (trigger_tx, trigger_rx) = mpsc::channel::<()>(1);
	let state = Arc::new(SharedState::new());
	let ai_state = Arc::clone(&state);
	let listen_state = Arc::clone(&state);

	// runtime::ai::complete_suggestion(ai_state, trigger_rx).await;
	runtime::ai::stream_suggestion(ai_state, trigger_rx).await;
	runtime::listen(listen_state, trigger_tx).await;

	let stdin = BufReader::new(io::stdin());
	let mut lines = stdin.lines();
	show_help();
	while let Ok(Some(line)) = lines.next_line().await {
		match line.trim() {
			".mute" | ".m" => {
				IS_PAUSED.store(true, Ordering::SeqCst);
				println!("muted, type '.unmute' to unmute");
			}
			".unmute" | ".u" => {
				IS_PAUSED.store(false, Ordering::SeqCst);
				println!("unmuted, type '.mute' to mute");
			}
			".quit" | ".q" => exit(0),
			".help" | ".h" => show_help(),
			_ => {}
		}
	}
}
