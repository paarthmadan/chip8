use super::hardware::Keyboard;

use termion::event::Key;
use termion::input::TermRead;

use std::io::stdin;

use std::thread;
use std::sync::{Arc, Mutex};

pub fn start_keyboard_listener(kb: Arc<Mutex<Keyboard>>) {
    thread::spawn(move || {
        let stdin = stdin();
        for key in stdin.keys().filter_map(|k| k.ok()) {
            match key {
                Key::Char(c) => {
                    if let Some(key) = char::to_digit(c, 16) {
                        let mut kb = kb.lock().unwrap();
                        kb.toggle(key as u8);
                    }
                }
                _ => continue,
            }
        }
    });
}
