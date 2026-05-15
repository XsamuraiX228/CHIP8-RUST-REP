pub struct CHIP8 {
    pub memory: [u8; 4096],
    pub v: [u8; 16],
    pub stack: [u16; 16],
    pub pc: u16,
    pub sp: u8,
    pub idx_i: u16,           // регистр I
    pub display: [bool; 64 * 32],
    pub keypad: [bool; 16],
    pub delay_timer: u8,
    pub sound_timer: u8,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum OpCode {
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