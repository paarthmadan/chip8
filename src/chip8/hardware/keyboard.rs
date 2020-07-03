use termion::event::Key;
use termion::input::TermRead;

use super::driver::Keyboard;

use std::io::stdin;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn listen(kb: Arc<Mutex<Keyboard>>) {
    thread::spawn(move || {
        let stdin = stdin();
        for key in stdin.keys().filter_map(|k| k.ok()) {
            match key {
                Key::Char(c) => {
                    if let Some(key) = char::to_digit(c, 16) {
                        match kb.try_lock() {
                            Ok(mut kb) => kb.toggle(key as u8),
                            Err(_) => println!("foke"),
                        }
                    }
                }
                _ => continue,
            }
        }
    });
}
