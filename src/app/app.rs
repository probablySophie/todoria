use std::io;
use crossterm::event::{self};
use todo_txt_rs::Todo;

use super::keybinds;

#[path = "../tests/app.rs"] mod test    ;
#[path = "render.rs"]       mod render  ;
#[path = "state.rs"]        mod state   ; pub use state::State;
#[path = "action.rs"]       mod action  ; pub use action::Action;
#[path = "settings.rs"]     mod settings;


#[derive(Debug)]
pub struct App
{
    title: &'static str,
    exit: bool,
    unsaved_changes: bool,
    state: State,                     // [[state.rs]]
    previous_state: State,
    keybinds: Vec<keybinds::KeyBind>, // [[keybinds.rs]]
    current_view: Vec<Todo>,
    loaded_todos: Vec<Todo>,          // [[todos.rs]]
    settings: settings::Settings,     // [[settings.rs]]
}

impl Default for App
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
            current_view: Vec::new(),
            loaded_todos: Vec::new(),
            settings: settings::Settings::default(),
        }
    }
}


impl App
{
    /// Our App's main while loop that draws, handles, events, & quits when done
    pub fn run(&mut self, mut terminal: ratatui::DefaultTerminal) -> io::Result<()>
    {
        self.setup();
                
    	while !self.exit
    	{
    		terminal.draw(|frame| {self.draw(frame)})?;
            self.handle_events()?;
    	}
    	Ok (()) // Return Ok(())
    }

    fn setup(&mut self)
    {        
        // TODO: Load settings  [[settings.rs]]
        
        // If we're saving multiple files
        if self.settings.save_seperate_by_project
        {
            // Load multiple files
            self.loaded_todos = crate::app::todos::load_all(&self.settings.save_path);
        }
        else // Else
        {
            // Just load one file thank you
            self.loaded_todos = crate::app::todos::load(&self.settings.save_path);
        }
    }

    fn draw(&self, frame: &mut ratatui::Frame)
    {
        // TODO: match self.state
        frame.render_widget(self, frame.area());
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

    fn do_the_thing(&mut self, action: Action)
    {
        match action
        {
            Action::None => { /* Do nothing :) */ },
            Action::Quit => self.exit(),
            Action::Close => self.change_state(State::Main),
            _ => { todo!() },
        }
    }

    /// Either set `self.exit` to true or change state to `State::UnsavedChanges`
    /// Depending on whether or not there are unsaved changes
    fn exit(&mut self)
    {
        // Don't quit if there's unsaved changes
        if self.unsaved_changes
        {
            self.change_state(State::UnsavedChanges);
        }
        else
        {
            self.exit = true;
        }
    }

    /// Update the app's `self.state`, also sets `self.previous_state`
    fn change_state(&mut self, state: State)
    {
        self.previous_state = self.state;
        self.state = state;
        
        // TODO: The drawn instructions string should be constructed here
        //       Using the actions we want to always display, and the matching
        //       KeyBind's KeyCodes
    }
}

