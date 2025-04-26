#![allow(dead_code)]

use super::AppState;
use crate::{llm::manager, settings::loader};
use axum::{
	extract::{Json, State},
	response::sse::{Event, Sse},
	routing::post,
	Router,
};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct ChatRequest {
	pub prompt: String,
}

pub fn routes() -> Router<Arc<AppState>> {
	Router::new().route("/chat/stream", post(chat_stream))
}

async fn chat_stream(
	State(state): State<Arc<AppState>>,
	Json(input): Json<ChatRequest>,
) -> Sse<impl futures_util::Stream<Item = Result<Event, axum::Error>>> {
	let prompt = input.prompt;
	let db = state.db.clone();
	let settings = loader::current_settings();
	let llm_model = &settings.chat.model;
	let llm = manager::build_provider(llm_model, None, None);
	let (tx, rx) = tokio::sync::mpsc::channel(8);
	tokio::spawn(async move {
		let _ = llm
			.complete_stream(&prompt, db, move |chunk| {
				let _ = tx.blocking_send(Ok(Event::default().data(chunk)));
			})
			.await;
	});
	Sse::new(tokio_stream::wrappers::ReceiverStream::new(rx))
		.keep_alive(axum::response::sse::KeepAlive::default())
}
