use std::cmp;

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
            //Quit
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: event::KeyModifiers::CONTROL,
                kind: _,
                state: _,
            } => return Ok(false),
            KeyEvent {
                code: KeyCode::Backspace,
                modifiers: event::KeyModifiers::NONE,
                kind: _,
                state: _,
            } => self.output.remove_char(),
            //Basic Navigation controls
            KeyEvent {
                code: KeyCode::Char(val),
                modifiers: KeyModifiers::CONTROL,
                kind: _,
                state: _,
            } => match val {
                'n' | 'p' | 'f' | 'b' | 'a' | 'e' => self.output.move_cursor(val),
                's' => {
                    self.output.editor_rows.save()?;
                    self.output.dirty = 0
                }
                'u' | 'd' => {
                    if matches!(val, 'u') {
                        self.output.cursor_controller.cursor_y =
                            self.output.cursor_controller.row_offset
                    } else {
                        self.output.cursor_controller.cursor_y = cmp::min(
                            self.output.win_size.1 + self.output.cursor_controller.row_offset - 1,
                            self.output.editor_rows.number_of_rows(),
                        );
                    }
                    (0..self.output.win_size.1).for_each(|_| {
                        self.output
                            .move_cursor(if matches!(val, 'u') { 'p' } else { 'n' });
                    })
                }
                _ => {}
            },
            KeyEvent {
                code: code @ (KeyCode::Char(..) | KeyCode::Tab),
                modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
                kind: _,
                state: _,
            } => self.output.insert_char(match code {
                KeyCode::Tab => '\t',
                KeyCode::Char(ch) => ch,
                _ => unreachable!(),
            }),
            _ => {}
        }
        Ok(true)
    }

    pub fn run(&mut self) -> Result<bool, std::io::Error> {
        self.output.refresh_screen()?;
        self.process_keypress()
    }
}
