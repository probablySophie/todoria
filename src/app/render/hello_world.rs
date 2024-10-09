use ratatui::{widgets::{Block, Paragraph}, Frame};


#[derive(Debug)]
pub struct HelloWorld<'a>
{
	paragraph: Paragraph<'a>
}
impl<'a> HelloWorld<'a>
{
	pub fn new(text: &str) -> Self
	{
        let text = ratatui::text::Text::from(
            vec![
                ratatui::text::Line::from(
                    vec![
                        text.to_owned().into()
                    ]
                )
            ]
        );

        Self
        {
        	paragraph: Paragraph::new(text)
				.centered(),
		}
	}
	
	pub fn render(&mut self, frame: &mut Frame, block: Block)
	{
		frame.render_widget(self.paragraph.clone().block(block), frame.area());
	}
}
