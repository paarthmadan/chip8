extern crate rand;
use crate::rand::Rng;
use std::env;
use std::fs;
use std::io;
use std::thread;
use std::time::Duration;
// memr_addr is referred to as _I_ in spec
struct Chip8 {
    memory: [u8; 4096],
    registers: [u8; 16],
    mem_addr_register: usize,
    delay: u8,
    sound: u8,
    pc: usize,
    sp: usize,
    stack: [usize; 16],
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

    fn clear(&mut self) {
        self.lcd = [[' '; 64]; 32];
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

    fn return_from_routine(&mut self) {
        self.pc = self.stack[self.sp];
        self.sp -= 1;
    }

    fn jump(&mut self, addr: u16) {
        self.pc = addr as usize;
    }

    fn call(&mut self, addr: u16) {
        self.sp += 1;
        self.stack[self.sp] = self.pc;
        self.pc = addr as usize;
    }

    fn read_register(&self, reg: u8) -> u8 {
        self.registers[reg as usize]
    }

    fn write_register(&mut self, reg: u8, value: u8) {
        self.registers[reg as usize] = value;
    }

    fn write_flag(&mut self, value: u8) {
        self.write_register(0xF, value);
    }

    fn run(&mut self) {
        loop {
            // Instruction Fetch (Load instruction from memory)
            let upper_word = self.load_word(self.pc);
            let lower_word = self.load_word(self.pc + 1);

            // Instruction Decode
            let a = upper_word >> 4;
            let b = (0x0F) & upper_word;
            let c = lower_word >> 4;
            let d = (0x0F) & lower_word;

            let addr: u16 = b as u16 + lower_word as u16;

            // Instruction Execute
            match (a, b, c, d) {
                (0, 0, 0xE, 0) => self.display.clear(),
                (0, 0, 0xE, 0xE) => {
                    self.return_from_routine();
                    continue;
                }
                (1, _, _, _) =>  {
                    self.jump(addr);
                    continue;
                }
                (2, _, _, _) => {
                    self.call(addr);
                    continue;
                },
                (3, reg, _, _) => {
                    if self.read_register(reg) == lower_word {
                        self.pc += 2;
                    }
                },
                (4, reg, _, _) => {
                    if self.read_register(reg) != lower_word {
                        self.pc += 2;
                    }
                },
                (5, reg1, reg2, 0) => {
                    if self.read_register(reg1) == self.read_register(reg2) {
                        self.pc += 2;
                    }
                },
                (6, dst, _, _) => self.write_register(dst, lower_word),
                (7, dst, _, _) => self.write_register(dst, self.read_register(dst) + lower_word),
                (8, dst, src, flag) => {
                    let (dst_val, src_val) = (self.read_register(dst), self.read_register(src));
                    let res = match flag {
                        0 => src_val,
                        1 => dst_val | src_val,
                        2 => dst_val & src_val,
                        3 => dst_val ^ src_val,
                        4 => {
                            let (sum, overflow) = dst_val.overflowing_add(src_val);
                            self.write_flag(overflow as u8);

                            sum
                        },
                        5 => {
                            let (diff, borrow) = dst_val.overflowing_sub(src_val);
                            self.write_flag(!borrow as u8);

                            diff
                        },
                        6 => {
                            let lsb = 0x01 & dst_val;
                            self.write_flag(lsb);

                            dst_val >> 1
                        }
                        7 => {
                            let (diff, borrow) = src_val.overflowing_sub(dst_val);
                            self.write_flag(!borrow as u8);

                            diff
                        },
                        0xE => {
                            let msb = 0x10 & dst_val;
                            self.write_flag(msb);

                            dst_val << 1
                        },
                        _ => unreachable!("Arithmetic Instruction not supported: {:x?}{:x?}{:x?}{:x?}", a, b, c, d),
                    };
                    self.write_register(dst, res);
                },
                (9, reg1, reg2, 0) => {
                    if self.read_register(reg1) != self.read_register(reg2) {
                        self.pc += 2;
                    }
                },
                (0xA, _, _, _) => self.mem_addr_register = addr as usize,
                (0xB, _, _, _) => self.jump(addr + self.read_register(0) as u16),
                (0xC, reg, _, _) => {
                    let mut rng = rand::thread_rng();
                    let rand: u8 = rng.gen();

                    self.write_register(reg, rand & lower_word)
                },
                _ => unreachable!("Instruction not supported: {:x?}{:x?}{:x?}{:x?}", a, b, c, d),
            }

            self.pc += 2;

            thread::sleep(Duration::from_millis(1000 / 60));
        }
    }

}

impl Default for Chip8 {
    fn default() -> Self {
        Chip8 {
            memory: [0; 4096],
            registers: [0; 16],
            mem_addr_register: 0,
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
