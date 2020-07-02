mod processor;
mod hardware;
mod driver;

use processor::{Processor, ProcessorState};
use hardware::{Display, Keyboard};

use std::io;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

const CLOCK_RATE: Duration = Duration::from_millis(1000 / 10);

pub struct Chip8 {
    processor: Processor,
    display: Display,
    keyboard: Arc<Mutex<Keyboard>>,
}

impl Default for Chip8 {
    fn default() -> Self {
        let kb = Keyboard::default();
        let kb = Arc::new(Mutex::new(kb));

        driver::keyboard::start_keyboard_listener(Arc::clone(&kb));

        Chip8 {
            processor: Processor::default(),
            display: Display::default(),
            keyboard: kb,
        }
    }
}

impl Chip8 {
    pub fn load_rom(&mut self, rom: &String) -> Result<(), io::Error>  {
        self.processor.load_rom(rom)
    }

    pub fn run(&mut self) {
        loop {
            match self.processor.run_next_cycle() {
                ProcessorState::Ready => {
                    self.display.dump(self.processor.get_display_matrix());
                },
                ProcessorState::WaitingForIO => {
                }
            }
            let mut kb = self.keyboard.lock().unwrap();
            kb.dump();
            kb.clear();
            thread::sleep(CLOCK_RATE);
        }
    }
}


