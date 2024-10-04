use std::io;
use todo_txt_rs as todotxt;

mod app;
mod test;
mod shared;

const TITLE: &str = "TODOria";
const DEBUG: bool = true;

// https://ratatui.rs/tutorials/counter-app/basic-app/#displaying-the-application
// In app.rs


fn main() -> io::Result<()>
{	
	let mut terminal = ratatui::init();
	terminal.clear()?;
	let app_result = app::App::default().run(terminal);
	ratatui::restore();
	if DEBUG { println!("Quit Safely :)") }
	app_result
}
