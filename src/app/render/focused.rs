use todo_txt_rs::Todo;

/*
	For when a [[table.rs]] item is selected
*/

// TODO: This will need to know if its a focus focus, or a new focus
pub struct Focused
{
	task: Todo,
	
}
impl Focused
{
	pub fn new(task: Todo) -> Self
	{
		Self
		{
			task
		}
	}
	// TODO: pub fn new_task() -> Self ???
}


/*
	TODO: Display the todo.description as a paragraph
	https://docs.rs/ratatui/latest/ratatui/widgets/struct.Paragraph.html


	TODO: Display any dependant todo items as a list
	      & a guage of completion

	
	Gauge | https://ratatui.rs/examples/widgets/gauge/
	List  | https://ratatui.rs/examples/widgets/list/

	Make a button widget | https://ratatui.rs/examples/widgets/custom_widget/
*/

/*
	Layout==========================================================================|
	| Description                                             | Projects | Contexts |
	|                                                         |          |          |
	| Paragraph                                               |scrollable|scrollable|
	|                                                         |   list   |   list   |
	|===============================================================================|
	| Before this task ↓              |======   20%         | |    TAGS             |
	| TASK1 DESCRIPTION ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~  | Name: value         |
	| TASK2 DESCRIPTION ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~  | Name: value         |
	|                                                         |                     |
	| Depends on this task ↓                                  |                     |
	| TASK3 DESCRIPTION                                       |                     |
	| TASK4 DESCRIPTION                                       | + Add Tag           |
	|===============================================================================|
*/
