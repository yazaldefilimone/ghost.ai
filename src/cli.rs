use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ghost-ai")]
#[command(about = "Second brain for your computer", long_about = None)]
pub struct Cli {
	#[command(subcommand)]
	pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
	Run,
	Init,
}
