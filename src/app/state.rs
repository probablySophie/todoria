use std::io;
use ratatui::prelude::Stylize;

use crate::{app:: {keybinds::char_from_action, Action}, DEBUG};

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
impl std::fmt::Display for State
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
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

impl crate::app::App<'_>
{
    
    /// Update the app's `self.state`, also sets `self.previous_state`
    pub fn change_state(&mut self, state: State)
    {
        if DEBUG
        {
            self.debug_info += &(String::from("Changed state to '") + &state.to_string() + "'");
        }
        self.previous_state = self.state;
        self.state = state;

        self.screens.update_instructions(ratatui::text::Line::from(match state
        {
            State::Main => {
                vec![
                    
                    " Quit: ".into(),
                    char_from_action(&self.keybinds, Action::Quit, State::Main).magenta().bold(),
                    " Select: ".into(),
                    char_from_action(&self.keybinds, Action::Select, State::Main).magenta().bold(),
                    " New: ".into(),
                    char_from_action(&self.keybinds, Action::New, State::Main).magenta().bold(),
                    " Save: ".into(),
                    char_from_action(&self.keybinds, Action::Save, State::Main).magenta().bold(),
                    " ".into(), // Keep me last in this vec
                ]            
            },
            State::Focused
            | State::UnsavedChanges
            | State::Settings
            | State::Filter
            | State::Sort
            | State::All => { vec![ "No Instructions Yet :(".red().bold() ] }
        }).fg(ratatui::style::Color::Indexed(165)) );
    }
}
