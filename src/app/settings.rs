// TODO: 
//use serde_derive::{Serialize, Deserialize};

// Use Confy? https://docs.rs/confy/latest/confy/index.html ??
// Confy ONLY does configs, not saving anything else

#[derive(Debug)] // Serialize, Deserialize)]
pub struct Settings
{
	//TODO: Keybinds
	pub save_path: String,
	pub save_seperate_by_project: bool
	
}

impl Default for Settings
{
	fn default() -> Self
	{
        Settings
        {
        	save_path: String::from(""), // dirs::data_dir() + "/sophie.coffee/todo/"
        	save_seperate_by_project: false,
        }
    }
}

// TODO: Make keybinds from loaded settings
