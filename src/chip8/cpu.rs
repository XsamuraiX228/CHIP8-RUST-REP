
use super::module::Versions;
use super::memory::Memory;
use super::display::Display;
use super::keypad::Keypad;
pub struct Registers {
    pub v: [u8; 16],
    idx_i: u16,
    pc: u16,
}

impl Registers {
    fn new() -> Registers {
        Registers {
            v: [0; 16],
            idx_i: 0,
            pc: 0x200,
        }
    }
}

pub struct Timers {
    pub delay_timer: u8,
    pub sound_timer: u8,
}

impl Timers {
    fn new() -> Timers {
        Timers {
            delay_timer: 0,
            sound_timer: 0,
        }
    }
}

pub struct Stack {
    pub stack: [u16; 16],
    sp: usize,
}

impl Stack {
    fn new() -> Stack {
        Stack {
            stack: [0; 16],
            sp: 0,
        }
    }

    pub fn push(&mut self, value: u16) {
        if self.sp >= 16 {
            panic!("Stack overflow");
        }
        self.stack[self.sp] = value;
        self.sp += 1;
    }

    pub fn pop(&mut self) -> u16 {
        if self.sp == 0 {
            panic!("Stack underflow");
        }
        self.sp -= 1;
        self.stack[self.sp]
    }
}

pub struct CPU {
    pub registers: Registers,
    pub timers: Timers,
    pub stack: Stack,
    pub high_res: bool,
    pub rpl: [u8; 8],
    pub mode: Versions,
}

impl CPU {
    pub fn new(mode: Versions) -> CPU {
        CPU {
            registers: Registers::new(),
            timers: Timers::new(),
            stack: Stack::new(),
            high_res: false,
            rpl: [0; 8],
            mode,
        }
    }
    pub fn get_pc(&self) -> u16 {
        self.registers.pc
    }
    pub fn set_pc(&mut self, value: u16) {
        self.registers.pc = value;
    }
    pub fn skip(&mut self) {
        self.registers.pc += 2;
    }
    pub fn get_v(&self, x: usize) -> u8 {
        self.registers.v[x]
    }
    pub fn set_v(&mut self, x: usize, value: u8) {
        self.registers.v[x] = value;
    }
    pub fn get_i(&self) -> u16 {
        self.registers.idx_i
    }
    pub fn set_i(&mut self, value: u16) {
        self.registers.idx_i = value;
    }
    pub fn get_dl_timer(&self) -> u8 {
        self.timers.delay_timer
    }
    pub fn set_dl_time(&mut self, val: u8) {
        self.timers.delay_timer = val
    }
    pub fn set_sound(&mut self, val: u8) { 
        self.timers.sound_timer = val; 
    }
    pub fn decrement_timers(&mut self) {
        if self.timers.delay_timer > 0 { self.timers.delay_timer -= 1; }
        if self.timers.sound_timer > 0 { self.timers.sound_timer -= 1; }
    }
}

pub struct Emulator {
    pub cpu: CPU,
    pub memory: Memory,
    pub keypad: Keypad,
    pub display: Display,
}

impl Emulator {
    pub fn new(mode: Versions) -> Self {
        Self {
            cpu: CPU::new(mode),
            memory: Memory::new(),
            display: Display::new(),
            keypad: Keypad::new(),
        }
    }
    pub fn load_rom(&mut self, rom: &[u8]) {
        self.memory.load_memory(rom);
    }
    pub fn press_key(&mut self, key: u8) {
        self.keypad.press_key(key);
    }
    pub fn release_key(&mut self, key: u8) {
        self.keypad.release_key(key);
    }
    pub fn render(&self) -> [u32; 64 * 32] {
        self.display.render()
    }
}