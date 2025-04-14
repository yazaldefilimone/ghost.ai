#![allow(dead_code, unused_variables)]

use channels::{Signal, BUFFER_SIZE};
use memory::store::Store;
use once_cell::sync::Lazy;
use runtime::state::SharedState;
use std::sync::{atomic::AtomicBool, Arc};
use tokio::sync::{mpsc, RwLock};

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
use crate::vision::ocr::TextEngine;

pub static IS_PAUSED: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));

fn create_text_engine() -> TextEngine {
	TextEngine::new()
}

pub static OCR_ENGINE: Lazy<TextEngine> = Lazy::new(create_text_engine);

#[inline(always)]
pub fn get_text_engine() -> &'static TextEngine {
	&OCR_ENGINE
}

#[tokio::main]
async fn main() {
	let store = Arc::new(RwLock::new(Store::new()));
	let state = Arc::new(SharedState::new());
	let listen_state = Arc::clone(&state);
	let store_embed = Arc::clone(&store);
	let store_vision = Arc::clone(&store);

	let (vision_sender, vision_receiver) = mpsc::channel::<Signal>(BUFFER_SIZE);
	let (embed_sender, embed_receiver) = mpsc::channel::<Signal>(BUFFER_SIZE);
	let (llm_sender, llm_receiver) = mpsc::channel::<Signal>(BUFFER_SIZE);

	tasks::tracker::run(vision_sender, llm_sender, listen_state).await;

	// tasks::vision::run(vision_receiver, embed_sender, store_vision).await;
	// tasks::embed::run(embed_receiver, store_embed).await;
	tokio::spawn(async move {
		tasks::vision::run(vision_receiver, embed_sender, store_vision).await;
	});

	tokio::spawn(async move {
		tasks::embed::run(embed_receiver, store_embed).await;
	});

	llm::chat_loop(store.clone()).await.unwrap();
	// let ai_state = Arc::clone(&state);
	// runtime::ai::stream_suggestion(ai_state, store, llm_receiver).await;
}
