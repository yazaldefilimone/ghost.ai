use tokio::sync::mpsc;

#[derive(Debug)]
pub enum VisionEvent {
	FrameCaptured,
	TextExtracted(String),
}
#[derive(Debug)]
pub enum Signal {
	StartCapture,
	StopCapture,
	UserIdle,
	AppChanged(String),
	FocusChanged(bool),
	Quit,
}

pub enum UserEvent {
	Typed(char),
	Moved(f64, f64),
	Scrolled(i64),
	Clicked,
}

#[derive(Debug)]
pub enum LLMEvent {
	Generate(String),
}

pub type VisionSender = mpsc::Sender<VisionEvent>;
pub type UserSender = mpsc::Sender<UserEvent>;
pub type LLMSender = mpsc::Sender<LLMEvent>;

pub type VisionReceiver = mpsc::Receiver<VisionEvent>;
pub type UserReceiver = mpsc::Receiver<UserEvent>;
pub type LLMReceiver = mpsc::Receiver<LLMEvent>;

pub struct Channels {
	pub vision_tx: VisionSender,
	pub user_tx: UserSender,
	pub llm_tx: LLMSender,
}

const BUFFER_SIZE: usize = 6;

impl Channels {
	pub fn create() -> (Self, VisionReceiver, UserReceiver, LLMReceiver) {
		let (vision_tx, vision_rx) = mpsc::channel(BUFFER_SIZE);
		let (user_tx, input_rx) = mpsc::channel(BUFFER_SIZE);
		let (llm_tx, llm_rx) = mpsc::channel(BUFFER_SIZE);
		(Self { vision_tx, user_tx, llm_tx }, vision_rx, input_rx, llm_rx)
	}
}
