use crate::channels::{VisionEvent, VisionReceiver};

pub async fn vision_worker(mut rx: VisionReceiver) {
	while let Some(event) = rx.recv().await {
		match event {
			VisionEvent::FrameCaptured => {
				println!("[vision] window taken");
			}
			VisionEvent::TextExtracted(text) => {
				println!("[vision] extracted: {}", text);
			}
		}
	}
}
