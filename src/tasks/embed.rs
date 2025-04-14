use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
	channels::{Signal, SignalReceiver},
	embed,
	memory::store::Store,
};

pub async fn run(mut embed_receiver: SignalReceiver, store: Arc<RwLock<Store>>) {
	while let Some(signal) = embed_receiver.recv().await {
		match signal {
			Signal::Embed(id) => {
				// println!("[embed] received embed signal for id: {}", id);
				let store = Arc::clone(&store);

				tokio::spawn(async move {
					let maybe_extracted_text = {
						let store_read = store.read().await;
						store_read.get(id).map(|entry| entry.extracted_text.clone())
					};
					if let Some(extracted_text) = maybe_extracted_text {
						match embed::api::generate_embedding(&extracted_text).await {
							Ok(embedding) => {
								let mut store_write = store.write().await;
								store_write.add_embedding(id, embedding);
								println!("[embed] embedding saved for id: {}", id);
							}
							Err(err) => {
								eprintln!("[embed] failed to generate embedding for id={id}: {err}");
							}
						}
					} else {
						eprintln!("[embed] no entry found for id={id}");
					}
				});
			}
			_ => {
				eprintln!("[embed] unexpected signal received: {signal:?}");
			}
		}
	}
}
