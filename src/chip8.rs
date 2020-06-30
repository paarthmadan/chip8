mod processor;
mod display;
mod keyboard;

use processor::Processor;
use display::Display;
use keyboard::Keyboard;

use std::io;

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
        self.processor.run()
    }
}
