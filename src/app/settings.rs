use serde::{Serialize, Deserialize};


// Use Confy? https://docs.rs/confy/latest/confy/index.html ??
// Confy ONLY does configs, not saving anything else

#[derive(Debug, Serialize, Deserialize)]
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
		let mut settings = Settings
	    {
	    	save_path: String::new(),
	    	save_seperate_by_project: false,
	    };
				
		if let Some(data_dir) = dirs::data_dir()
		{
			if let Some(path) = data_dir.as_path().to_str()
			{
				settings.save_path = crate::shared::files::safe_directory(path) + "sophie.coffee/todoria/";
			}
	    }

	    settings // Return our settings
    }
}

// TODO: Make keybinds from loaded settings
