use std::{fs, io};

pub fn read(file_path: &str) -> io::Result<String>
{
	// Attempt to read from the file path
	if let Ok(contents) = fs::read_to_string(file_path)
	{
		// We were successful!
		return Ok(contents)
	}
	// Else
	Err(
		io::Error::new(
			io::ErrorKind::InvalidData, 
			"Failed to read any data from given file_path or file_path/todo.txt"
		)
	)
}

// TODO: pub fn read_all(directory_path: &str) -> io::Result<Vec<String>>
//           let directory_path = safe_directory(directory_path);

pub fn safe_directory(directory_path: &str) -> String
{
	let folder_seperator_char = if cfg!(windows) {"\\"} else {"/"};
	
	if directory_path.ends_with(folder_seperator_char)
	{
		// We're happy
		return directory_path.to_string()
	}
	
	// Else
	directory_path.to_string() + folder_seperator_char
}
