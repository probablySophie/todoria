pub trait Screen
{
	/// For making a new instance of the Screen item
	fn new() -> Self;
	/// For actively displaying the Screen item
	fn display();
	/// Called on render
	fn render(&mut self, frame: &mut Frame, outer_block: Block);
}

// TODO: This declaration will need to be moved to inside of a macro run in main.rs
pub struct Screens
{
	//
}
impl Screens
{
	// TODO: I will need to be a macro loop that goes through all of the States & which screen to call .render on
	pub fn render()
	{
		//
	}
}
