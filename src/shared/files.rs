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

/// Read all files in a given directory into a single `Vec<String>`
///
/// # Errors
///
/// - Will return an error if unable to read the given directory path
pub fn read_all(directory_path: &str) -> io::Result<Vec<String>>
{
	let directory_path = safe_directory(directory_path);

	// Attempt to read the directory path we were given
	// If this fails then the function has failed anyway, so we may as well return Err
	let read_dir = fs::read_dir(directory_path)?;

	// If we've gotten here then hopefully we'll be able to return something :)
	let mut return_vec = Vec::new();
	// For each directory entry in read_dir
	for dir_entry in read_dir.flatten()
	{
		// If we were able to successfully read the contents of that item
		if let Ok(contents) = read(&dir_entry.path().display().to_string())
		{
			// Add the file contents to our Vec<String>!
			return_vec.push(contents);
		}
	}

	Ok(return_vec)
}

/// Attempts to write the given `&str` to the given file path
/// Returns a `bool` containing `true` if successful, and `false` if not
pub fn write(file_path: &str, contents: &str) -> bool
{
	let result = fs::write(file_path, contents);

	result.is_err()
}



pub fn recursive_mkdir(file_path: &str) -> bool
{
	// TODO: recursive_mkdir
	false
}

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
