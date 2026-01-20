use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
use std::io::{Write, stdout};

pub fn clear_screen() {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All)).unwrap();
}
