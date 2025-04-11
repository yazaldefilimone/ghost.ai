use rdev::{Button, EventType, Keyboard, KeyboardState};
use tokio::sync::mpsc::Sender;

use crate::channels::UserEvent;

pub async fn run(tx: Sender<UserEvent>) {
	let mut keyboard = rdev::Keyboard::new().unwrap();
	let result = rdev::listen(move |event| {
		keyboard_event(event, &mut keyboard);
	});
}

fn keyboard_event(event: rdev::Event, board: &mut Keyboard) {
	match event.event_type {
		EventType::ButtonPress(mouse_btn) => match mouse_btn {
			Button::Left => {
				println!("left click");
			}
			Button::Right => {
				println!("right click");
			}
			Button::Middle => {
				println!("middle click");
			}
			Button::Unknown(btn) => {}
		},
		EventType::Wheel { delta_y, .. } => {
			println!("wheel {}", delta_y);
		}
		EventType::MouseMove { x, y } => {
			println!("mouse move {}, {}", x, y);
		}
		event => {
			if let Some(character) = board.add(&event) {
				println!("character {}", character);
			}
		}
	};
}
