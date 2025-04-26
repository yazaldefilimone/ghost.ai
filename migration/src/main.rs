use sea_orm_migration::prelude::*;
use std::env;
use std::path::PathBuf;

const MAIN_DIR_NAME: &str = "ghost.ai";
const DB_FILE_NAME: &str = ".data.db";

pub fn create_config_file_path(file_name: &str) -> PathBuf {
	dirs::home_dir()
		.unwrap_or_else(|| PathBuf::from("."))
		.join(".config")
		.join(MAIN_DIR_NAME)
		.join(file_name)
}

pub fn database_file_path() -> PathBuf {
	let path = create_config_file_path(DB_FILE_NAME);
	path
}

#[async_std::main]
async fn main() {
	env::set_var("DATABASE_URL", format!("sqlite://{}", database_file_path().display()));
	cli::run_cli(migration::Migrator).await;
}
