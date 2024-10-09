use ratatui::{buffer::Buffer, layout::{Constraint, Rect}, style::{Style, Stylize}, widgets::{Block, Cell, Row, Table, TableState, Widget}, Frame};
use todo_txt_rs::Todo;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Density
{
	Comfy,
	Compact
}

#[derive(Debug)]
pub struct Column
{
	name: String,
	show: bool,
	width: Constraint,
	order: i8,
	// TODO: Per column styling
}
impl Column
{
	fn new(name: &str, show: bool, width: Constraint, order: i8) -> Self
	{
		Self
		{
			name: name.to_owned(),
			show,
			width,
			order
		}
	}
}

#[derive(Debug)]
pub struct Display
{
	pub columns:  Vec<Column>,
	pub density:  Density,
	pub selected: usize,
}

impl Default for Display
{
	fn default() -> Self
	{
        Display
        {   // Name: &str, show: bool, width: i8, order: i8
        	columns: vec![
        		Column::new("Completion" , true , Constraint::Max(1), 0),
        		Column::new("Priority"   , true , Constraint::Max(1) , 1),
        		Column::new("Description", true , Constraint::Min(15), 2),
        		Column::new("Due"        , true , Constraint::Max(10), 3),
        		Column::new("Project"    , false, Constraint::Max(10), 4),
        		Column::new("Context"    , false, Constraint::Min(10), 5),
        		Column::new("Created"    , false, Constraint::Max(10), 6),
        		Column::new("Completed"  , false, Constraint::Max(10), 7)],
        	density:        Density::Comfy,
        	selected:       0,
        	
        }
    }
}
impl Display
{
	fn widths(&self) -> Vec<Constraint>
	{
		self.columns.iter()
			.filter_map(|row|
				if row.show { Some(row.width) }
				else { None }
			).collect::<Vec<Constraint>>()
	}
}

#[derive(Debug, Clone)]
struct TodoRow
{
	index: usize,
	values: Vec<String>,
}

#[allow(clippy::struct_field_names, clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct TableContainer<'a>
{
	table: ratatui::widgets::Table<'a>,
	table_state: ratatui::widgets::TableState,
	
	rows: Vec<TodoRow>,
	
	display: Display,
}


impl<'a> TableContainer<'a>
{
	pub fn down(&mut self)
	{
		if self.display.selected < self.rows.len()
		{
			self.display.selected += 1;
		}
		*self.table_state.selected_mut() = Some(self.display.selected);
	}
	pub fn up(&mut self)
	{
		if self.display.selected > 0
		{
			self.display.selected -= 1;
		}
		*self.table_state.selected_mut() = Some(self.display.selected);
	}
	
	pub fn new() -> Self
	{
		let display = Display::default();
		let table_state = TableState::new()
			.with_selected(Some(display.selected));

		let mut new = Self
		{
			table: Table::new(Vec::<Row>::new(), display.widths()),
			table_state,
			rows: Vec::new(),
			display,
		};

		new.refresh();

		new
	}

	fn as_table_rows(&mut self) -> Vec<Row<'a>>
	{
		let mut row_vec = Vec::new();

		for row in &self.rows
		{
			let mut new_row: Vec<Cell> = Vec::new();
			for (i, item) in row.values.iter().enumerate()
			{
				// We're assuming here that the column order & value order are the same
				// So skip if we're not showing the column :)
				if self.display.columns[i].show
				{
					// TODO: This doesn't respect self.display.columns[i].order.  Fix that pls
					//       We can almost certainly do the same thing as build_rows
					new_row.push( Cell::new(item.to_owned()) );
				}
			}

			row_vec.push(Row::new(new_row));
		}


		row_vec // Return a Vec<Row<'a>>
	}

	pub fn build_rows(&mut self, todo_vec: Vec<Todo>)
	{
		self.rows = Vec::new();

		//let mut todo_index = 0;
		#[allow(clippy::explicit_counter_loop)]
		// We don't really want to clone todo_vec as it could be really quite large
		// And doing .into_iter
		// TODO: Check if using .iter().enumerate() clones the Vec<Todo> or whether
		//       It just provides an iter of references
		for (todo_index, todo) in todo_vec.iter().enumerate()
		{
			let mut cells: Vec<(String, i8)> = Vec::new();
			for display in &self.display.columns
			{
				match display.name.as_ref()
				{
					"Completion" => {cells.push(
						((if todo.complete {"x"} else {" "}).to_string(),
						display.order)
					);},
					"Priority"   => {cells.push(
						(todo.priority.unwrap_or(' ').to_string(),
						display.order)
					);},
					"Completed"  => {cells.push(
						(if todo.completion_date.is_some() {todo.completion_date.unwrap().to_string()} else {String::new()},
						display.order)
					);},
					"Created" => {cells.push(
						(if todo.creation_date.is_some() {todo.creation_date.unwrap().to_string()} else {String::new()},
						display.order)
					);},
					"Description" => {cells.push(
						(todo.description.clone(),
						display.order)
					);},
					/*"Project" => {cells.push( TODO: this
						(Cell::new(todo),
						display.order)
					);},*/
					//"Context" => {cells.push();}, TODO: Thus
					"Due" => {cells.push(
						(todo.tags.tag_value("due").unwrap_or(String::new())
						, display.order)
					);},
					_ => {},
				}
			}

			// Sort the items by their order
			cells.sort_by(|(_, a), (_, b)| a.cmp(b) );

			self.rows.push( TodoRow {
				index: todo_index,
				values: cells.into_iter().map(|( c ,_)| c ).collect::<Vec<String>>(),
			} );
			//todo_index += 1;
		}
		self.refresh();
	}

	pub fn refresh(&mut self)
	{
		
		self.table = Table::new(self.as_table_rows(), self.display.widths())
				.column_spacing(3) // TODO: Have this number depend on self.display.density
				.style(Style::new())
				.highlight_style(Style::new().reversed().bold());
	}

	/// Render the table!
	pub fn render(&mut self, frame: &mut Frame, block: Block)
	{
		frame.render_stateful_widget(&self.table.clone().block(block), frame.area(), &mut self.table_state);
		// widgets::Table doesn't implement Copy, so de-referencing it
		// ... is the best solution I can come up with right now
	}
}
