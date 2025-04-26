use once_cell::sync::Lazy;
use signal::{Signal, BUFFER_SIZE};
use std::sync::{atomic::AtomicBool, Arc};
use tokio::sync::mpsc;

mod api;
mod console;
mod constants;
mod database;
mod embed;
mod llm;
mod logger;
mod security;
mod settings;
mod signal;
mod tasks;
mod text_buffer;
mod typer;
mod utils;
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
	let connection = database::connect().await;
	let settings = settings::loader::load_settings_or_default();
	settings::loader::set_settings(settings);
	let db = Arc::new(connection);
	// Model::delete_all(&db).await;

	let extract_db = Arc::clone(&db);
	let typer_db = Arc::clone(&db);
	let vision_db = Arc::clone(&db);
	let console_db = Arc::clone(&db);

	let double_buffer_size = BUFFER_SIZE * 2;
	let (vision_sender, vision_receiver) = mpsc::channel::<Signal>(double_buffer_size);
	let (extract_sender, extract_receiver) = mpsc::channel::<Signal>(double_buffer_size);
	let (typer_sender, typer_receiver) = mpsc::channel::<Signal>(BUFFER_SIZE);

	tasks::tracker::start_keyboard_listener(vision_sender, typer_sender).await;

	tokio::spawn(async move {
		tasks::vision::start_vision_listener(vision_receiver, extract_sender, vision_db).await;
	});

	tokio::spawn(async move {
		tasks::vision::start_extract_listener(extract_receiver, extract_db).await;
	});
	// help_info!("haha, everything working...");
	tokio::spawn(async move {
		tasks::typer::autocomplete_listener(typer_receiver, typer_db).await;
	});
	console::console(console_db).await.unwrap();
}
