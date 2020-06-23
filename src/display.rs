pub struct Display {
    lcd: [[char; 64]; 32]
}

impl Display {
    pub fn dump(&self) {
        for row in &self.lcd {
            for c in row.iter() {
                print!("{}", c);
            }
            println!("");
        }
    }

    pub fn clear(&mut self) {
        self.lcd = [[' '; 64]; 32];
    }
}

impl Default for Display {
    fn default() -> Self {
        Display { lcd: [[' '; 64]; 32] }
    }
}
