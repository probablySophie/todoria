#[cfg(test)] use crate::app::App;
#[cfg(test)] use crossterm::event;


#[test]
fn key_pressed()
{
    // Make sure the Q key exits the app
    let mut app = App::default();
    app.key_pressed(event::KeyCode::Char('q'));

    assert!(app.exit);
}

