use ratatui::{style::Stylize, text::Line, widgets::{self, block::Title, Block}, Frame};

use crate::app::State;

use super::{hello_world::HelloWorld, table::TableContainer};

#[derive(Debug)]
pub struct Screens<'a>
{
    // The block, so it can update
    pub block: Block<'a>,
    
    pub todo_table: TableContainer<'a>,
    pub hello_world: HelloWorld<'a>,
}
impl<'a> Screens<'a>
{
    pub fn new(title: &str) -> Self
    {
        let title = widgets::block::Title::from(title.to_owned().bold());
                
        let block = Self::block( Title::from("") );        
        Screens
        {
            block,
            todo_table: TableContainer::new(),
            hello_world: HelloWorld::new("Hello World!"),
        }
    }

    fn block(instructions: Title) -> Block
    {
        Block::bordered()
            .title(
                Title::from(crate::TITLE.to_owned().bold())
            )
            .title(instructions
                .alignment(ratatui::layout::Alignment::Center)
                .position(widgets::block::Position::Bottom)
            )
            .border_set(ratatui::symbols::border::ROUNDED)
            //.style(Style::new().blue())
    }

    pub fn update_instructions(&mut self, instructions: Line<'a>)
    {
        self.block = Self::block(instructions.into());
    }

    pub fn render(&mut self, frame: &mut Frame, state: crate::app::State)
    {
        let block = self.block.clone();
        match state
        {
            State::Main           => self.todo_table.render(frame, block),
            State::Focused        => todo!(),
            State::UnsavedChanges => todo!(),
            State::Filter         => todo!(),
            State::Sort           => todo!(),
            State::Settings       => todo!(),
            State::All            => self.hello_world.render(frame, block), 
            // State::All should eventually be unreachable
        }
    }
}

/*
impl App
{
    pub fn draw(&self, frame: &mut Frame)
    {
        // TODO: Update all the below draw functions to actually take a Frame
        //       And call frame.render_widget or frame.render_stateful_widget
        //       with the widget that they're for e.g.
        //       frame.render_stateful_widget(table, frame.area(), &mut table_state)
        frame.render_widget(self, frame.area());
    }
    
    fn draw_main(&self, area: Rect, buf: &mut Buffer, block: Block)
    {
        // A table  | https://docs.rs/ratatui/latest/ratatui/widgets/struct.Table.html
        // Of rows  | https://docs.rs/ratatui/latest/ratatui/widgets/struct.Row.html
        // And cells| https://docs.rs/ratatui/latest/ratatui/widgets/struct.Cell.html

        let mut rows: Vec<widgets::Row> = Vec::new();

        for todo in &self.loaded_todos
        {
            rows.push( widgets::Row::new( vec![
                        widgets::Cell::from(todo.description.clone()),
                        //widgets::Cell::from(todo.tags.pr)
            ] ) );
        }
        
        let widths = vec![
            Constraint::Min(10),
            Constraint::Min(5),
        ];
        
        widgets::Table::new(rows, widths)
            .column_spacing(1)
            .style(Style::new().blue())
            .highlight_style(Style::new().reversed())
            .block(block)
            .render(area, buf);
        // TODO: Table with currently loaded todo items

        // A table for viewing the Todo items
        // https://ratatui.rs/showcase/widgets/#table

        // A scrollbar for going up/down
        // https://ratatui.rs/examples/widgets/scrollbar

        // A guage of total tasks complete?
        // https://ratatui.rs/examples/widgets/gauge
    }

    fn draw_unsaved_changes(&self, area: Rect, buf: &mut Buffer, block: Block)
    {
        // TODO: A warning screen that says *unsaved changes ahead*
        //       Do you want to save?
        //           _Yes    _No     

        // Render self.previous_state first?
        // Then draw a little custom popup widget on top?
    }
    
    fn draw_focused(&self, area: Rect, buf: &mut Buffer, block: Block)
    {
        // TODO: A single todo item, its info, & editing

        // A guage for subclass completion (if there are any)
        // https://ratatui.rs/showcase/widgets/#gauge

        // A paragraph for drawing the actual Todo item
        // https://ratatui.rs/examples/widgets/paragraph

        // A scrollbar for if there's too much going on
        // https://ratatui.rs/examples/widgets/scrollbar

        // Lists for the projects & contexts
        // https://ratatui.rs/examples/widgets/list

        // TODO: And/or a single setting/filter/sortable?
        //       Based on previous_state?
        //       And keep drawing previous_state underneed the current popup?
        //       Can we darken what's behind?
    }

    fn settings_screen(&self, area: Rect, buf: &mut Buffer, block: Block)
    {
        // TODO: A standard settings screen that takes a list of settings/filters/sortable options & lists them

        // Tabs??     | Filter | Sort | Settings | Keybinds |
        // https://ratatui.rs/examples/widgets/tabs
    }

    fn draw_filter(&self, area: Rect, buf: &mut Buffer, block: Block)
    {
        // TODO: A standard screen for this, settings??, & sort
        //       Projects: 1, 2, _3_, 4
        //       Contexts: 1, 3, 2, _1_
        //       Complete    Incomplete
    }
    
    fn draw_sort(&self, area: Rect, buf: &mut Buffer, block: Block)
    {
        // TODO: A standard screen for this & filter
        //       <Ascending>  <Descending>
        //       Priority  Date Due
        //       Date Created  Alphabetical
        //       Project    Context
    }
    
    fn draw_settings(&self, area: Rect, buf: &mut Buffer, block: Block)
    {
        // TODO: A list of the settings & keybinds with their current values
    }

    fn draw_hello_world(area: Rect, buf: &mut Buffer, block: Block)
    {
    }
}




impl ratatui::widgets::Widget for &App
{
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    {  
        // TODO: Have the match create a block item that we display
        //       With each match arm just customising what's in the item
        //       And also have the arms populate the instructions
        // let content = match self.state
        
        let title = widgets::block::Title::from(self.title.bold());

        let instructions = widgets::block::Title::from(
            // self.instructions is a ratatui::text::Line
            self.instructions.clone()
        );

        let block = widgets::Block::bordered()
            .title(title.alignment(ratatui::layout::Alignment::Left))
            .title(instructions
                .alignment(ratatui::layout::Alignment::Center)
                .position(widgets::block::Position::Bottom)
            )
            .border_set(ratatui::symbols::border::ROUNDED);


        match self.state
        {
            State::Main           => self.draw_main(area, buf, block),
            State::Focused        => self.draw_focused(area, buf, block),
            State::UnsavedChanges => self.draw_unsaved_changes(area, buf, block),
            State::Filter         => self.draw_filter(area, buf, block),
            State::Sort           => self.draw_sort(area, buf, block),
            State::Settings       => self.draw_settings(area, buf, block),
            State::All            => App::draw_hello_world(area, buf, block), // This should be unreachable
        }
    }
}

*/
