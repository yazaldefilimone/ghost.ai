use std::{fs, path::PathBuf};

use reqwest::Client;

use crate::{
	config_error, config_info,
	constants::{DETECTION_MODEL_URL, RECOGNITION_MODEL_URL},
	utils::create_config_file_path,
};

pub fn detection_model_path() -> PathBuf {
	create_config_file_path("vision/models").join("text-detection.rten")
}

pub fn recognition_model_path() -> PathBuf {
	create_config_file_path("vision/models").join("text-recognition.rten")
}

pub async fn run_init_vision() {
	let models_dir = create_config_file_path("vision/models");
	if !models_dir.exists() {
		if let Err(e) = fs::create_dir_all(&models_dir) {
			config_error!("failed to create models directory, reason: {}", e);
			std::process::exit(1);
		}
	}
	let text_detection_path = models_dir.join("text-detection.rten");
	download_model(DETECTION_MODEL_URL, text_detection_path, "text-detection").await;
	let text_recognition_path = models_dir.join("text-recognition.rten");
	download_model(RECOGNITION_MODEL_URL, text_recognition_path, "text-recognition").await;
}

async fn download_model(url: &str, path: PathBuf, name: &str) {
	if path.exists() {
		config_info!("'{}', skipping download...", name);
		return;
	}
	let client = Client::new();
	let response = match client.get(url).send().await {
		Ok(response) => response,
		Err(e) => {
			config_error!("failed to check model '{}', reason: {}", name, e);
			std::process::exit(1)
		}
	};

	let response = match response.error_for_status() {
		Ok(response) => response,
		Err(e) => {
			config_error!("failed to check model '{}', reason: {}", name, e);
			std::process::exit(1)
		}
	};

	let bytes = match response.bytes().await {
		Ok(bytes) => bytes,
		Err(e) => {
			config_error!("failed to download model '{}', reason: {}", name, e);
			std::process::exit(1)
		}
	};
	match fs::write(&path, &bytes) {
		Ok(_) => config_info!("downloaded model '{}'", name),
		Err(e) => config_error!("failed to write model '{}', reason: {}", name, e),
	}
}
