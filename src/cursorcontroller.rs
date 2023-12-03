pub struct CursorController {
    pub cursor_x: usize,
    pub cursor_y: usize,
}

impl CursorController {
    pub fn new() -> CursorController {
        Self {
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    pub fn move_cursor(&mut self, direction: char) {
        match direction {
            'p' => {
                self.cursor_y -= 1;
            }
            'b' => {
                self.cursor_x -= 1;
            }
            'n' => {
                self.cursor_y += 1;
            }
            'f' => {
                self.cursor_x += 1;
            }
            _ => unimplemented!(),
        }
    }
}
