use crate::typer_error;
use enigo::{
	Direction::{Press, Release},
	Enigo, InputError, Key, Keyboard, NewConError, Settings,
};
use std::{thread, time::Duration};

fn create_enigo() -> Option<Enigo> {
	match Enigo::new(&Settings::default()) {
		Ok(e) => return Some(e),
		Err(NewConError::NoPermission) => typer_error!("no permission"),
		Err(NewConError::EstablishCon(e)) => typer_error!("cannot connect, reason '{}'", e),
		Err(NewConError::NoEmptyKeycodes) => typer_error!("no available keycodes"),
		Err(NewConError::Reply) => typer_error!("unexpected reply"),
	}
	None
}

fn handle_input_error(err: InputError) {
	match err {
		InputError::Mapping(k) => typer_error!("cannot map key '{}'", k),
		InputError::Unmapping(k) => typer_error!("cannot unmap key '{}'", k),
		InputError::NoEmptyKeycodes => typer_error!("no available keycodes"),
		InputError::Simulate(r) => typer_error!("something went wrong, reason: {}", r),
		InputError::InvalidInput(w) => typer_error!("unsupported text input: {}", w),
	}
}

pub fn typer(text: &str) {
	if let Some(mut enigo) = create_enigo() {
		if let Err(err) = enigo.text(text) {
			handle_input_error(err);
		}
	}
}

#[allow(dead_code)]
pub fn backspace(count: usize) {
	if let Some(mut enigo) = create_enigo() {
		for _ in 0..count {
			enigo.key(Key::Backspace, Press).ok();
			thread::sleep(Duration::from_secs(2));
			enigo.key(Key::Backspace, Release).ok();
		}
	}
}
