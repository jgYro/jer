use crossterm::event::{self, Event, KeyEvent};

pub struct Reader;

impl Reader {
    pub fn read_key(&self) -> Result<KeyEvent, std::io::Error> {
        loop {
            if let Event::Key(event) = event::read()? {
                return Ok(event);
            }
        }
    }
}
