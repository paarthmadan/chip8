mod processor;
mod hardware;
mod driver;

use processor::{Processor, ProcessorState};
use driver::{Display, Keyboard};

use std::io;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

const CLOCK_RATE: Duration = Duration::from_millis(1000 / 60);

pub struct Chip8 {
    processor: Processor,
    display: Display,
    keyboard: Arc<Mutex<Keyboard>>,
}

impl Default for Chip8 {
    fn default() -> Self {

        Chip8 {
            processor: Processor::default(),
            display: Display::default(),
            keyboard: Keyboard::default().start_listening(),
        }
    }
}

impl Chip8 {
    pub fn load_rom(&mut self, rom: &String) -> Result<(), io::Error>  {
        self.processor.load_rom(rom)
    }

    pub fn run(&mut self) {
        loop {
            let mut input = self.keyboard.lock().unwrap();
            let buffer = input.read();

            match self.processor.next(buffer, &mut self.display) {
                ProcessorState::Ready => {
                    self.display.output();
                    input.clear_state();
                },
                ProcessorState::WaitingForIO => {
                    continue;
                }
            }

            thread::sleep(CLOCK_RATE);
        }
    }
}


