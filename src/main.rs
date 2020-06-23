// memr_addr is referred to as _I_ in spec
struct Chip8 {
    memory: [u8; 4096],
    registers: [u8; 16],
    memr_addr: usize,
    delay: u8,
    sound: u8,
    pc: u16,
    sp: usize,
    stack: [u16; 16],
}

struct Cartridge {
}

impl Chip8 {
    fn load(&mut self, rom: Cartridge) {
        // load program into memory
    }

    fn run(&mut self) {
        // Instruction Fetch (Load instruction from memory)
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
            pc: 0,
            sp: 0,
            stack: [0; 16],
        }
    }
}


fn main () {
    let chip = Chip8::default();
    println!("Chip8 Emulator!");
}
