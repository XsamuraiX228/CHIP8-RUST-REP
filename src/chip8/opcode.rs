use super::module::Instruction;
use super::module::{ControlFlow, DataOps, MemoryOps, Graphics, Input, Timers};

impl Instruction {
    pub fn decode(opcode: u16) -> Self {
        let a = ((opcode & 0xF000) >> 12) as u8;
        let b = ((opcode & 0x0F00) >> 8)  as u8;
        let c = ((opcode & 0x00F0) >> 4)  as u8;
        let d = ((opcode & 0x000F))        as u8;

        let nnn = opcode & 0x0FFF;
        let kk  = (opcode & 0x00FF) as u8;

        match (a, b, c, d) {
            (0x0, 0x0, 0xE, 0x0) => Instruction::Graphics(Graphics::Clear),
            (0x0, 0x0, 0xE, 0xE) => Instruction::Control(ControlFlow::Return),
            (0x1,  _,  _,  _)   => Instruction::Control(ControlFlow::Jump { nnn }),
            (0x2,  _,  _,  _)   => Instruction::Control(ControlFlow::Call { nnn }),
            (0x3,  x,  _,  _)   => Instruction::Control(ControlFlow::SkipIfEq { vx: x, kk }),
            (0x4,  x,  _,  _)   => Instruction::Control(ControlFlow::SkipIfNotEq { vx: x, kk }),
            (0x5,  x,  y, 0x0)  => Instruction::Control(ControlFlow::SkipIfRegEq { vx: x, vy: y }),
            (0x6,  x,  _,  _)   => Instruction::Data(DataOps::LoadImm { vx: x, kk }),
            (0x7,  x,  _,  _)   => Instruction::Data(DataOps::AddImm { vx: x, kk }),
            (0x8,  x,  y, 0x0)  => Instruction::Data(DataOps::Mov { vx: x, vy: y }),
            (0x8,  x,  y, 0x1)  => Instruction::Data(DataOps::Or  { vx: x, vy: y }),
            (0x8,  x,  y, 0x2)  => Instruction::Data(DataOps::And { vx: x, vy: y }),
            (0x8,  x,  y, 0x3)  => Instruction::Data(DataOps::Xor { vx: x, vy: y }),
            (0x8,  x,  y, 0x4)  => Instruction::Data(DataOps::Add { vx: x, vy: y }),
            (0x8,  x,  y, 0x5)  => Instruction::Data(DataOps::Sub { vx: x, vy: y }),
            (0x8,  x,  _,  0x6) => Instruction::Data(DataOps::Shr { vx: x }),
            (0x8,  x,  y, 0x7)  => Instruction::Data(DataOps::Subn { vx: x, vy: y }),
            (0x8,  x,  _,  0xE) => Instruction::Data(DataOps::Shl { vx: x }),
            (0x9,  x,  y, 0x0)  => Instruction::Control(ControlFlow::SkipIfRegNeq { vx: x, vy: y }),
            (0xA,  _,  _,  _)   => Instruction::Memory(MemoryOps::SetI { nnn }),
            (0xB,  _,  _,  _)   => Instruction::Control(ControlFlow::JumpReg { nnn }),
            (0xC,  x,  _,  _)   => Instruction::Data(DataOps::Rand { vx: x, kk }),
            (0xD,  x,  y,  n)   => Instruction::Graphics(Graphics::Draw { vx: x, vy: y, n }),
            (0xE,  x,  0x9, 0xE) => Instruction::Input(Input::SkipPressed { vx: x }),
            (0xE,  x,  0xA, 0x1) => Instruction::Input(Input::SkipNotPressed { vx: x }),
            (0xF,  x,  0x0, 0x7) => Instruction::Timers(Timers::GetDelay { vx: x }),
            (0xF,  x,  0x0, 0xA) => Instruction::Input(Input::WaitKey { vx: x }),
            (0xF,  x,  0x1, 0x5) => Instruction::Timers(Timers::SetDelay { vx: x }),
            (0xF,  x,  0x1, 0x8) => Instruction::Timers(Timers::SetSound { vx: x }),
            (0xF,  x,  0x1, 0xE) => Instruction::Memory(MemoryOps::AddI { vx: x }),
            (0xF,  x,  0x2, 0x9) => Instruction::Memory(MemoryOps::LoadSprite { vx: x }),
            (0xF,  x,  0x3, 0x3) => Instruction::Memory(MemoryOps::StoreBcd { vx: x }),
            (0xF,  x,  0x5, 0x5) => Instruction::Memory(MemoryOps::StoreRegs { vx: x }),
            (0xF,  x,  0x6, 0x5) => Instruction::Memory(MemoryOps::ReadRegs { vx: x }),
            _ => Instruction::Unknown(opcode),
        }
    }
}

