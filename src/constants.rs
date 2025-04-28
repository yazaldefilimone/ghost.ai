pub const MAIN_DIR_NAME: &str = "ghost";
pub const SETTINGS_FILE_NAME: &str = "settings.toml";
pub const DB_FILE_NAME: &str = ".data.db";

pub const TEMPERATURE: f64 = 0.75;
pub const AUTO_COMPLETE_MAX_TOKENS: u64 = 150;

pub const VISION_DIFF_TOLERANCE: f32 = 0.30; // 30%

pub const DETECTION_MODEL_URL: &str =
	"https://ocrs-models.s3-accelerate.amazonaws.com/text-detection.rten";
pub const RECOGNITION_MODEL_URL: &str =
	"https://ocrs-models.s3-accelerate.amazonaws.com/text-recognition.rten";
