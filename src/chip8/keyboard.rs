
pub struct Keyboard {
    pub pad: [bool; 16],
}

impl Default for Keyboard {
    fn default() -> Self {
        Keyboard { pad: [false; 16] }
    }
}

impl Keyboard {
    pub fn clear(&mut self) {
        self.pad = [false; 16];
    }

    pub fn toggle(&mut self, key: u8) {
        self.pad[key as usize] = true;
    }

    pub fn dump(&self) {
        for key in &self.pad {
            print!("{}", if *key == true { "1" } else { "0" });
        }
        println!("");
    }
}
