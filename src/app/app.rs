use std::io;
use crossterm::event::{self};

use super::keybinds;

#[path = "../tests/app.rs"] mod test;
#[path = "render.rs"] mod render;

#[path = "state.rs"] mod state; pub use state::State;

#[path = "action.rs"] mod action; pub use action::Action;


#[derive(Debug)]
pub struct App
{
    title: &'static str,
    exit: bool,
    unsaved_changes: bool,
    state: State,
    previous_state: State,
    keybinds: Vec<keybinds::KeyBind>
    // TODO: current_todo: Vec<Todo>
    // TODO: all_todo_items: Vec<Todo>
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
        }
    }
}


impl App
{
    /// Our App's main while loop that draws, handles, events, & quits when done
    pub fn run(&mut self, mut terminal: ratatui::DefaultTerminal) -> io::Result<()>
    {
    	while !self.exit
    	{
    		terminal.draw(|frame| {self.draw(frame)})?;
            self.handle_events()?;
    	}
    	Ok (()) // Return Ok(())
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

    fn exit(&mut self)
    {
        // Don't quit if there's unsaved changes
        if self.unsaved_changes
        {
            self.state = State::UnsavedChanges;
        }
        else
        {
            self.exit = true;
        }
    }

    fn change_state(&mut self, state: State)
    {
        self.previous_state = self.state;
        self.state = state;
        // TODO: The drawn instructions string should be constructed here
        //       Using the actions we want to always display, and the matching
        //       KeyBind's KeyCodes
    }
}

