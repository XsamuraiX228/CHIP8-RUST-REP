use super::module::{CHIP8, OpCode};
use rand::random;


impl CHIP8 {
    pub fn execute(&mut self, opcode: OpCode) {
        match opcode {
            // 0x00E0 - Clear screen
            OpCode::Clear {} => {
                println!("Clear the screen");
                self.display = [false; 64 * 32];
            },

            // 0x00EE - Return from subroutine
            OpCode::Return => {
                if self.sp == 0 {
                    panic!("sp = 0");
                } else {
                    self.sp -= 1;
                }
                self.pc = self.stack[self.sp as usize];
            },

            // 0x1nnn - Jump to address nnn
            OpCode::Jump { nnn } => {
                self.pc = nnn
            },

            // 0x2nnn - Call subroutine at nnn
            OpCode::Call { nnn } => {
                if self.sp >= 16 {
                    panic!("Move out of the stack");
                } else {
                    self.stack[self.sp as usize] = self.pc;
                    self.sp += 1
                }
                self.pc = nnn
            },

            // 0x3xkk - Skip if Vx == kk
            OpCode::SE { vx, kk } => {
                if self.v[vx as usize] == kk {
                    self.pc += 2
                }
            },

            // 0x4xkk - Skip if Vx != kk
            OpCode::SNE { vx, kk } => {
                if self.v[vx as usize] != kk {
                    self.pc += 2
                }
            },

            // 0x5xy0 - Skip if Vx == Vy
            OpCode::Compare { vx, vy } => {
                if self.v[vx as usize] == self.v[vy as usize] {
                    self.pc += 2
                }
            },

            // 0x6xkk - Set Vx = kk
            OpCode::SetRegister { x, kk } => {
                self.v[x as usize] = kk;
            },

            // 0x7xkk - Add kk to Vx
            OpCode::AddRegister { x, kk } => {
                self.v[x as usize] = self.v[x as usize].wrapping_add(kk);
            },

            // 0x8xy0 - Set Vx = Vy
            OpCode::Mov { vx, vy } => {
                self.v[vx as usize] = self.v[vy as usize]
            },

            // 0x8xy1 - Vx = Vx OR Vy
            OpCode::Or { vx, vy } => {
                self.v[vx as usize] |= self.v[vy as usize]
            },

            // 0x8xy2 - Vx = Vx AND Vy
            OpCode::And { vx, vy } => {
                self.v[vx as usize] &= self.v[vy as usize]
            },

            // 0x8xy3 - Vx = Vx XOR Vy
            OpCode::Xor { vx, vy } => {
                self.v[vx as usize] ^= self.v[vy as usize]
            },

            // 0x8xy4 - Add Vy to Vx (with carry)
            OpCode::AddReg { vx, vy } => {
                let result = self.v[vx as usize] as u16 + self.v[vy as usize] as u16;
                self.v[0xF] = if result > 0xFF { 1 } else { 0 };
                self.v[vx as usize] = result as u8;
            },

            // 0x8xy5 - Subtract Vy from Vx (Vx = Vx - Vy)
            OpCode::Sub { vx, vy } => {
                let vx_val = self.v[vx as usize];
                let vy_val = self.v[vy as usize];

                self.v[0xF] = if vx_val > vy_val { 1 } else { 0 };
                self.v[vx as usize] = vx_val.wrapping_sub(vy_val);
            },

            // 0x8xy6 - Shift Vx right by 1
            OpCode::Shr { vx } => {
                let lsb = self.v[vx as usize] & 0x1;
                self.v[0xF] = lsb;
                self.v[vx as usize] >>= 1
            },

            // 0x8xy7 - Subtract Vx from Vy (Vx = Vy - Vx)
            OpCode::Subn { vx, vy } => {
                let vx_val = self.v[vx as usize];
                let vy_val = self.v[vy as usize];

                self.v[0xF] = if vy_val > vx_val { 1 } else { 0 };
                self.v[vx as usize] = vy_val.wrapping_sub(vx_val);
            },

            // 0x8xyE - Shift Vx left by 1
            OpCode::Shl { vx } => {
                let msb = self.v[vx as usize] & 0x80;
                self.v[0xF] = msb >> 7;
                self.v[vx as usize] <<= 1;
            },

            // 0xAnnn - Set I = nnn
            OpCode::LoadI { nnn } => {
                self.idx_i = nnn
            },

            // 0xBnnn - Jump to nnn + V0
            OpCode::JumpT { nnn } => {
                self.pc = nnn + self.v[0] as u16
            },

            // 0xCxkk - Random number AND kk
            OpCode::RND { vx, kk } => {
                let rnd: u8 = random();
                self.v[vx as usize] = rnd & kk;
            },

            // 0xDxyn - Draw sprite
            OpCode::Draw { vx, vy, n } => {
                // 1. берём координаты из регистров + wrap around
                let x = self.v[vx as usize] as usize % 64;
                let y = self.v[vy as usize] as usize % 32;
                
                // 2. сбрасываем флаг коллизии
                self.v[0xF] = 0;
                
                // 3. идём по строкам спрайта
                for row in 0..n as usize {
                    let byte = self.memory[self.idx_i as usize + row];
                    
                    // 4. идём по битам каждой строки
                    for col in 0..8 { 
                        let pixel = (byte >> (7 - col)) & 1;
                        if pixel == 1 {
                            let idx = (y + row) % 32 * 64 + (x + col) % 64;
                            
                            // 5. коллизия — пиксель уже горел
                            if self.display[idx] {
                                self.v[0xF] = 1;
                            }
                            
                            // 6. XOR — переключаем пиксель
                            self.display[idx] ^= true;
                        }
                    }
                }
            },

            OpCode::SKP { vx } => {
                let key = self.v[vx as usize] as usize;
                if self.keypad[key] {
                    self.pc += 2
                }
            }

            OpCode::SKNP { vx } => {
                let key = self.v[vx as usize] as usize;
                if !self.keypad[key] {
                    self.pc += 2
                }
            }

            OpCode::GetDelay { vx } => {
                self.v[vx as usize] = self.delay_timer
            }

            OpCode::SetDelay { vx } => {
                self.delay_timer = self.v[vx as usize]
            }

            OpCode::SetSound { vx } => {
                self.sound_timer = self.v[vx as usize]
            }

            OpCode::AddI { vx } => {
                self.idx_i += self.v[vx as usize] as u16
            }

            OpCode::LoadSprite { vx } => {
                let digit = self.v[vx as usize] as u16;
                self.idx_i = digit * 5
            }

            OpCode::LoadBCD { vx } => {
                let val = self.v[vx as usize];
                self.memory[self.idx_i as usize] = val / 100;
                self.memory[self.idx_i as usize + 1] = (val / 10) % 10;
                self.memory[self.idx_i as usize + 2] = val % 10
            }

            OpCode::StorReg { vx } => {
                for i in 0..=vx as usize {
                    self.memory[self.idx_i as usize + i] = self.v[i]; 
                }
            }

            OpCode::ReadReg { vx } => {
                for i in 0..=vx as usize {
                    self.v[i] = self.memory[self.idx_i as usize + i]
                }
            }

            OpCode::GetKey { vx } => {
                let mut pressed = false;
                for i in 0..16 {
                    if self.keypad[i] {
                        self.v[vx as usize] = i as u8;
                        pressed = true;
                        break;
                    }
                }
                if !pressed {
                    self.pc -= 2
                }
            }


            // Unknown opcode handler
            other => {
                println!("Unknown opcode: {:?}", other)
            }
        }
    }   
}

