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

pub fn load_all(file_path: &str) -> Vec<Todo>
{
	let mut vec_todo = Vec::new();
	
	if let Ok(contents_vec) = files::read_all(file_path)
	{
		for contents in contents_vec
		{
			vec_todo.append( &mut todo_txt_rs::bulk_create(&contents) );
		}
	}
	
	vec_todo
}

pub fn save(file_path: &str, todo_vec: Vec<Todo>) -> bool
{
	//TODO: Call files::recursive_mkdir();

	files::write(file_path, &todo_txt_rs::flatten_vec(todo_vec))
}

pub fn save_seperate(directory_path: &str, todo_vec: Vec<Todo>) -> bool
{
	//TODO: Call files::recursive_mkdir();
	let directory_path = files::safe_directory(directory_path);
	
	let mut success = true;
	let projects = todo_txt_rs::info::get_projects(&todo_vec);

	for project in projects
	{
		let save_vec = todo_txt_rs::filter::project(&todo_vec, &project);

		// Set success to false if:
		//     Success was already false
		//     We failed to save this set of Todo items
		success = success && files::write(
			&( (directory_path.clone() + &project) + ".txt"),
			&todo_txt_rs::flatten_vec(save_vec)
		);
	}

	success
}

