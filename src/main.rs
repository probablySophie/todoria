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
