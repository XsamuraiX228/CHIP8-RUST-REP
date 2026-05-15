use super::module::{CHIP8, OpCode};
use super::fonts::FONTSET;
impl OpCode {
    pub fn decode(code: u16) -> Self {
        match (code & 0xF000) >> 12 {
            0x0 => match code & 0x00FF {
                0xE0 => OpCode::Clear,
                0xEE => OpCode::Return,
                _ => OpCode::Unknown(code),
            },
            0x1 => OpCode::Jump {
                nnn: code & 0x0FFF
            },
            0x2 => OpCode::Call { 
                nnn: code & 0x0FFF 
            },
            0x3 => OpCode::SE { 
                vx: ((code & 0x0F00) >> 8) as u8, 
                kk: ((code & 0x00FF)) as u8, 
            },
            0x4 => OpCode::SNE { 
                vx: ((code & 0x0F00) >> 8) as u8, 
                kk: ((code & 0x00FF)) as u8, 
            },
            0x5 => OpCode::Compare { 
                vx: ((code & 0x0F00) >> 8) as u8, 
                vy: ((code & 0x00F0) >> 4) as u8,
            },
            0x6 => OpCode::SetRegister {
                x: ((code & 0x0F00) >> 8) as u8,
                kk: ((code & 0x00FF)) as u8,
            },
            0x7 => OpCode::AddRegister { 
                x: ((code & 0x0F00) >> 8) as u8, 
                kk: ((code & 0x00FF)) as u8 ,
            },
            0x8 => {
                let vx = ((code & 0x0F00) >> 8) as u8;
                let vy = ((code & 0x00F0) >> 4) as u8;

                match code & 0x000F {
                    0x0 => OpCode::Mov  { vx, vy },
                    0x1 => OpCode::Or   { vx, vy },
                    0x2 => OpCode::And  { vx, vy },
                    0x3 => OpCode::Xor  { vx, vy },
                    0x4 => OpCode::AddReg { vx, vy },
                    0x5 => OpCode::Sub  { vx, vy },
                    0x6 => OpCode::Shr  { vx },
                    0x7 => OpCode::Subn { vx, vy },
                    0xE => OpCode::Shl  { vx },
                    _   => OpCode::Unknown(code),
                }
            }
            0xA => OpCode::LoadI { 
                nnn: code & 0x0FFF 
            },
            0xB => OpCode::JumpT { 
                nnn: code & 0x0FFF 
            },
            0xC => OpCode::RND { 
                vx: ((code & 0x0F00) >> 8) as u8, 
                kk: ((code & 0x00FF)) as u8 
            },
            0xD => OpCode::Draw {
                vx: ((code & 0x0F00) >> 8) as u8,
                vy: ((code & 0x00F0) >> 4) as u8,
                n:  (code & 0x000F) as u8,
            },
            0xE => {
                let vx = ((code & 0x0F00) >> 8) as u8;
                match code & 0x00FF {
                    0x9E => OpCode::SKP { vx: vx },
                    0xA1 => OpCode::SKNP { vx: vx },
                    _ => OpCode::Unknown(code),
                }
            }
            0xF => {
                let vx = ((code & 0x0F00) >> 8) as u8;
                match code & 0x00FF {
                    0x07 => OpCode::GetDelay { vx },
                    0x0A => OpCode::GetKey { vx },
                    0x15 => OpCode::SetDelay { vx },
                    0x18 => OpCode::SetSound { vx },
                    0x1E => OpCode::AddI { vx },
                    0x29 => OpCode::LoadSprite { vx },
                    0x33 => OpCode::LoadBCD { vx },
                    0x55 => OpCode::StorReg { vx },
                    0x65 => OpCode::ReadReg { vx },
                    _    => OpCode::Unknown(code),
                }
            }
            _ => OpCode::Unknown(code),
        }
    }
}

impl CHIP8 {
    pub fn new() -> Self {
        let mut chip8 = CHIP8 {
            memory: [0; 4096],
            v: [0; 16],
            stack: [0; 16],
            pc: 0x200,
            sp: 0,
            idx_i: 0,
            display: [false; 64 * 32],
            keypad: [false; 16],
            delay_timer: 0,
            sound_timer: 0,
        };
        chip8.memory[..80].copy_from_slice(&FONTSET);
        chip8
    }
    pub fn fetch(&mut self) -> u16 {
        let hi = self.memory[self.pc as usize] as u16;
        let lo = self.memory[self.pc as usize + 1] as u16;
        self.pc += 2;
        hi << 8 | lo
    }


    pub fn load_rom(&mut self, rom: &[u8]) {
        for (i, &byte) in rom.iter().enumerate() {
            self.memory[0x200 + i] = byte;
        }
    }
}
