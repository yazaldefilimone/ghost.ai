use crate::{database::Model, llm::manager::build_provider, settings::loader};
use crossterm::{
	event::{self, Event, KeyCode},
	ExecutableCommand,
};
use sea_orm::DbConn;
use std::{
	io::{stdout, Write},
	sync::Arc,
};

type Result<T> = std::result::Result<T, std::io::Error>;

fn read_line(prompt: &str) -> Result<String> {
	let mut out = stdout();
	out.execute(crossterm::style::Print(prompt))?;
	out.flush()?;
	let mut input = String::new();
	loop {
		if let Event::Key(key) = event::read()? {
			match key.code {
				KeyCode::Char(c) => {
					input.push(c);
					out.flush()?;
				}
				KeyCode::Enter => {
					break;
				}
				KeyCode::Backspace => {
					if input.pop().is_some() {
						out.execute(crossterm::cursor::MoveLeft(1))?;
						print!(" ");
						out.execute(crossterm::cursor::MoveLeft(1))?;
						out.flush()?;
					}
				}
				_ => {}
			}
		};
	}
	Ok(input.trim().to_string())
}

fn hard_reset_terminal() {
	print!("\x1Bc");
}

pub fn show_help() {
	hard_reset_terminal();
	// println!("console commands:");
	println!(".chat - start a chat");
	println!(".exit - exit the console");
	println!(".delete - delete memories");
	println!(".count - count memory entries");
	println!(".clear - clear the screen");
	stdout().flush().unwrap();
}

pub fn show_chat_help() {
	hard_reset_terminal();
	println!(".exit - exit the chat");
	println!(".clear - clear the screen");
	stdout().flush().unwrap();
}

async fn chat_console(db: Arc<DbConn>) -> Result<()> {
	show_chat_help();
	let model = &loader::current_settings().chat.model;
	let llm = build_provider(model, None, None);
	loop {
		let input = read_line("chat >> ")?;

		if input.is_empty() {
			continue;
		}
		if input == ".exit" {
			break;
		}
		if input == ".clear" {
			show_chat_help();
			continue;
		}

		if !input.is_empty() {
			print!("ghost:");
			stdout().flush().unwrap();
			match llm
				.ask_question(&input, db.clone(), |text| {
					print!("{}", text);
					stdout().flush().unwrap();
				})
				.await
			{
				Ok(_) => {
					println!();
					stdout().flush().unwrap();
				}
				Err(e) => {
					println!("error: {}", e);
					stdout().flush().unwrap();
				}
			}
		}
	}
	Ok(())
}

pub async fn console(db: Arc<DbConn>) -> Result<()> {
	show_help();
	loop {
		match read_line("> ")?.as_str() {
			".chat" => {
				chat_console(db.clone()).await?;
				show_help();
			}
			".delete" => {
				Model::delete_all(&db).await;
				println!("done.");
			}
			".clear" => {
				show_help();
			}
			".count" => {
				let count = Model::get_all(&db).await.len();
				println!("count: {}", count);
			}
			".exit" => {
				println!("Bye!");
				break;
			}
			_ => {}
		}
	}
	Ok(())
}
