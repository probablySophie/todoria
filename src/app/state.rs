use std::io;

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
