use crossterm::event::KeyEvent;

pub enum Event{
    Key(KeyEvent),
    Resize(u16,u16),
}

