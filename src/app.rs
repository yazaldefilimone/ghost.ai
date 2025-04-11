use crate::{channels::Channels, memory::store::Store, runtime::workers, tasks};

pub async fn bootstrap() {
	let mut store = Store::new();
	let (channels, vision_rx, system_rx, llm_rx) = Channels::create();
	tokio::spawn(workers::vision::vision_worker(vision_rx));
	tokio::spawn(tasks::tracker::run(channels.user_tx));
	// tokio::spawn(workers::system::system_worker(system_rx));
	tokio::spawn(workers::llm::llm_worker(llm_rx));
}
