use termion::event::Key;
use termion::input::TermRead;

use std::io::stdin;

pub fn try_poll() -> Option<u8> {
    let stdin = stdin();
    let mut keys = stdin.keys().filter_map(|key| key.ok());

    let key: Option<u32> = keys.find_map(|key| match key {
        Key::Char(c) => char::to_digit(c, 16),
        _ => None,
    });

    key.map(|k| k as u8)
}

pub fn poll() -> u8 {
    loop {
        let stdin = stdin();
        let mut keys = stdin.keys().filter_map(|key| key.ok());

        let key: Option<u32> = keys.find_map(|key| match key {
            Key::Char(c) => char::to_digit(c, 16),
            _ => None,
        });

        if let Some(k) = key {
            return k as u8;
        }
    }
}
