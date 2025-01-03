use std::io;
use crossterm::event;
use todo_txt_rs::Todo;

use super::keybinds;

#[path = "../tests/app.rs"] mod test    ;
#[path = "render/mod.rs"]   mod render  ;
#[path = "state.rs"]        mod state   ; pub use state::State;
#[path = "action.rs"]       mod action  ; pub use action::Action;
#[path = "settings.rs"]     mod settings;

// TODO: This declaration will probably need to be turned into a Macro so that main.rs can add non-generic items like loaded_todos
#[derive(Debug)]
pub struct App<'a>
{
    title: &'static str,
    exit: bool,
    unsaved_changes: bool,
    
    state: State,                     // [[state.rs]]
    previous_state: State,
    
    keybinds: Vec<keybinds::KeyBind>, // [[keybinds.rs]]
    
    loaded_todos: Vec<Todo>,          // [[todos.rs]]
    
    pub settings: settings::Settings,     // [[settings.rs]]
    pub debug_info: String,               // Printed if crate::DEBUG is true

    instructions: ratatui::text::Line<'a>, // The instructions & keybinds to display

    screens: render::Screens<'a>,
}

impl<'a> Default for App<'a>
{
    fn default() -> Self
    {
        App
        {
            title: crate::TITLE,
            exit: false,
            unsaved_changes: false,
            state: State::Main,
            previous_state: State::Main,
            keybinds: keybinds::default_vec(),
            loaded_todos: Vec::new(),
            settings: settings::Settings::default(),
            debug_info: String::new(),
            instructions: ratatui::text::Line::from(""),
            screens: render::Screens::new(),
        }
    }
}


impl App<'_>
{
    /// Our App's main while loop that draws, handles, events, & quits when done
    pub fn run(&mut self, mut terminal: ratatui::DefaultTerminal) -> io::Result<String>
    {
        self.on_load();
        self.setup();
        self.change_state(State::Main);

        // Main loop!
        loop
        {
            // Should we exit?
            if self.exit
            {
                self.before_quit();
                // Double check we still should exit
                if self.exit { break };
            }
    		terminal.draw(|frame| {self.screens.render(frame, self.state)})?;
    		
            self.handle_events()?;
        }
        
    	Ok (self.debug_info.clone()) // Return Ok(())
    }

    fn setup(&mut self)
    {
        // Load settings using Confy [[settings.rs]]
        if let Ok(loaded_settings) = confy::load("Todoria", None)
        {
            self.settings = loaded_settings;
            if crate::DEBUG {self.debug_info += "Successfully loaded settings\n"};
        }
        else if crate::DEBUG
        {
            self.debug_info += "Failed to load settings\n";
        }
        // Else we'll get the default settings
    }

    /// Handle input events
    fn handle_events(&mut self) -> io::Result<()>
    {
        // WARN: event::read() blocks until an event happens.  If we want 
        // to do other things at the same time we should use event::poll() 
        // https://docs.rs/crossterm/latest/crossterm/event/fn.poll.html
    	if let event::Event::Key(key) = event::read()?
    	{
    	    // Did we press a key?
            if key.kind == event::KeyEventKind::Press
        	{
        	    self.key_pressed(keybinds::safe_uppercase(key.code));
        	}
        }
        Ok(()) // Return Ok(())
    }

    fn key_pressed(&mut self, keycode: crossterm::event::KeyCode)// -> io::Result<()>
    {
        // Get the first matching `self.keybinds` action
        let current_action = keybinds::get_action(&self.keybinds, keycode, self.state);

        // Do the thing!
        self.do_the_thing(current_action);
    }

    /// Do whatever action we've got
    fn do_the_thing(&mut self, action: Action)
    {
        match (action, self.state)
        {
            (Action::None,   _) => { /* Do nothing :) */ },
            (Action::Quit,   _) => self.exit(),
            (Action::Close,  _) => self.change_state(State::Main),
            (Action::Save,   _) => self.save(),
            (Action::Select | Action::New, _) 
                                => self.change_state(State::Focused),
            (Action::Up,
                   State::Main) => self.screens.todo_table.up(),
            (Action::Down,
                   State::Main) => self.screens.todo_table.down(),
            _ => { todo!() },
        }
    }

    pub fn exit(&mut self)
    {
        self.exit = true
    }

    /// Attempt to save all our loaded todo items
    fn save(&mut self)
    {
        // Are we saving everything seperately?
        let success:bool = if self.settings.save_seperate_by_project
            {
                crate::app::todos::save_seperate(
                    &self.settings.save_path, 
                    &self.loaded_todos
                )
            }
            else
            {
                crate::app::todos::save(
                    &self.settings.save_path, 
                    &self.loaded_todos
                )
            };

        if success
        {
            self.unsaved_changes = false;
        }

        if crate::DEBUG
        {
            if success
            {
                self.debug_info += "Successfully saved todo items\n";
            }
            else
            {
                self.debug_info += "Failed to save todo items\n";
            }
        }
    }
}

