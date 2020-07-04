extern crate rand;
extern crate structopt;
extern crate termion;

mod chip8;

use chip8::Chip8;
use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone)]
#[structopt(name = "chip8", about = "A CHIP-8 VM emulator.")]
struct Opt {
    #[structopt(name = "PATH_TO_ROM", required = true)]
    rom: String,
    #[structopt(short, long, default_value = "250")]
    clock_rate: u16,
    #[structopt(short, long, default_value = "█")]
    symbol: char,
}

fn main() {
    let opt = Opt::from_args();
    let mut chip = Chip8::from(&opt);

    if chip.load_rom(&opt.rom).is_err() {
        eprintln!("Could not load ROM from: {}", &opt.rom);
        std::process::exit(1);
    }

    chip.run();
}
