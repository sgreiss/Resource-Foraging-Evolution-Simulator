use crossterm::{
    execute,
    style::{Color, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::io::{Write, stdout};

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
