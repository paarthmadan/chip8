use std::io::{Write, stdout, Stdout};
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Display {
    lcd: [[u8; 64]; 32], 
    stdout: RawTerminal<Stdout>,
}

impl Display {
    pub const DIGIT_SPRITES: [u8; 5*16] = [
        0xF0, 0x90, 0x90, 0x90, 0xF0,
        0x20, 0x60, 0x20, 0x20, 0x70,
        0xF0, 0x10, 0xF0, 0x80, 0xF0,
        0xF0, 0x10, 0xF0, 0x10, 0xF0,
        0x90, 0x90, 0xF0, 0x10, 0x10,
        0xF0, 0x80, 0xF0, 0x10, 0xF0,
        0xF0, 0x80, 0xF0, 0x90, 0xF0,
        0xF0, 0x10, 0x20, 0x40, 0x40,
        0xF0, 0x90, 0xF0, 0x90, 0xF0,
        0xF0, 0x90, 0xF0, 0x10, 0xF0,
        0xF0, 0x90, 0xF0, 0x90, 0x90,
        0xE0, 0x90, 0xE0, 0x90, 0xE0,
        0xF0, 0x80, 0x80, 0x80, 0xF0,
        0xE0, 0x90, 0x90, 0x90, 0xE0,
        0xF0, 0x80, 0xF0, 0x80, 0xF0,
        0xF0, 0x80, 0xF0, 0x80, 0x80,
    ];

    pub fn dump(&mut self) {
        write!(self.stdout, "{}{}{}", termion::clear::All, termion::cursor::Goto(1, 1), termion::cursor::Hide).unwrap();
        for y in 0..32 {
            for x in 0..64 {
                write!(self.stdout, "{}{}", termion::cursor::Goto(x + 1, y + 1), if self.lcd[y as usize][x as usize] == 1 { '*' } else { ' ' }).unwrap();
            }
        }
        self.stdout.flush().unwrap();
    }

    pub fn write(&mut self, x: u8, y: u8, sprite: &Vec<u8>) -> bool {
        let mut changed = false;
        for (i, row) in sprite.iter().enumerate() {
            for x_offset in 0..=7 {
                let px = (x + x_offset) % 64;
                let py = (y + i as u8) % 32;

                let curr = self.lcd[py as usize][px as usize];
                let new = 0x01 & (row >> (7 - (x_offset)));

                self.lcd[py as usize][px as usize] = curr ^ new;

                if !changed && curr == 1 && curr ^ new == 0 {
                    changed = true;
                }
            }
        }
        return changed;
    }

    pub fn clear(&mut self) {
        self.lcd = [[0; 64]; 32];
    }
}

impl Default for Display {
    fn default() -> Self {
        Display { lcd: [[0; 64]; 32], stdout: stdout().into_raw_mode().unwrap()}
    }
}
