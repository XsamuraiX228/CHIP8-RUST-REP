use std::{error::Error, io};
use std::fs;
use rand::random;

#[allow(dead_code)]
#[derive(Debug)]
enum OpCode {
    Clear,
    Jump { nnn: u16 },
    SetRegister { x: u8, kk: u8 },
    AddRegister { x: u8, kk: u8 },
    Call { nnn: u16 },
    Return,
    Unknown(u16),
    Draw { vx: u8, vy: u8, n: u8 },
    SE { vx: u8, kk: u8 },
    SNE { vx: u8, kk: u8 },
    Compare { vx: u8, vy: u8 },
    LoadI { nnn: u16 },
    JumpT { nnn: u16 },
    RND { vx: u8, kk: u8 },

    // семейство 8
    Mov  { vx: u8, vy: u8 },   // 8xy0 — Vx = Vy
    Or   { vx: u8, vy: u8 },   // 8xy1 — Vx |= Vy
    And  { vx: u8, vy: u8 },   // 8xy2 — Vx &= Vy
    Xor  { vx: u8, vy: u8 },   // 8xy3 — Vx ^= Vy
    AddReg { vx: u8, vy: u8 }, // 8xy4 — Vx += Vy, VF = carry
    Sub  { vx: u8, vy: u8 },   // 8xy5 — Vx -= Vy, VF = borrow
    Shr  { vx: u8 },           // 8xy6 — Vx >>= 1, VF = старший бит
    Subn { vx: u8, vy: u8 },   // 8xy7 — Vx = Vy - Vx, VF = borrow
    Shl  { vx: u8 },           // 8xyE — Vx <<= 1, VF = старший бит
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
            _ => OpCode::Unknown(code),
        }
    }
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
        }
    }
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
    fn execute(&mut self, opcode: OpCode) {
        match opcode {
            OpCode::Clear {} => {
                println!("Clear the sceen");
                self.display = [false; 64 * 32];
            },

            // 0x3705 -> skip if V7 == 5
            OpCode::SE { vx, kk } => {
                if self.v[vx as usize] == kk {
                    self.pc += 2
                }
            }
            OpCode::SNE { vx, kk } => {
                if self.v[vx as usize] != kk {
                    self.pc += 2
                }
            }
            OpCode::Compare { vx, vy } => {
                if self.v[vx as usize] == self.v[vy as usize] {
                    self.pc += 2
                }
            }
            OpCode::LoadI { nnn } => {
                self.idx_i = nnn
            }
            OpCode::Jump { nnn } => {
                self.pc = nnn
            },
            OpCode::JumpT { nnn } => {
                self.pc = nnn + self.v[0] as u16
            }
            OpCode::RND { vx, kk } => {
                let rnd: u8 = random();
                self.v[vx as usize] = rnd & kk;
            }
            OpCode::Call { nnn } => {
                if self.sp >= 16 {
                    panic!("Move out of the stack");
                } else {
                    self.stack[self.sp as usize] = self.pc;
                    self.sp += 1
                }
                self.pc = nnn
            },
            OpCode::Return => {
                if self.sp == 0 {panic!("sp = 0");} 
                else {self.sp -= 1;}
                self.pc = self.stack[self.sp as usize];
            },
            OpCode::SetRegister { x, kk } => {
                self.v[x as usize] = kk;
            },
            OpCode::AddRegister { x, kk } => {
                self.v[x as usize] = self.v[x as usize].wrapping_add(kk);
            }
            // 8 Family
            OpCode::Mov { vx, vy } => {
                self.v[vx as usize] = vy
            }
            OpCode::Or { vx, vy } => {
                self.v[vx as usize] |= self.v[vy as usize]
            }
            OpCode::And { vx, vy } => {
                self.v[vx as usize] &= self.v[vy as usize]
            }
            OpCode::Xor { vx, vy } => {
                self.v[vx as usize] ^= self.v[vy as usize]
            }
            OpCode::AddReg { vx, vy } => {
                let result = self.v[vx as usize] as u16 + self.v[vy as usize] as u16;
                self.v[0xF] = if result > 0xFF { 1 } else { 0 };
                self.v[vx as usize] = result as u8;
            }
            OpCode::Sub { vx, vy } => {
                let vx_val = self.v[vx as usize];
                let vy_val = self.v[vy as usize];

                self.v[0xF] = if vx_val > vy_val { 1 } else { 0 };
                self.v[vx as usize] = vx_val.wrapping_sub(vy_val);
            }
            OpCode::Shr { vx } => {
                let lsb = self.v[vx as usize] & 0x1;
                self.v[0xF] = lsb;
                self.v[vx as usize] >>= 1
            }
            OpCode::Subn { vx, vy } => {
                let vx_val = self.v[vx as usize];
                let vy_val = self.v[vy as usize];

                self.v[0xF] = if vy_val > vx_val { 1 } else { 0 };
                self.v[vx as usize] = vx_val.wrapping_sub(vy_val);
            }
            OpCode::Shl { vx } => {
                let rsb = self.v[vx as usize] &0x80;
                self.v[0xF] = rsb >> 7;
                self.v[vx as usize] <<= 1;
            }
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
                        // 0x00000001
                        // 0x00000010
                        // 0x00000100
                        // ...
                        // 0x10000000
                        if pixel == 1 {
                            let sx = (x + col) % 64;  // wrap around
                            let sy = (y + row) % 32;
                            let idx = sy * 64 + sx;   // позиция в display
                            
                            // 5. коллизия — пиксель уже горел
                            if self.display[idx] {
                                self.v[0xF] = 1;
                            }
                            
                            // 6. XOR — переключаем пиксель
                            self.display[idx] ^= true;
                        }
                    }
                }
            }
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
}


fn get_opcode() -> Result<u16, Box<dyn Error>> {
    let mut opcode = String::new();
    io::stdin().read_line(&mut opcode)?;
    let trimmed = opcode.trim();

    let code = u16::from_str_radix(trimmed, 16)?;

    Ok(code)
}

 
fn read_file(path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    Ok(fs::read(path)?)
}



fn main() {
    let mut cpu = CHIP8 {
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
    
    let mut user_choice = String::new();
    println!("Would you like to run a ROM or enter opcodes manually?");
    println!("Type 'run' to load the ROM, or 'manual' to enter opcodes.");
    io::stdin().read_line(&mut user_choice).unwrap();

    if user_choice.trim() == "run" || user_choice.trim() == "Run" {
        match read_file("C:/My_VS_code_Projects/Rust/new_project/src/IBM Logo.ch8") {
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
                        println!("\nCurrent state of registers:\n {:?}", cpu.v);
                        println!("\nCurrent state of memory:\n {:?}", cpu.memory);
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
                    println!("Current state of registers:\n {:?}", cpu.v);
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
}