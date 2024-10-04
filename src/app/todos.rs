use todo_txt_rs::Todo;
use std::io;
use crate::shared::files;

pub fn load(file_path: &str) -> io::Result<Vec<Todo>>
{
	if let Ok(contents) = files::read(file_path)
	{
		return Ok( todo_txt_rs::bulk_create(&contents) )
	}
	// Else try appending "/todo.txt"
	if let Ok(contents) = files::read( &(files::safe_directory(file_path) + "todo.txt") )
	{
		return Ok( todo_txt_rs::bulk_create(&contents) )
	}
	// Else else

	Err(
		io::Error::new(
			io::ErrorKind::InvalidData, 
			"Failed to read any data from given file_path or file_path/todo.txt"
		)
	)
}

pub fn load_all(file_path: &str)// -> Vec<Todo>
{
	// TODO: 1. Get all .txt files in the given dir
	//       2. For each, call load()
	//       3. Combine the responses
	
}

pub fn save(file_path: &str, todo_vec: Vec<Todo>) -> bool
{
	let string = todo_txt_rs::flatten_vec(todo_vec);

	// TODO: Save the string to the file_path

	false
}

pub fn save_seperate(file_path: &str, todo_vec: Vec<Todo>) -> bool
{
	let mut success = true;
	let projects = todo_txt_rs::info::get_projects(&todo_vec);

	for project in projects
	{
		// TODO: 1. Filter the todo_vec by that project
		//       2. this_success = save(file_path + "/" + project + ".txt", filtered)
		//       3. success = success && this_success // false if either value is false
	}

	success
}

// TODO: save(file_path) -> bool


