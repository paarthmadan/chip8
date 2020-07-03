use rand::Rng;
use std::fs; 
use std::io;
use super::driver::Display;

pub struct Processor { 
    memory: [u8; 4096],
    registers: [u8; 16],
    mem_addr_register: usize,
    delay: u8,
    sound: u8,
    pc: usize,
    sp: usize,
    stack: [usize; 16],
}

pub enum ProcessorState {
    Ready,
    WaitingForIO,
}

pub const DIGIT_SPRITES: [u8; 5*16] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0,
    0x20, 0x60, 0x20, 0x20, 0x70,
    0xF0, 0x10, 0xF0, 0x80, 0xF0,
    0xF0, 0x10, 0xF0, 0x10, 0xF0,
    0x90, 0x90, 0xF0, 0x10, 0x10,
    0xF0, 0x80, 0xF0, 0x10, 0xF0,
    0xF0, 0x80, 0xF0, 0x90, 0xF0,
    0xF0, 0x10, 0x20, 0x40, 0x40,
    0xF0, 0x90, 0xF0, 0x90, 0xF0,
    0xF0, 0x90, 0xF0, 0x10, 0xF0,
    0xF0, 0x90, 0xF0, 0x90, 0x90,
    0xE0, 0x90, 0xE0, 0x90, 0xE0,
    0xF0, 0x80, 0x80, 0x80, 0xF0,
    0xE0, 0x90, 0x90, 0x90, 0xE0,
    0xF0, 0x80, 0xF0, 0x80, 0xF0,
    0xF0, 0x80, 0xF0, 0x80, 0x80,
];

impl Processor {
    const MEM_START: usize = 512;

    fn store_word(&mut self, offset: usize, word: u8) {
        self.memory[self.mem_addr_register + offset] = word;
    }

    fn load_word(&self, ptr: usize) -> u8 {
        self.memory[ptr]
    }

    fn load_sprite(&self, n: u8) -> Vec<u8> {
        let mut sprite: Vec<u8> = Vec::with_capacity(n as usize);
        
        for i in 0..n {
            sprite.push(self.load_word(self.mem_addr_register + i as usize));
        }

        sprite
    }

