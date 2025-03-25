use enigo::{Enigo, Keyboard, Settings};

pub fn type_text(text: &str) {
	let mut enigo = match Enigo::new(&Settings::default()) {
		Ok(enigo) => enigo,
		Err(_error) => {
			eprintln!("Hmm...something seems to have gone wrong.");
			return;
		}
	};

	match enigo.text(text) {
		Ok(_) => {}
		Err(_error) => {
			eprintln!("Hmm...something seems to have gone wrong.");
		}
	}
}
