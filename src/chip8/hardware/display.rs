use std::io::{Write, stdout, Stdout};
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Display {
    stdout: RawTerminal<Stdout>,
}

impl Display {
    pub fn dump(&mut self, display_matrix: &[[u8; 64]; 32]) {
        write!(self.stdout, "{}{}{}", termion::clear::All, termion::cursor::Goto(2, 1), termion::cursor::Hide).unwrap();
        for y in 0..32 {
            for x in 0..64 {
                write!(self.stdout, "{}{}", termion::cursor::Goto(x + 1, y + 1), if display_matrix[y as usize][x as usize] == 1 { '*' } else { ' ' }).unwrap();
            }
        }
        self.stdout.flush().unwrap();
    }
}

impl Default for Display {
    fn default() -> Self {
        Display { stdout: stdout().into_raw_mode().unwrap()}
    }
}
