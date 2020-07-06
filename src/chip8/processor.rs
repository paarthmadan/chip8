use super::driver::Display;
use rand::Rng;
use std::fs;
use std::io;
use ProgramCounter::*;

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
    Continue,
    BlockForIO,
}

enum ProgramCounter {
    Increment,
    Skip(bool),
    Jump(u16),
    Halt,
}

pub const DIGIT_SPRITES: [u8; 5 * 16] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80, 0xF0, 0xF0,
    0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0, 0xF0, 0x80,
    0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90, 0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0,
    0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80,
    0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0, 0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
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

    fn draw_sprite(&mut self, x: u8, y: u8, sprite: u8, display: &mut Display) -> ProgramCounter {
        let sprite = self.load_sprite(sprite);
        let flag = display.write_buffer(self.read_register(x), self.read_register(y), &sprite);

        self.write_flag(flag as u8)
    }


    fn return_from_routine(&mut self) -> ProgramCounter {
        self.sp -= 1;

        Jump(self.stack[self.sp] as u16)
    }

    fn call(&mut self, addr: u16) -> ProgramCounter {
        self.stack[self.sp] = self.pc + 2;
        self.sp += 1;

        Jump(addr)
    }

    fn read_register(&self, reg: u8) -> u8 {
        self.registers[reg as usize]
    }

    fn write_register(&mut self, reg: u8, value: u8) -> ProgramCounter {
        self.registers[reg as usize] = value;
        Increment
    }

    fn write_mem_addr_register(&mut self, value: u16) -> ProgramCounter {
        self.mem_addr_register = value as usize;
        Increment
    }

    fn write_delay(&mut self, reg: u8) -> ProgramCounter {
        self.delay = self.read_register(reg) as u8;
        Increment
    }

    fn write_sound(&mut self, reg: u8) -> ProgramCounter {
        self.sound = self.read_register(reg) as u8;
        Increment
    }

    fn increment_mem_addr_register(&mut self, incr: u8) -> ProgramCounter {
        self.mem_addr_register += incr as usize;
        Increment
    }

    fn write_flag(&mut self, value: u8) -> ProgramCounter {
        self.write_register(0xF, value);
        Increment
    }

    fn arithmetic_instr(&mut self, dst: u8, src: u8, flag: u8) -> ProgramCounter {
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
            }
            5 => {
                let (diff, borrow) = dst_val.overflowing_sub(src_val);
                self.write_flag(!borrow as u8);

                diff
            }
            6 => {
                let lsb = 0x01 & dst_val;
                self.write_flag(lsb);

                dst_val >> 1
            }
            7 => {
                let (diff, borrow) = src_val.overflowing_sub(dst_val);
                self.write_flag(!borrow as u8);

                diff
            }
            0xE => {
                let msb = dst_val >> 7;
                self.write_flag(msb);

                dst_val << 1
            }
            _ => unreachable!{}
        };
        self.write_register(dst, res)
    }

    fn generate_random(&mut self, reg: u8, lower_word: u8) -> ProgramCounter {
        let mut rng = rand::thread_rng();
        let rand: u8 = rng.gen();

        self.write_register(reg, rand & lower_word)
    }

    fn write_bcd(&mut self, reg: u8) -> ProgramCounter {
        let dec = self.read_register(reg);

        self.store_word(0, dec / 100);
        self.store_word(1, (dec % 100) / 10);
        self.store_word(2, dec % 10);

        Increment
    }

    fn wait_for_keypress(&mut self, reg: u8, input: &[bool; 16]) -> ProgramCounter {
      match input.iter().position(|&k| k) {
            Some(index) => {
                self.write_register(reg, index as u8)
            }
            None => Halt,
      }
    }

    fn store_registers(&mut self, reg: u8) -> ProgramCounter {
        let register_vals: Vec<u8> = (0..=reg).map(|reg| self.read_register(reg)).collect();
        for (i, val) in register_vals.iter().enumerate() {
            self.store_word(i, *val);
        }

        self.increment_mem_addr_register(reg + 1)
    }

    fn load_registers(&mut self, reg: u8) -> ProgramCounter {
        let memory_vals: Vec<u8> = (0..=reg)
            .map(|i| self.load_word(self.mem_addr_register + i as usize))
            .collect();

        for (i, val) in memory_vals.iter().enumerate() {
            self.write_register(i as u8, *val);
        }

        self.increment_mem_addr_register(reg + 1)
    }

    fn clear_display(&mut self, display: &mut Display) -> ProgramCounter {
        display.clear_buffer();
        Increment
    }

    pub fn load_rom(&mut self, rom: &str) -> Result<(), io::Error> {
        let bytes = fs::read(rom)?;

        let prog_end = Processor::MEM_START + bytes.len();

        if prog_end > 4096 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "ROM exceeds 4096 bytes",
            ));
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

        let addr: u16 = ((b as u16) << 8) + lower_word as u16;

        // Instruction Execute
        let pc_state: ProgramCounter = match (a, b, c, d) {
            (0x0, 0x0, 0xE, 0x0)   => self.clear_display(display),
            (0x0, 0x0, 0xE, 0xE)   => self.return_from_routine(),
            (0x0, _, _, _)         => unimplemented! {},
            (0x1, _, _, _)         => Jump(addr),
            (0x2, _, _, _)         => self.call(addr),
            (0x3, reg, _, _)       => Skip(self.read_register(reg) == lower_word),
            (0x4, reg, _, _)       => Skip(self.read_register(reg) != lower_word),
            (0x5, reg1, reg2, _)   => Skip(self.read_register(reg1) == self.read_register(reg2)),
            (0x6, dst, _, _)       => self.write_register(dst, lower_word),
            (0x7, dst, _, _)       => self.write_register(dst, self.read_register(dst).overflowing_add(lower_word).0),
            (0x8, dst, src, flag)  => self.arithmetic_instr(dst, src, flag),
            (0x9, reg1, reg2, 0x0) => Skip(self.read_register(reg1) != self.read_register(reg2)),
            (0xA, _, _, _)         => self.write_mem_addr_register(addr),
            (0xB, _, _, _)         => Jump(addr + self.read_register(0) as u16),
            (0xC, reg, _, _)       => self.generate_random(reg, lower_word),
            (0xD, x, y, n)         => self.draw_sprite(x, y, n, display),
            (0xE, reg, 0x9, 0xE)   => Skip(input[self.read_register(reg) as usize]),
            (0xE, reg, 0xA, 0x1)   => Skip(!input[self.read_register(reg) as usize]),
            (0xF, reg, 0x0, 0x7)   => self.write_register(reg, self.delay),
            (0xF, reg, 0x0, 0xA)   => self.wait_for_keypress(reg, input),
            (0xF, reg, 0x1, 0x5)   => self.write_delay(reg),
            (0xF, reg, 0x1, 0x8)   => self.write_sound(self.read_register(reg) as u8),
            (0xF, reg, 0x1, 0xE)   => self.increment_mem_addr_register(self.read_register(reg)),
            (0xF, reg, 0x2, 0x9)   => self.increment_mem_addr_register(5 * self.read_register(reg)),
            (0xF, reg, 0x3, 0x3)   => self.write_bcd(reg),
            (0xF, reg, 0x5, 0x5)   => self.store_registers(reg),
            (0xF, reg, 0x6, 0x5)   => self.load_registers(reg),
            _                      => unreachable!("{:x?}{:x?}{:x?}{:x?}", a, b, c, d),
        };

        match pc_state {
            Increment => self.pc += 2,
            Jump(addr) => self.pc = addr as usize,
            Skip(predicate) => self.pc += if predicate { 4 } else { 2 },
            Halt => return ProcessorState::BlockForIO,
        }

        if self.delay > 0 { self.delay -= 1; }
        if self.sound > 0 { self.sound -= 1; }

        ProcessorState::Continue
    }
}

impl Default for Processor {
    fn default() -> Self {
        let mut memory = [0; 4096];
        memory[0..DIGIT_SPRITES.len()].copy_from_slice(&DIGIT_SPRITES);

        Processor {
            memory,
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