    fn return_from_routine(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp];
    }

    fn jump(&mut self, addr: u16) {
        self.pc = addr as usize;
    }

    fn call(&mut self, addr: u16) {
        self.stack[self.sp] = self.pc + 2;
        self.sp += 1;
        self.pc = addr as usize;
    }

    fn read_register(&self, reg: u8) -> u8 {
        self.registers[reg as usize]
    }

    fn write_register(&mut self, reg: u8, value: u8) {
        self.registers[reg as usize] = value;
    }

    fn write_mem_addr_register(&mut self, value: u16) {
        self.mem_addr_register = value as usize;
    }

    fn increment_mem_addr_register(&mut self, incr: u8) {
        self.mem_addr_register += incr as usize;
    }

    fn write_flag(&mut self, value: u8) {
        self.write_register(0xF, value);
    }

    pub fn load_rom(&mut self, rom: &String) -> Result<(), io::Error> {
        let bytes = fs::read(rom)?;

        let prog_end = Processor::MEM_START + bytes.len();

        if prog_end > 4096 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "ROM exceeds 4096 bytes"))
        }

        self.memory[Processor::MEM_START..prog_end].copy_from_slice(&bytes);

        Ok(())
    }

    pub fn next(&mut self, input: &[bool; 16], display: &mut Display) -> ProcessorState {
        // Instruction Fetch (Load instruction from memory)
        let upper_word = self.load_word(self.pc);
        let lower_word = self.load_word(self.pc + 1);

        // Instruction Decode
        let a = upper_word >> 4;
        let b = (0x0F) & upper_word;
        let c = lower_word >> 4;
        let d = (0x0F) & lower_word;

        //println!("{:x?}{:x?}{:x?}{:x?}", a, b, c, d);

        let addr: u16 = ((b as u16) << 8)  + lower_word as u16;

        // Instruction Execute
        match (a, b, c, d) {
            (0, 0, 0xE, 0) => display.clear_buffer(),
            (0, 0, 0xE, 0xE) => {
                self.return_from_routine();
                return ProcessorState::Ready
            },
            (0, _, _, _) => unimplemented!{},
            (1, _, _, _) =>  {
                self.jump(addr);
                return ProcessorState::Ready
            }
            (2, _, _, _) => {
                self.call(addr);
                return ProcessorState::Ready
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
            (7, dst, _, _) => self.write_register(dst, self.read_register(dst).overflowing_add(lower_word).0),
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
                        let msb = dst_val >> 7;
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
            (0xA, _, _, _) => self.write_mem_addr_register(addr),
            (0xB, _, _, _) => self.jump(addr + self.read_register(0) as u16),
            (0xC, reg, _, _) => {
                let mut rng = rand::thread_rng();
                let rand: u8 = rng.gen();

                self.write_register(reg, rand & lower_word)
            },
            (0xD, x, y, n) =>  {
                let sprite = self.load_sprite(n);
                let flag = display.write_buffer(self.read_register(x), self.read_register(y), &sprite);

                self.write_flag(flag as u8);
            },
            (0xE, reg, 0x9, 0xE) => {
                if input[self.read_register(reg) as usize] == true {
                    self.pc += 2;
                }
            },
            (0xE, reg, 0xA, 1) => {
                if input[self.read_register(reg) as usize] == false {
                    self.pc += 2;
                }
            },
            (0xF, reg, 0, 7) => self.write_register(reg, self.delay),
            (0xF, reg, 0, 0xA) => {
                match input.into_iter().position(|&k| k == true) {
                    Some(index) => {
                        self.write_register(reg, index as u8);
                    },
                    None => return ProcessorState::WaitingForIO,
                }
            }
            (0xF, reg, 1, 5) => self.delay = self.read_register(reg),
            (0xF, reg, 1, 8) => self.sound = self.read_register(reg),
            (0xF, reg, 1, 0xE) => self.increment_mem_addr_register(self.read_register(reg)),
            (0xF, reg, 2, 9) => self.increment_mem_addr_register(5 * self.read_register(reg)),
            (0xF, reg, 3, 3) => {
                let dec = self.read_register(reg);

                self.store_word(0, dec / 100);
                self.store_word(1, (dec % 100) / 10);
                self.store_word(2, dec % 10);
            },
            (0xF, reg, 5, 5) => {
                println!("{}", reg);
                let register_vals: Vec<u8> = (0..=reg).into_iter().map(|reg| self.read_register(reg)).collect();
                for (i, val) in register_vals.iter().enumerate() {
                    self.store_word(i, *val);
                }

                self.increment_mem_addr_register(reg + 1);
            },
            (0xF, reg, 6, 5) => {
                let memory_vals: Vec<u8> = (0..=reg).into_iter().map(|i| self.load_word(self.mem_addr_register + i as usize)).collect();

                for (i, val) in memory_vals.iter().enumerate() {
                    self.write_register(i as u8, *val);
                }

                self.increment_mem_addr_register(reg + 1);
            }
            _ => unreachable!("Instruction not supported: {:x?}{:x?}{:x?}{:x?}", a, b, c, d),
        }

        self.pc += 2;

        if self.delay > 0 {
            self.delay -= 1;
        }

        if self.sound > 0 {
            self.sound -= 1;
        }

        ProcessorState::Ready
    }
}

impl Default for Processor {
    fn default() -> Self {
        let mut memory = [0; 4096];
        memory[0..DIGIT_SPRITES.len()].copy_from_slice(&DIGIT_SPRITES);

        Processor {
            memory: memory,
            registers: [0; 16],
            mem_addr_register: 0,
            delay: 0,
            sound: 0,
            pc: 512,
            sp: 0,
            stack: [0; 16],
        }
    }
}
