use super::hardware;

use std::sync::{Arc, Mutex};

pub struct Keyboard {
    pad: [bool; 16],
}

impl Default for Keyboard {
    fn default() -> Self {
        Keyboard { pad: [false; 16] }
    }
}

impl Keyboard {
    pub fn start_listening(self) -> Arc<Mutex<Self>> {
        let mutex = Arc::new(Mutex::new(self));
        hardware::keyboard::listen(Arc::clone(&mutex));

        mutex
    }

    pub fn clear_state(&mut self) {
        self.pad = [false; 16];
    }

    pub fn toggle(&mut self, key: u8) {
        println!("{}", key);
        self.pad[key as usize] = true;
    }

    pub fn read(&self) -> &[bool; 16] {
        &self.pad
    }

    pub fn dump(&self) {
        for key in &self.pad {
            print!("{}", if *key == true { "1" } else { "0" });
        }
        println!("");
    }
}
