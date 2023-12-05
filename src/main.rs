use cleanup::CleanUp;
use crossterm::terminal;
use editor::Editor;

mod cleanup;
mod cursorcontroller;
mod editor;
mod editor_contents;
mod editor_rows;
mod output;
mod reader;

fn main() -> Result<(), std::io::Error> {
    let _clean_up = CleanUp;
    terminal::enable_raw_mode()?;
    let mut editor = Editor::new();
    while editor.run()? {}
    Ok(())
}
