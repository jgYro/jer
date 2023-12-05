use std::cmp;

pub struct CursorController {
    pub cursor_x: usize,
    pub cursor_y: usize,
    screen_columns: usize,
    screen_rows: usize,
    pub row_offset: usize,
}

impl CursorController {
    pub fn new(win_size: (usize, usize)) -> CursorController {
        Self {
            cursor_x: 0,
            cursor_y: 0,
            screen_columns: win_size.0,
            screen_rows: win_size.1,
            row_offset: 0,
        }
    }

    pub fn scroll(&mut self) {
        self.row_offset = cmp::min(self.row_offset, self.cursor_y);
        if self.cursor_y >= self.row_offset + self.screen_rows {
            self.row_offset = self.cursor_y - self.screen_rows + 1;
        }
    }

    pub fn move_cursor(&mut self, direction: char, number_of_rows: usize) {
        match direction {
            'p' => {
                self.cursor_y = self.cursor_y.saturating_sub(1);
            }
            'b' => {
                if self.cursor_x != 0 {
                    self.cursor_x -= 1;
                }
            }
            'n' => {
                if self.cursor_y < number_of_rows {
                    self.cursor_y += 1;
                };
            }
            'f' => {
                if self.cursor_x != self.screen_columns - 1 {
                    self.cursor_x += 1
                };
            }
            'a' => self.cursor_x = 0,
            'e' => self.cursor_x = self.screen_columns - 1,
            _ => unimplemented!(),
        }
    }
}
