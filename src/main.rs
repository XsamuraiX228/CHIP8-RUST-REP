use std::{error::Error};
use std::fs;
use rand::random;
use minifb::{Key, Window, WindowOptions};

const FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

#[allow(dead_code)]
#[derive(Debug)]
enum OpCode {
    // 0x0
    Clear,                          // 00E0 - Clear screen
    Return,                         // 00EE - Return from subroutine

    // 0x1
    Jump { nnn: u16 },             // 1nnn - Jump to address nnn

    // 0x2
    Call { nnn: u16 },             // 2nnn - Call subroutine at nnn

    // 0x3
    SE { vx: u8, kk: u8 },        // 3xkk - Skip if Vx == kk

    // 0x4
    SNE { vx: u8, kk: u8 },       // 4xkk - Skip if Vx != kk

    // 0x5
    Compare { vx: u8, vy: u8 },   // 5xy0 - Skip if Vx == Vy

    // 0x6
    SetRegister { x: u8, kk: u8 }, // 6xkk - Set Vx = kk

    // 0x7
    AddRegister { x: u8, kk: u8 }, // 7xkk - Add kk to Vx

    // 0x8 — семейство
    Mov  { vx: u8, vy: u8 },      // 8xy0 — Vx = Vy
    Or   { vx: u8, vy: u8 },      // 8xy1 — Vx |= Vy
    And  { vx: u8, vy: u8 },      // 8xy2 — Vx &= Vy
    Xor  { vx: u8, vy: u8 },      // 8xy3 — Vx ^= Vy
    AddReg { vx: u8, vy: u8 },    // 8xy4 — Vx += Vy, VF = carry
    Sub  { vx: u8, vy: u8 },      // 8xy5 — Vx -= Vy, VF = borrow
    Shr  { vx: u8 },              // 8xy6 — Vx >>= 1, VF = lsb
    Subn { vx: u8, vy: u8 },      // 8xy7 — Vx = Vy - Vx, VF = borrow
    Shl  { vx: u8 },              // 8xyE — Vx <<= 1, VF = msb

    // 0xA
    LoadI { nnn: u16 },            // Annn - Set I = nnn

    // 0xB
    JumpT { nnn: u16 },            // Bnnn - Jump to nnn + V0

    // 0xC
    RND { vx: u8, kk: u8 },       // Cxkk - Random number AND kk

    // 0xD
    Draw { vx: u8, vy: u8, n: u8 }, // Dxyn - Draw sprite

    // 0xE
    SKP { vx: u8 },               // Ex9E - Skip if key pressed
    SKNP { vx: u8 },              // ExA1 - Skip if key not pressed

    // 0xF
    GetDelay { vx: u8 },          // Fx07 - Get delay timer into Vx
    GetKey { vx: u8 },            // Fx0A - Wait for key press
    SetDelay { vx: u8 },          // Fx15 - Set delay timer
    SetSound { vx: u8 },          // Fx18 - Set sound timer
    AddI   { vx: u8 },            // Fx1E - Add Vx to I
    LoadSprite { vx: u8 },        // Fx29 - Load sprite for digit
    LoadBCD  { vx: u8 },          // Fx33 - BCD store
    StorReg  { vx: u8 },          // Fx55 - Store registers
    ReadReg  { vx: u8 },          // Fx65 - Read registers

    // Unknown opcode
    Unknown(u16),
}

