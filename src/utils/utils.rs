use crossterm::{
    execute,
    style::{Color, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::io::{stdout};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Coordinate(u32, u32);

impl Coordinate {
    pub fn new(x: u32, y: u32) -> Self {
        Coordinate(x, y)
    }

    pub fn from_usize(x: usize, y: usize) -> Self {
        Coordinate(x as u32, y as u32)
    }

    pub fn as_usize(&self) -> (usize, usize) {
        (self.0 as usize, self.1 as usize)
    }
}

pub fn clear_screen() {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All)).unwrap();
}

pub fn print_colored(text: &str, color: Color) {
    let mut stdout = stdout();
    execute!(stdout, SetForegroundColor(color)).unwrap();
    print!("{}", text);
    execute!(stdout, SetForegroundColor(Color::Reset)).unwrap();
}

pub fn println_colored(text: &str, color: Color) {
    let mut stdout = stdout();
    execute!(stdout, SetForegroundColor(color)).unwrap();
    println!("{}", text);
    execute!(stdout, SetForegroundColor(Color::Reset)).unwrap();
}
