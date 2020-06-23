use std::env;
use std::fs;
use std::io;
// memr_addr is referred to as _I_ in spec
struct Chip8 {
    memory: [u8; 4096],
    registers: [u8; 16],
    memr_addr: usize,
    delay: u8,
    sound: u8,
    pc: usize,
    sp: usize,
    stack: [u16; 16],
    display: Display,
}

struct Display {
    lcd: [[char; 64]; 32]
}

impl Display {
    fn dump(&self) {
        for row in &self.lcd {
            for c in row.iter() {
                print!("{}", c);
            }
            println!("");
        }
    }
}

impl Default for Display {
    fn default() -> Self {
        Display { lcd: [[' '; 64]; 32] }
    }
}

impl Chip8 {
    const MEM_START: usize = 512;

    fn load_rom(&mut self, rom: &String) -> Result<(), io::Error> {
        let bytes = fs::read(rom)?;

        let prog_end = Chip8::MEM_START + bytes.len();

        if prog_end > 4096 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "ROM exceeds 4096 bytes"))
        }

        self.memory[Chip8::MEM_START..prog_end].copy_from_slice(&bytes);

        Ok(())
    }

    fn load_word(&self, ptr: usize) -> u8 {
        self.memory[ptr]
    }

    fn run(&mut self) {
        // Instruction Fetch (Load instruction from memory)
        let upper_word = self.load_word(self.pc);
        let lower_word = self.load_word(self.pc + 1);

        let a = upper_word >> 4;
        let b = (0x0F) & upper_word;
        let c = lower_word >> 4;
        let d = (0x0F) & lower_word;

        match (a, b, c, d) {
            (0, 0, 0xE, 0) => println!("clear"),
            (0, 0, 0xE, 0xE) => println!("ret"),
            (1, _, _, _) => println!("jump"),
            (2, _, _, _) => println!("call"),
            _ => unreachable!("Instruction not supported: {:x?}{:x?}{:x?}{:x?}", a, b, c, d),
        }
        // Instruction Decode (Figure out what to do)
        // Instruction Execute
    }

}

impl Default for Chip8 {
    fn default() -> Self {
        Chip8 {
            memory: [0; 4096],
            registers: [0; 16],
            memr_addr: 0,
            delay: 0,
            sound: 0,
            pc: 512,
            sp: 0,
            stack: [0; 16],
            display: Display::default(),
        }
    }
}


fn main () {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: cargo run <file>");
        std::process::exit(1);
    }

    let file = &args[1];

    let mut chip = Chip8::default();

    if chip.load_rom(file).is_err() {
        eprintln!("Could not load ROM!");
        std::process::exit(1);
    }

    println!("Chip8 Emulator!");
    chip.run();
}
