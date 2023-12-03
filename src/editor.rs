use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers};

use crate::{output::Output, reader::Reader};

pub struct Editor {
    reader: Reader,
    output: Output,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            reader: Reader,
            output: Output::new(),
        }
    }

    pub fn process_keypress(&mut self) -> Result<bool, std::io::Error> {
        match self.reader.read_key()? {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: event::KeyModifiers::CONTROL,
                kind: _,
                state: _,
            } => return Ok(false),
            KeyEvent {
                code: KeyCode::Char(val),
                modifiers: KeyModifiers::CONTROL,
                kind: _,
                state: _,
            } => match val {
                'n' | 'p' | 'f' | 'b' => self.output.move_cursor(val),
                _ => {}
            },
            _ => {}
        }
        Ok(true)
    }

    pub fn run(&mut self) -> Result<bool, std::io::Error> {
        self.output.refresh_screen()?;
        self.process_keypress()
    }
}
