use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action
{
    None, // Required (for keybinds::KeyBind::matches)
    Quit,
    
    Up,   // For moving around in menus & stuff
    Down,
    Left,
    Right,

    Select, // Select the currently hovered item
    Close,  // Close any popups or go back to the main display
    Save,
}


// TODO: There 100% has to be a way of creating the enum & the from_string in a macro
//       by giving the macro (option, "STRING")(option, "STRING")
impl Action
{
	pub fn from_string(string: &str) -> io::Result<Self>
	{
		match string.to_uppercase().as_str()
		{
			"NONE"   => Ok(Action::None),
			"QUIT"   => Ok(Action::Quit),
			"UP"     => Ok(Action::Up),
			"DOWN"   => Ok(Action::Down),
			"LEFT"   => Ok(Action::Left),

			"RIGHT"  => Ok(Action::Right),
			"SELECT" => Ok(Action::Select),
			"CLOSE"  => Ok(Action::Close),
			_ => Err(
				std::io::Error::new (
					std::io::ErrorKind::NotFound,
					"Unable to match loaded string to enum values"
				)
			)
		}
	}
}
