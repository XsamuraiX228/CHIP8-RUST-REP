use super::module::{ControlFlow, 
    DataOps, 
    MemoryOps, 
    Graphics, 
    Input, 
    Timers, 
    Instruction
};
use super::cpu::Emulator;
impl Emulator {
    pub fn cycle(&mut self) {
        let opcode = self.memory.read_word(self.cpu.get_pc() as usize);
        self.cpu.skip();

        let instruction = Instruction::decode(opcode);

        self.execute(instruction)
    }

    pub fn decrement_timers(&mut self) {
        self.cpu.decrement_timers();
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Control(op)  => self.ex_control(op),
            Instruction::Data(op)     => self.ex_data(op),
            Instruction::Memory(op)   => self.ex_memory(op),
            Instruction::Graphics(op) => self.ex_graphics(op),
            Instruction::Input(op)    => self.ex_input(op),
            Instruction::Timers(op)   => self.ex_timers(op),
            Instruction::Unknown(op)  => println!("Unknown: {:#04X}", op),
        }
    }
}

impl Emulator {
    pub fn get_v_pair(&self, vx: u8, vy: u8) -> (u16, u16) {
        let val_x = self.cpu.get_v(vx as usize) as u16;
        let val_y = self.cpu.get_v(vy as usize) as u16;
        (val_x, val_y)
    }
    pub fn ex_control(&mut self, op: ControlFlow) {
        match op {
            ControlFlow::Return => {
                let addr = self.cpu.stack.pop();
                self.cpu.set_pc(addr);
            }
            ControlFlow::Jump { nnn } => {
                self.cpu.set_pc(nnn)
            }
            ControlFlow::Call { nnn } => {
                self.cpu.stack.push(self.cpu.get_pc());
                self.cpu.set_pc(nnn);
            }
            ControlFlow::SkipIfEq { vx, kk } => {
                if self.cpu.get_v(vx as usize) == kk {
                    self.cpu.skip();
                }
            }
            ControlFlow::SkipIfNotEq { vx, kk } => {
                if self.cpu.get_v(vx as usize) != kk {
                    self.cpu.skip();
                }
            }
            ControlFlow::SkipIfRegEq { vx, vy } => {
                if self.cpu.get_v(vx as usize) == self.cpu.get_v(vy as usize) {
                    self.cpu.skip();
                }
            }
            ControlFlow::SkipIfRegNeq { vx, vy } => {
                if self.cpu.get_v(vx as usize) != self.cpu.get_v(vy as usize) {
                    self.cpu.skip();
                }
            }
            ControlFlow::JumpReg { nnn } => {
                self.cpu.set_pc(nnn + self.cpu.get_v(0) as u16);
            }
        }
    }
    pub fn ex_data(&mut self, op: DataOps) {
        match op {
            // Assignment
            DataOps::LoadImm { vx, kk } => {
                // self.v[vx as usize] = kk;
                self.cpu.set_v(vx as usize, kk);
            }
            DataOps::Mov { vx, vy } => {
                let val = self.cpu.get_v(vy as usize);
                self.cpu.set_v(vx as usize, val);
            }
            // Arythmetic operations
            DataOps::AddImm { vx, kk } => {
                let val = self.cpu.get_v(vx as usize).wrapping_add(kk);
                self.cpu.set_v(vx as usize, val);
            }
            DataOps::Add { vx, vy } => {
                let val = self.cpu.get_v(vx as usize) as u16 + self.cpu.get_v(vy as usize) as u16;
                self.cpu.set_v(vx as usize, val as u8);
                self.cpu.set_v(0xF, if val > 0xFF {1} else {0});
            }
            DataOps::Sub { vx, vy } => {
                let (val_x, val_y) = self.get_v_pair(vx, vy);
                self.cpu.set_v(vx as usize, val_x.wrapping_sub(val_y) as u8);
                self.cpu.set_v(0xF, if val_x >= val_y {1} else  {0});
            }
            DataOps::Subn { vx, vy } => {
                let (val_x, val_y) = self.get_v_pair(vx, vy);
                self.cpu.set_v(vx as usize, val_y.wrapping_sub(val_x) as u8);
                self.cpu.set_v(0xF, if val_y >= val_x {1} else  {0});
            }
            // Побитовы
            DataOps::Or { vx, vy } => {
                let val = self.cpu.get_v(vx as usize) | self.cpu.get_v(vy as usize);
                self.cpu.set_v(vx as usize, val);
            }
            DataOps::And { vx, vy } => {
                let val = self.cpu.get_v(vx as usize) & self.cpu.get_v(vy as usize);
                self.cpu.set_v(vx as usize, val);
            }
            DataOps::Xor { vx, vy } => {
                let val = self.cpu.get_v(vx as usize) ^ self.cpu.get_v(vy as usize);
                self.cpu.set_v(vx as usize, val);
            }
            // Shifts
            DataOps::Shr { vx } => {
                let lsb = self.cpu.get_v(vx as usize) & 0x1;
                let val = self.cpu.get_v(vx as usize) >> 1;
                self.cpu.set_v(vx as usize, val);
                self.cpu.set_v(0xF, lsb);
            }
            DataOps::Shl { vx } => {
                let msb = (self.cpu.get_v(vx as usize) >> 7) & 0x1;
                let val = self.cpu.get_v(vx as usize) << 1;
                self.cpu.set_v(vx as usize, val);
                self.cpu.set_v(0xF, msb);
            }
            // Random numbers
            DataOps::Rand { vx, kk } => {
                let rnd: u8 = rand::random();
                self.cpu.set_v(vx as usize, rnd & kk);
            }
        }
    }
    pub fn ex_memory(&mut self, op: MemoryOps) {
        match op {
            MemoryOps::SetI { nnn } => {
                self.cpu.set_i(nnn);
            }
            MemoryOps::AddI { vx } => {
                let curr_i = self.cpu.get_i();
                self.cpu.set_i(curr_i + self.cpu.get_v(vx as usize) as u16);
            }
            MemoryOps::LoadSprite { vx } => {
                let digit = self.cpu.get_v(vx as usize);
                self.cpu.set_i(digit as u16 * 5);
            }
            MemoryOps::StoreBcd { vx } => {
                let i = self.cpu.get_i() as usize;
                let value = self.cpu.get_v(vx as usize);
                self.memory.write(i, value / 100);
                self.memory.write(i + 1, (value / 10) % 10);
                self.memory.write(i + 2, value % 10);
            }
            MemoryOps::StoreRegs { vx } => {
                let i = self.cpu.get_i();
                let regs = &self.cpu.registers.v[..=vx as usize];
                self.memory.write_bytes(i.into(), &regs);
            }
            MemoryOps::ReadRegs { vx } => {
                let i = self.cpu.get_i() as usize;
                for idx in 0..=vx as usize {
                    let val = self.memory.read(idx + i);
                    self.cpu.set_v(idx, val);
                }
            }
        }
    }
    pub fn ex_graphics(&mut self, op: Graphics) {
        match op {
            Graphics::Clear => {
                self.display.clear();
            }
            Graphics::Draw { vx, vy, n } => {
                let (cord_x, cord_y) = self.get_v_pair(vx, vy);
                let i = self.cpu.get_i() as usize;
                let sprite = self.memory.read_bytes(i, n as usize).to_vec();
                let collision = self.display.drawing(&sprite, cord_x as usize, cord_y as usize);
                self.cpu.set_v(0xF, collision as u8);
            }
        }
    }
    pub fn ex_input(&mut self, op: Input) {
        match op {
            Input::SkipPressed { vx } => {
                let key = self.cpu.get_v(vx as usize);
                if self.keypad.is_key_pressed(key) {
                    self.cpu.skip();
                }
            }
            Input::SkipNotPressed { vx } => {
                let key = self.cpu.get_v(vx as usize);
                if !self.keypad.is_key_pressed(key) {
                    self.cpu.skip();
                }
            }
            Input::WaitKey { vx } => {
                if !self.keypad.is_any_key_pressed() {
                    self.cpu.set_pc(self.cpu.get_pc() - 2);
                } else {
                    let key = self.keypad.get_last_pressed_key();
                    self.cpu.set_v(vx as usize, key)
                }
            }
        }
    }
    pub fn ex_timers(&mut self, op: Timers) {
        match op {
            Timers::GetDelay { vx } => {
                let val = self.cpu.get_dl_timer();
                self.cpu.set_v(vx as usize, val);
            }
            Timers::SetDelay { vx } => {
                let val = self.cpu.get_v(vx as usize);
                self.cpu.set_dl_time(val);
            }
            Timers::SetSound { vx } => {
                let val = self.cpu.get_v(vx as usize);
                self.cpu.set_sound(val);
            }
        }
    }
}