impl OpCode {
    fn decode(code: u16) -> Self {
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
    /*
    fn info(&self) -> String {
        match self {
            OpCode::Clear => 
                "CLS — очистить экран".to_string(),
            OpCode::Return => 
                "RET — возврат из подпрограммы".to_string(),
            OpCode::Jump { nnn } => 
                format!("JP — прыжок на 0x{:03X}", nnn),
            OpCode::Call { nnn } => 
                format!("CALL — вызов 0x{:03X}", nnn),
            OpCode::SetRegister { x, kk } => 
                format!("LD — V{:X} = 0x{:02X}", x, kk),
            OpCode::AddRegister { x, kk } => 
                format!("ADD — V{:X} += 0x{:02X}", x, kk),
            OpCode::SE { vx, kk } => 
                format!("SE — пропустить если V{:X} == 0x{:02X}", vx, kk),
            OpCode::SNE { vx, kk } => 
                format!("SNE — пропустить если V{:X} != 0x{:02X}", vx, kk),
            OpCode::Compare { vx, vy } => 
                format!("SE — пропустить если V{:X} == V{:X}", vx, vy),
            OpCode::LoadI { nnn } => 
                format!("LD I — I = 0x{:03X}", nnn),
            OpCode::JumpT { nnn } => 
                format!("JP V0 — прыжок на 0x{:03X} + V0", nnn),
            OpCode::RND { vx, kk } => 
                format!("RND — V{:X} = random & 0x{:02X}", vx, kk),
            OpCode::Draw { vx, vy, n } => 
                format!("DRW — рисуем спрайт V{:X} V{:X} {} строк", vx, vy, n),
            OpCode::Mov  { vx, vy } => 
                format!("MOV — V{:X} = V{:X}", vx, vy),
            OpCode::Or   { vx, vy } => 
                format!("OR  — V{:X} |= V{:X}", vx, vy),
            OpCode::And  { vx, vy } => 
                format!("AND — V{:X} &= V{:X}", vx, vy),
            OpCode::Xor  { vx, vy } => 
                format!("XOR — V{:X} ^= V{:X}", vx, vy),
            OpCode::AddReg { vx, vy } => 
                format!("ADD — V{:X} += V{:X}", vx, vy),
            OpCode::Sub  { vx, vy } => 
                format!("SUB — V{:X} -= V{:X}", vx, vy),
            OpCode::Subn { vx, vy } => 
                format!("SUBN — V{:X} = V{:X} - V{:X}", vx, vy, vx),
            OpCode::Shr  { vx } => 
                format!("SHR — V{:X} >>= 1", vx),
            OpCode::Shl  { vx } => 
                format!("SHL — V{:X} <<= 1", vx),
            OpCode::Unknown(code) => 
                format!("??? — неизвестный 0x{:04X}", code),
            OpCode::SKP { vx } =>
                format!("SKP — пропустить если клавиша V{:X} нажата", vx),
            OpCode::SKNP { vx } =>
                format!("SKNP — пропустить если клавиша V{:X} не нажата", vx),
            OpCode::GetDelay { vx } =>
                format!("LD — V{:X} = delay_timer", vx),
            OpCode::SetDelay { vx } =>
                format!("LD — delay_timer = V{:X}", vx),
            OpCode::SetSound { vx } =>
                format!("LD — sound_timer = V{:X}", vx),
            OpCode::AddI { vx } =>
                format!("ADD — I += V{:X}", vx),
            OpCode::LoadSprite { vx } =>
                format!("LD F — I = спрайт цифры V{:X}", vx),
            OpCode::LoadBCD { vx } =>
                format!("LD B — BCD из V{:X} в память[I]", vx),
            OpCode::StorReg { vx } =>
                format!("LD [I] — сохранить V0-V{:X} в память", vx),
            OpCode::ReadReg { vx } =>
                format!("LD Vx — загрузить V0-V{:X} из памяти", vx),
            OpCode::GetKey { vx } =>
                format!("LD K — ждать клавишу → V{:X}", vx),
        }
    }
    */
}

#[allow(dead_code)]
#[derive(Debug)]
struct CHIP8 {
    memory: [u8; 4096],
    v: [u8; 16],
    stack: [u16; 16],
    pc: u16,
    sp: u8,
    idx_i: u16,           // регистр I
    display: [bool; 64 * 32],
    keypad: [bool; 16],
    delay_timer: u8,
    sound_timer: u8,
}

impl CHIP8 {
    fn new() -> Self {
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
    fn execute(&mut self, opcode: OpCode) {
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
    fn fetch(&mut self) -> u16 {
        let hi = self.memory[self.pc as usize] as u16;
        let lo = self.memory[self.pc as usize + 1] as u16;
        self.pc += 2;
        hi << 8 | lo
    }


    fn load_rom(&mut self, rom: &[u8]) {
        for (i, &byte) in rom.iter().enumerate() {
            self.memory[0x200 + i] = byte;
        }
    }
    /* 
    fn print_display(&self) {
        for y in 0..32 {
            for x in 0..64 {
                if self.display[y * 64 + x] {
                    print!("█");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
    */
}

/* 
fn get_opcode() -> Result<u16, Box<dyn Error>> {
    let mut opcode = String::new();
    io::stdin().read_line(&mut opcode)?;
    let trimmed = opcode.trim();

    let code = u16::from_str_radix(trimmed, 16)?;

    Ok(code)
}
*/

 
fn read_file(path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    Ok(fs::read(path)?)
}

fn map_keys(window: &Window, cpu: &mut CHIP8) {
    let keys = [
        (Key::X, 0x0), (Key::Key1, 0x1), (Key::Key2, 0x2), (Key::Key3, 0x3),
        (Key::Q, 0x4), (Key::W, 0x5),    (Key::E, 0x6),    (Key::A, 0x7),
        (Key::S, 0x8), (Key::D, 0x9),    (Key::Z, 0xA),    (Key::C, 0xB),
        (Key::Key4, 0xC), (Key::R, 0xD), (Key::F, 0xE),    (Key::V, 0xF),
    ];
    
    for (key, idx) in keys {
        cpu.keypad[idx] = window.is_key_down(key);
    }
}

use std::time::{Duration, Instant};

fn main() {
    let mut cpu = CHIP8::new();
    
    match read_file("C:/My_VS_code_Projects/Rust/new_project/src/Pong (1 player).ch8") {
        Ok(rom) => cpu.load_rom(&rom),
        Err(e) => println!("{}", e),
    }

    let mut window = Window::new(
        "CHIP-8", 64 * 10, 32 * 10, WindowOptions::default()
    ).unwrap();

    // 500Hz = каждые 2ms один опкод
    window.set_target_fps(60);
    let opcodes_per_frame = 15; // 10 опкодов за кадр = ~600Hz

    while window.is_open() && !window.is_key_down(Key::Escape) {
        map_keys(&window, &mut cpu);
        
        // выполняем несколько опкодов за кадр
        for _ in 0..opcodes_per_frame {
            let code = cpu.fetch();
            let opcode = OpCode::decode(code);
            cpu.execute(opcode);
        }

        // таймеры уменьшаются 60 раз в секунду
        if cpu.delay_timer > 0 { cpu.delay_timer -= 1; }
        if cpu.sound_timer > 0 { cpu.sound_timer -= 1; }

        let buffer: Vec<u32> = cpu.display.iter()
            .map(|&p| if p { 0xFFFFFFFF } else { 0x00000000 })
            .collect();
        window.update_with_buffer(&buffer, 64, 32).unwrap();
    }
}

/*
let mut cpu = CHIP8::new();
    // C:/My_VS_code_Projects/Rust/new_project/src
    let mut user_choice = String::new();
    println!("Would you like to run a ROM or enter opcodes manually?");
    println!("Type 'run' to load the ROM, or 'manual' to enter opcodes.");
    io::stdin().read_line(&mut user_choice).unwrap();

    if user_choice.trim() == "run" || user_choice.trim() == "Run" {
        match read_file("C:/My_VS_code_Projects/Rust/new_project/src/test_opcode.ch8") {
            Ok(rom) => {
                cpu.load_rom(&rom);
                loop {
                    let pc = cpu.pc;
                    let code = cpu.fetch();
                    let opcode = OpCode::decode(code);
                    println!("0x{:03X}: {}", pc, opcode.info());
                    cpu.execute(opcode);
                    if pc == cpu.pc {
                        println!("--- бесконечный цикл на 0x{:03X} ---", pc);
                        cpu.print_display();
                        break;
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    } else {
        loop {
            println!("Enter opcode: ");
            match get_opcode() {
                Ok(code) => {
                    let opcode = OpCode::decode(code);
                    cpu.execute(opcode);
                    println!("Current state of registers:/n {:?}", cpu.v);
                }
                Err(e) => println!("Ошибка: {}", e),
            }
            println!("Press Enter to continue, or type 'quit' or 'q' to exit");
            let mut usr_input = String::new();
            io::stdin().read_line(&mut usr_input).unwrap();
            if usr_input.trim() == "quit" || usr_input.trim() == "q" {
                break;
            }
        }
    }
*/