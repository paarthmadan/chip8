use super::hardware;

pub struct Display {
    buffer: [[u8; 64]; 32],
    lcd: hardware::Display,
}

impl Display {
    pub fn output(&mut self) {
        self.lcd.dump(&self.buffer);
    }

    pub fn write_buffer(&mut self, x: u8, y: u8, sprite: &Vec<u8>) -> bool {
        let mut changed = false;
        for (i, row) in sprite.iter().enumerate() {
            for x_offset in 0..=7 {
                let px = (x + x_offset) % 64;
                let py = (y + i as u8) % 32;

                let curr = self.buffer[py as usize][px as usize];
                let new = 0x01 & (row >> (7 - (x_offset)));

                self.buffer[py as usize][px as usize] = curr ^ new;

                if !changed && curr == 1 && curr ^ new == 0 {
                    changed = true;
                }
            }
        }
        return changed;
    }

    pub fn clear_buffer(&mut self) {
        self.buffer = [[0; 64]; 32];
    }
}

impl Default for Display {
    fn default() -> Self {
        Display {
            buffer: [[0; 64]; 32],
            lcd: hardware::Display::default(),
        }
    }
}
