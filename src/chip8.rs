mod processor;
mod display;
mod keyboard;

use processor::Processor;
use display::Display;
use keyboard::Keyboard;

use std::io;
use std::thread;
use std::time::Duration;

const CLOCK_RATE: Duration = Duration::from_millis(1000 / 60);

pub struct Chip8 {
    processor: Processor,
    display: Display,
    keyboard: Keyboard,
}

impl Default for Chip8 {
    fn default() -> Self {
        Chip8 {
            processor: Processor::default(),
            display: Display::default(),
            keyboard: Keyboard::default(),
        }
    }
}

impl Chip8 {
    pub fn load_rom(&mut self, rom: &String) -> Result<(), io::Error>  {
        self.processor.load_rom(rom)
    }

    pub fn run(&mut self) {
        loop {
            self.processor.run_next_cycle();
            self.display.dump(self.processor.get_display_matrix());

            thread::sleep(CLOCK_RATE);
        }
    }
}
