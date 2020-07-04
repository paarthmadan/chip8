use std::io::{stdout, Stdout, Write};
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Display {
    stdout: RawTerminal<Stdout>,
    symbol: char,
}

impl Display {
    pub fn dump(&mut self, display_matrix: &[[u8; 64]; 32]) {
        write!(
            self.stdout,
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(2, 1),
            termion::cursor::Hide
        )
        .unwrap();
        for y in 0..32 {
            for x in 0..64 {
                write!(
                    self.stdout,
                    "{}{}",
                    termion::cursor::Goto(x + 1, y + 1),
                    if display_matrix[y as usize][x as usize] == 1 {
                        self.symbol
                    } else {
                        ' '
                    }
                )
                .unwrap();
            }
        }
        self.stdout.flush().unwrap();
    }

    pub fn new(symbol: char) -> Self {
        Display {
            symbol,
            stdout: stdout().into_raw_mode().unwrap(),
        }
    }
}
