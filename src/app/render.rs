use ratatui::{style::Stylize, widgets};

use super::{App, State};

impl App
{
    fn draw_main(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    {
        // TODO: Table with currently loaded todo items

        // A table for viewing the Todo items
        // https://ratatui.rs/showcase/widgets/#table

        // A scrollbar for going up/down
        // https://ratatui.rs/examples/widgets/scrollbar

        // A guage of total tasks complete?
        // https://ratatui.rs/examples/widgets/gauge
    }

    fn draw_unsaved_changes(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    {
        // TODO: A warning screen that says *unsaved changes ahead*
        //       Do you want to save?
        //           _Yes    _No     

        // Render self.previous_state first?
        // Then draw a little custom popup widget on top?
    }
    
    fn draw_focused(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
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

    fn settings_screen(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    {
        // TODO: A standard settings screen that takes a list of settings/filters/sortable options & lists them

        // Tabs??     | Filter | Sort | Settings | Keybinds |
        // https://ratatui.rs/examples/widgets/tabs
    }

    fn draw_filter(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    {
        // TODO: A standard screen for this, settings??, & sort
        //       Projects: 1, 2, _3_, 4
        //       Contexts: 1, 3, 2, _1_
        //       Complete    Incomplete
    }
    
    fn draw_sort(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    {
        // TODO: A standard screen for this & filter
        //       <Ascending>  <Descending>
        //       Priority  Date Due
        //       Date Created  Alphabetical
        //       Project    Context
    }
    
    fn draw_settings(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    {
        // TODO: A list of the settings & keybinds with their current values
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
        match self.state
        {
            State::Main           => self.draw_main(area, buf),
            State::Focused        => self.draw_focused(area, buf),
            State::UnsavedChanges => self.draw_unsaved_changes(area, buf),
            State::Filter         => self.draw_filter(area, buf),
            State::Sort           => self.draw_sort(area, buf),
            State::Settings       => self.draw_settings(area, buf),
            State::All            => panic!(), // This should be unreachable
        }
        
        let title = widgets::block::Title::from(self.title.bold());

        let instructions = widgets::block::Title::from(
            ratatui::text::Line::from(
                vec![
                    
                    " Quit:".into(), " <Q> ".blue().bold(),
                    " Tab:".into(), " <Tab> ".blue().bold(),
                ]
            )
        );

        let block = widgets::Block::bordered()
            .title(title.alignment(ratatui::layout::Alignment::Left))
            .title(instructions
                .alignment(ratatui::layout::Alignment::Center)
                .position(widgets::block::Position::Bottom)
            )
            .border_set(ratatui::symbols::border::ROUNDED);

        let text = ratatui::text::Text::from(
            vec![
                ratatui::text::Line::from(
                    vec![
                        "Hello World!".into()
                    ]
                )
            ]
        );

        widgets::Paragraph::new(text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

