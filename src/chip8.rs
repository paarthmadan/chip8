mod driver;
mod hardware;
mod processor;

use driver::{Display, Keyboard};
use processor::{Processor, ProcessorState};

use std::io;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};

use super::Opt;

pub struct Chip8 {
    processor: Processor,
    display: Display,
    keyboard: Arc<Mutex<Keyboard>>,
    clock_rate: Duration,
}

impl From<&Opt> for Chip8 {
    fn from(opt: &Opt) -> Self {
        Chip8 {
            clock_rate: Duration::from_millis(1000 / opt.clock_rate as u64),
            processor: Processor::default(),
            display: Display::from(opt),
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
                ProcessorState::Continue => {
                    self.display.output();
                    self.keyboard.lock().unwrap().clear_state();
                }
                ProcessorState::BlockForIO => {
                    continue;
                }
            }

            println!("{}", now.elapsed().unwrap().as_millis());

            thread::sleep(self.clock_rate);
        }
    }
}
