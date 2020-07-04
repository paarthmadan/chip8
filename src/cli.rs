use super::StructOpt;
#[derive(Debug, StructOpt, Clone)]
#[structopt(name = "chip8", about = "A CHIP-8 VM emulator.")]

pub struct Opt {
    #[structopt(name = "PATH_TO_ROM", required = true)]
    pub rom: String,
    #[structopt(short, long, default_value = "250")]
    pub clock_rate: u16,
    #[structopt(short, long, default_value = "█")]
    pub symbol: char,
}
