#[derive(Debug)]
pub enum Versions {
    CHIP8,
    SuperChip8,
}

#[derive(Debug)]
pub enum Instruction {
    Control(ControlFlow),
    Data(DataOps),
    Memory(MemoryOps),
    Graphics(Graphics),
    Input(Input),
    Timers(Timers),
    Unknown(u16),
}

#[allow(dead_code)]
#[derive(Debug)]
// ============================================================
// 1. Управление потоком выполнения
// ============================================================
pub enum ControlFlow {
    Return,                                          // 00EE
    Jump        { nnn: u16 },                        // 1nnn
    Call        { nnn: u16 },                        // 2nnn
    SkipIfEq    { vx: u8, kk: u8 },                  // 3xkk
    SkipIfNotEq { vx: u8, kk: u8 },                  // 4xkk
    SkipIfRegEq { vx: u8, vy: u8 },                  // 5xy0
    SkipIfRegNeq{ vx: u8, vy: u8 },                  // 9xy0
    JumpReg     { nnn: u16 },                        // Bnnn
}

// ============================================================
// 2. Работа с данными
// ============================================================
#[derive(Debug)]
pub enum DataOps {
    // Присваивание
    LoadImm     { vx: u8, kk: u8 },                  // 6xkk
    Mov         { vx: u8, vy: u8 },                  // 8xy0

    // Арифметика
    AddImm      { vx: u8, kk: u8 },                  // 7xkk
    Add         { vx: u8, vy: u8 },                  // 8xy4
    Sub         { vx: u8, vy: u8 },                  // 8xy5
    Subn        { vx: u8, vy: u8 },                  // 8xy7

    // Побитовые
    Or          { vx: u8, vy: u8 },                  // 8xy1
    And         { vx: u8, vy: u8 },                  // 8xy2
    Xor         { vx: u8, vy: u8 },                  // 8xy3

    // Сдвиги
    Shr         { vx: u8 },                          // 8xy6
    Shl         { vx: u8 },                          // 8xyE

    // Случайные числа
    Rand        { vx: u8, kk: u8 },                  // Cxkk
}

// ============================================================
// 3. Работа с памятью и регистром I
// ============================================================
#[derive(Debug)]
pub enum MemoryOps {
    SetI        { nnn: u16 },                        // Annn
    AddI        { vx: u8 },                          // Fx1E
    LoadSprite  { vx: u8 },                          // Fx29
    StoreBcd    { vx: u8 },                          // Fx33
    StoreRegs   { vx: u8 },                          // Fx55
    ReadRegs    { vx: u8 },                          // Fx65
}

// ============================================================
// 4. Графика
// ============================================================
#[derive(Debug)]
pub enum Graphics {
    Clear,                                           // 00E0
    Draw { vx: u8, vy: u8, n: u8 },                  // Dxyn
}

// ============================================================
// 5. Ввод с клавиатуры
// ============================================================
#[derive(Debug)]
pub enum Input {
    SkipPressed     { vx: u8 },                      // Ex9E
    SkipNotPressed  { vx: u8 },                      // ExA1
    WaitKey         { vx: u8 },                      // Fx0A
}

// ============================================================
// 6. Таймеры и звук
// ============================================================
#[derive(Debug)]
pub enum Timers {
    GetDelay    { vx: u8 },                          // Fx07
    SetDelay    { vx: u8 },                          // Fx15
    SetSound    { vx: u8 },                          // Fx18
}

pub enum UnknownOpCode {
    Unknown(u16),
}