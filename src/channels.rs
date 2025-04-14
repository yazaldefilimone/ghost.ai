use tokio::sync::mpsc;

#[derive(Debug)]
pub enum Signal {
	ForceCapture,
	SoftCapture,
	Embed(usize),
	Llm,
	Quit,
}

pub const BUFFER_SIZE: usize = 6;

pub type SignalSender = mpsc::Sender<Signal>;
pub type SignalReceiver = mpsc::Receiver<Signal>;

#[derive(Clone)]
pub struct SignalBus {
	vision_tx: mpsc::Sender<Signal>,
	embed_tx: mpsc::Sender<Signal>,
}

impl SignalBus {
	pub fn new(vision_tx: mpsc::Sender<Signal>, embed_tx: mpsc::Sender<Signal>) -> Self {
		Self { vision_tx, embed_tx }
	}

	pub fn send(&self, signal: Signal) {
		let result = match signal {
			Signal::ForceCapture | Signal::SoftCapture => self.vision_tx.try_send(signal),
			Signal::Embed(_) => self.embed_tx.try_send(signal),
			Signal::Llm => self.vision_tx.try_send(Signal::Llm),
			Signal::Quit => {
				std::process::exit(0);
			}
		};

		if let Err(err) = result {
			eprintln!("[signal] failed to send: {}", err);
		}
	}
}
