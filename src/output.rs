use std::{
    cmp,
    io::{stdout, Write},
};

use crossterm::{
    cursor, execute, queue,
    terminal::{self, ClearType},
};

use crate::cursorcontroller::CursorController;
use crate::editor_contents::EditorContents;
use crate::editor_rows::EditorRows;

pub struct Output {
    pub win_size: (usize, usize),
    editor_contents: EditorContents,
    cursor_controller: CursorController,
    editor_rows: EditorRows,
}

impl Output {
    pub fn new() -> Self {
        let win_size = terminal::size()
            .map(|(x, y)| (x as usize, y as usize))
            .unwrap();
        Self {
            win_size,
            editor_contents: EditorContents::new(),
            cursor_controller: CursorController::new(win_size),
            editor_rows: EditorRows::new(),
        }
    }

    pub fn move_cursor(&mut self, direction: char) {
        self.cursor_controller
            .move_cursor(direction, self.editor_rows.number_of_rows());
    }

    pub fn clear_screen() -> Result<(), std::io::Error> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))
    }

    fn draw_rows(&mut self) {
        let screen_rows = self.win_size.1;
        let screen_columns = self.win_size.0;
        for i in 0..screen_rows {
            let file_row = i + self.cursor_controller.row_offset; // add line
            if file_row >= self.editor_rows.number_of_rows() {
                //modify
                if self.editor_rows.number_of_rows() == 0 && i == screen_rows / 3 {
                    let mut welcome = format!("Pound Editor --- Version {}", "Hello");
                    if welcome.len() > screen_columns {
                        welcome.truncate(screen_columns)
                    }
                    let mut padding = (screen_columns - welcome.len()) / 2;
                    if padding != 0 {
                        self.editor_contents.push('~');
                        padding -= 1
                    }
                    (0..padding).for_each(|_| self.editor_contents.push(' '));
                    self.editor_contents.push_str(&welcome);
                } else {
                    self.editor_contents.push('~');
                }
            } else {
                let len = cmp::min(self.editor_rows.get_row(file_row).len(), screen_columns); //modify
                self.editor_contents
                    .push_str(&self.editor_rows.get_row(file_row)[..len]) //modify
            }
            queue!(
                self.editor_contents,
                terminal::Clear(ClearType::UntilNewLine)
            )
            .unwrap();
            if i < screen_rows - 1 {
                self.editor_contents.push_str("\r\n");
            }
        }
    }
    pub fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        self.cursor_controller.scroll();
        queue!(self.editor_contents, cursor::Hide, cursor::MoveTo(0, 0))?;
        self.draw_rows();
        let cursor_x = self.cursor_controller.cursor_x;
        //modify
        let cursor_y = self.cursor_controller.cursor_y - self.cursor_controller.row_offset;
        queue!(
            self.editor_contents,
            cursor::MoveTo(cursor_x as u16, cursor_y as u16),
            cursor::Show
        )?;
        self.editor_contents.flush()
    }
}
