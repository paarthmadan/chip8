extern crate rand;

mod chip8;
mod display;

use chip8::Chip8;
use std::env;

fn main () {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: cargo run <file>");
        std::process::exit(1);
    }

    let filename = &args[1];

    let mut chip = Chip8::default();

    if chip.load_rom(filename).is_err() {
        eprintln!("Could not load ROM from: {}", filename);
        std::process::exit(1);
    }

    println!("Chip8 Emulator!");

    chip.run();
}
