use std::io;
use ratatui::prelude::Stylize;

use crate::app:: {keybinds::char_from_action, Action};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum State
{
    Main,
    Focused,
    UnsavedChanges,
    Settings,
    Filter,
    Sort,
    All, // Required (for keybinds::KeyBind::matches) [[keybinds.rs]]
}
impl State
{
	/// Make a new `State` enum item from a given string
    pub fn from_string(string: &str) -> io::Result<Self>
    {
        match string.to_uppercase().as_str()
        {
            "MAIN"     => Ok(State::Main),
            "FOCUSED"  => Ok(State::Focused),
            "SETTINGS" => Ok(State::Settings),
            "FILTER"   => Ok(State::Filter),
            "SORT"     => Ok(State::Sort),
            "ALL"      => Ok(State::All),
            _ => Err(
                std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Unable to match loaded string with enum values"
                )
            ),
            
        }
    }
}

impl crate::app::App
{
    
    /// Update the app's `self.state`, also sets `self.previous_state`
    pub fn change_state(&mut self, state: State)
    {
        self.previous_state = self.state;
        self.state = state;

        self.instructions = ratatui::text::Line::from(match state
        {
            State::Main => {
                vec![
                    
                    " Quit: ".into(), 
                    char_from_action(&self.keybinds, Action::Quit, State::Main).blue().bold(),
                    " Select: ".into(),
                    char_from_action(&self.keybinds, Action::Select, State::Main).blue().bold(),
                    " ".into(),
                ]            
            },
            
            State::Focused
            | State::UnsavedChanges
            | State::Settings
            | State::Filter
            | State::Sort
            | State::All => { vec![ "No Instructions Yet :(".red().bold() ] }
        });        
    }
}
