mod driver;
mod hardware;
mod processor;

use driver::{Display, Keyboard};
use processor::{Processor, ProcessorState};

use std::io;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};

const CLOCK_RATE: Duration = Duration::from_millis(1000 / 250);

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
    pub fn load_rom(&mut self, rom: &str) -> Result<(), io::Error> {
        self.processor.load_rom(rom)
    }

    pub fn run(&mut self) {
        loop {
            let input = self.keyboard.lock().unwrap();
            let buffer = *input.read();

            std::mem::drop(input);

            let now = SystemTime::now();

            match self.processor.next(&buffer, &mut self.display) {
                ProcessorState::Continue(flush) => {
                    self.display.output();
                    if flush {
                        self.keyboard.lock().unwrap().clear_state();
                    }
                }
                ProcessorState::BlockForIO => {
                    continue;
                }
            }

            println!("{}", now.elapsed().unwrap().as_millis());

            thread::sleep(CLOCK_RATE);
        }
    }
}
