extern crate rand;
extern crate structopt;
extern crate termion;

mod chip8;
mod cli;

use chip8::Chip8;
use cli::Opt;
use structopt::StructOpt;

fn main() {
    let opt = Opt::from_args();
    let mut chip = Chip8::from(&opt);

    if chip.load_rom(&opt.rom).is_err() {
        eprintln!("Could not load ROM from: {}", &opt.rom);
        std::process::exit(1);
    }

    chip.run();
}
