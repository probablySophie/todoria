use std::io;

#[macro_use]
mod app;
mod test;
mod shared;

const TITLE: &str = "TODOria";
const DEBUG: bool = true;

// https://ratatui.rs/tutorials/counter-app/basic-app/#displaying-the-application
// In app.rs

// Variable name, variable type, default value
// AppVariables!
// (
//    loaded_todos, Vec<Todo>, Vec::new(),
// )

// // Default Actions like None are added by the app themselves
// Name of the action, required state to happen, thing to do
// AppAction!
// (
//     Save,    _   ,  self.safe()
//     Up,      Main,  self.screens.todo_table.up()
//     Down,    Main,  self.screens.todo_table.down()
//     Select,  Main,  self.change_state(focused)
//     New,     Main,  self.change_state(focused)
// )

// AppState!
// (
//     Main,            Which screen to draw
//     Focused,         
//     UnsavedChanges,  
//     Settings,        
//     Filter,          
//     Sort,            
// )


// AppScreens!
// {
//     screenName, struct
// }

use crossterm::event::KeyCode;
use app::{Action, State, keybinds::KeyBind};

DefaultKeybinds!
(
	// KeyCode            Action to call   States this is valid in
	KeyCode::Char('Q'),   Action::Quit,    State::All,
	KeyCode::Up,          Action::Up,      State::All,
    KeyCode::Down,        Action::Down,    State::All,
    KeyCode::Left,        Action::Left,    State::All,
    KeyCode::Right,       Action::Right,   State::All,
    KeyCode::Enter,       Action::Select,  State::All,
    KeyCode::Esc,         Action::Close,   State::All,
    KeyCode::Char('N'),   Action::New,     State::Main,
    KeyCode::Char('S'),   Action::Save,    State::Main,
);

impl app::App<'_>
{
	fn on_load(&mut self)
	{
		// code called on load
        // If we're saving multiple files
        if self.settings.save_seperate_by_project
        {
            // Load multiple files
            self.loaded_todos = crate::app::todos::load_all(&self.settings.save_path);
        }
        else // Else
        {
            // Just load one file thank you
            if let Ok(loaded) = crate::app::todos::load(&self.settings.save_path)
            {
                self.loaded_todos = loaded;
            }
        }
        if ! self.loaded_todos.is_empty()
        {
            self.screens.todo_table.build_rows(self.loaded_todos.clone());
        }

        if crate::DEBUG
        {
            self.debug_info += "loaded '";
            self.debug_info += &(self.loaded_todos.len().to_string() + "' Todo items");
        }
	}
	
	fn before_quit(&mut self)
	{
		// code called if self.exit is True.  self.exit can be set to False to cancel quitting
	}

	fn handle_action(&mut self, action: Action, state: State)
	{
		match (action, state) {
			/* Default Actions */
		    (Action::None, _) => { /* Do nothing */ },
		    (Action::Quit, _) => { self.exit() },
		    /* Custom Actions */
			(Action::Up, _) => todo!(),
			(Action::Down, _) => todo!(),
		    (Action::Left, _) => todo!(),
		    (Action::Right, _) => todo!(),
		    (Action::Select, _) => todo!(),
		    (Action::Close, _) => todo!(),
		    (Action::Save, _) => todo!(),
		    (Action::New, _) => todo!(),
		}
	}
}

fn main() -> io::Result<()>
{	
	let mut terminal = ratatui::init();
	
	terminal.clear()?;
	
	let app_result = app::App::default().run(terminal);
	
	ratatui::restore();

	// If DEBUG, print our debug string :)
	if DEBUG
	{
		if let Ok(result) = &app_result
		{
			println!("Quit Safely :)");
			println!("Debug info: \n\x1b[93m{}\x1b[0m", result.clone());
		}
		else
		{
			println!("Did not exit safely!!!");
		}
	}

	// And return an io::Result<()>
	if app_result.is_ok()
	{
		Ok(())
	}
	else
	{
		Err(
			io::Error::new(
				io::ErrorKind::Other,
				"App did not exit successfully"
			)
		)
	}
}
