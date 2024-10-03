use crossterm::event::KeyCode;

use super::app::{Action, State};

// TODO: tests

#[derive(Debug)]
pub struct KeyBind
{
    // The key - this one changes
    key: crossterm::event::KeyCode,
    // The action that gets called, this one doesn't change
    action: Action,
    // The states that the KeyBind applies to
    states: Vec<State>,
}
impl KeyBind
{
	/// Check if the current `KeyBind` matches a given `KeyCode` and `app::State`
	pub fn matches(&self, keycode: crossterm::event::KeyCode, state: State) -> bool
	{
		// Check if the keycode matches first, because that's faster then checking the state
		if self.key != keycode
		{
			return false
		}
		// Else
		
		// For each of the current KeyBind's states
		for s in &self.states
		{
			// If the state matches the input state
			// Or the state is ALL (you know.  Matching all of them)
			if s == &state || s == &State::All
			{
				// Then we're happy :)
				return true
			}
		}
		// Else we don't match
		false // Return false
	}

	/// Make a single new keybind
	pub fn new(key: crossterm::event::KeyCode, action: Action, states: Vec<State>) -> Self
	{
		KeyBind
		{
			key,
			action,
			states
		}
	}

	/// Update a keybind
	pub fn update(&mut self, keycode: crossterm::event::KeyCode)
	{		
		self.key = keycode;
	}

	pub fn from(state: &str, action: &str, keycode: &str) -> Self
	{
		// TODO: make this return io::Result<Self>
		// TODO: Make this safe, so it doesn't blindly .unwrap()
		// TODO: Testing.  For the love of god.  Testing
		KeyBind
		{
			key:    crossterm::event::KeyCode::Null, // TODO: this
			action: Action::from_string(action).unwrap(),
			states: vec![State::from_string(state).unwrap()],
		}
		
		// TODO: Make a keybind from the state, action, and keycode
	}
}


/// Takes a `Vec<Keybind>`, an active `KeyCode`, and the current `app::State`
/// Returns the **first** matching `Keybind`'s `app::Action` as a `Result<>`
pub fn get_action(keybinds: &Vec<KeyBind>, keycode: crossterm::event::KeyCode, current_state: State) -> Action //io::Result<Action>
{
	// For each possible keybind
	for keybind in keybinds
	{
		// If the keybind matches the input KeyCode and app::State
		if keybind.matches(keycode, current_state)
		{
			// Yay
			return keybind.action//return Ok(keybind.action)
		}
	}
	// Else do nothing
	Action::None //Ok(Action::None)
}


pub fn default_vec() -> Vec<KeyBind>
{
	// TODO: Make it possible to overwrite some of these keybinds with
	//       Other keybinds loaded from a text file or something
	vec![
        KeyBind::new( // Make sure we can quit
            crossterm::event::KeyCode::Char('Q'),
            Action::Quit,
            vec![State::All],
        ),
        KeyBind::new(
        	crossterm::event::KeyCode::Up,
        	Action::Up,
        	vec![State::All],
        ),
        KeyBind::new(
        	crossterm::event::KeyCode::Down,
        	Action::Down,
        	vec![State::All],
        ),
        KeyBind::new(
        	crossterm::event::KeyCode::Left,
        	Action::Left,
        	vec![State::All],
        ),
        KeyBind::new(
        	crossterm::event::KeyCode::Right,
        	Action::Right,
        	vec![State::All],
        ),
        KeyBind::new(
        	crossterm::event::KeyCode::Enter,
        	Action::Select,
        	vec![State::All],
        ),
        KeyBind::new(
        	crossterm::event::KeyCode::Esc,
        	Action::Close,
        	vec![State::All],
        )
    ]
}

/// Take a `CrossTerm[...]KeyCode` and return either the same `KeyCode` or an uppercase version of it.
pub fn safe_uppercase(key: KeyCode) -> KeyCode
{	
	// First check if we're dealing with a KeyCode::Char(_)
	if let KeyCode::Char(key_char) = key
	{
		// If we can safely uppercase the char
		if let Some(key_char_uppercase) = 
			key_char.to_uppercase().collect::<Vec<char>>().first()
		{
			// Return an uppercase version of what we were given
			KeyCode::Char(key_char_uppercase.to_owned())
		}
		else
		{
			// We aren't dealing with a cleanly uppercase-able char
			key // Just return they KeyCode we got in the first place
		}
	}
	else
	{
		// We aren't dealing with a KeyCode::Char(_)
		key // Just return the KeyCode we got in the first place
	}
}
