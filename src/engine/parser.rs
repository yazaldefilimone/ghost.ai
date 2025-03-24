use rdev::Key;

use super::state::KeyboardState;

pub fn parse_key(key: Key, state: &KeyboardState) -> Option<char> {
	let shifted = state.is_uppercase();
	use Key::*;
	#[rustfmt::skip]
    let c = match key {
      //
      KeyA => 'a', KeyB => 'b', KeyC => 'c', KeyD => 'd', KeyE => 'e',
      KeyF => 'f', KeyG => 'g', KeyH => 'h', KeyI => 'i', KeyJ => 'j',
      KeyK => 'k', KeyL => 'l', KeyM => 'm', KeyN => 'n', KeyO => 'o',
      KeyP => 'p', KeyQ => 'q', KeyR => 'r', KeyS => 's', KeyT => 't',
      KeyU => 'u', KeyV => 'v', KeyW => 'w', KeyX => 'x', KeyY => 'y', KeyZ => 'z',

      //
      Num1 => if state.shift { '!' } else { '1' },
      Num2 => if state.shift { '@' } else { '2' },
      Num3 => if state.shift { '#' } else { '3' },
      Num4 => if state.shift { '$' } else { '4' },
      Num5 => if state.shift { '%' } else { '5' },
      Num6 => if state.shift { '^' } else { '6' },
      Num7 => if state.shift { '&' } else { '7' },
      Num8 => if state.shift { '*' } else { '8' },
      Num9 => if state.shift { '(' } else { '9' },
      Num0 => if state.shift { ')' } else { '0' },

      //
      Minus => if state.shift { '_' } else { '-' },
      Equal => if state.shift { '+' } else { '=' },
      LeftBracket => if state.shift { '{' } else { '[' },
      RightBracket => if state.shift { '}' } else { ']' },
      BackSlash => if state.shift { '|' } else { '\\' },
      SemiColon => if state.shift { ':' } else { ';' },
      Quote => if state.shift { '"' } else { '\'' },
      Comma => if state.shift { '<' } else { ',' },
      Dot => if state.shift { '>' } else { '.' },
      Slash => if state.shift { '?' } else { '/' },
      BackQuote => if state.shift { '~' } else { '`' },


      Space => ' ',

      _ => return None,
    };

	Some(if shifted { c.to_ascii_uppercase() } else { c })
}
