use crate::channels::{LLMEvent, LLMReceiver};

pub async fn llm_worker(mut rx: LLMReceiver) {
	while let Some(task) = rx.recv().await {
		match task {
			LLMEvent::Generate(input) => {
				println!("[llm] generate: {}", input);
			}
		}
	}
}
