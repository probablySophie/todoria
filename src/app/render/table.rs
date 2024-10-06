use ratatui::{layout::Constraint, widgets::Row};
use todo_txt_rs::Todo;

pub enum Density
{
	Comfy,
	Compact
}

#[derive(Default)]
pub struct Display
{
	description: bool,
	project: bool,
	context: bool,
	date_created: bool,
	date_completed: bool,
	date_due: bool,
	priority: bool,
	completion: bool,
}

pub struct Table
{
	constaints: Vec<Constraint>,
	rows: Vec<Row>,
}

pub fn new(todo_vec: Vec<Todo>)
{
	// TODO: Use a stateful widget for selection & remembering things?
	// https://docs.rs/ratatui/latest/ratatui/widgets/trait.StatefulWidget.html

    // TODO: Make it possible to specify which rows to create
    // create_table(true, false, true, true, false) sort of deal

    // TODO: Maybe we should store the table in the app?
    //       So it's only recreated when there's a change in the filters?
    //       Because recreating it constantly is a bad idea, its SLOW
}
