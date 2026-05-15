pub mod chip8;
pub mod cpu {
    use crate::chip8::module::{CHIP8, OpCode};
    use std::error::Error;
    use std::fs;
    use minifb::{Key, Window, WindowOptions};
    
    pub fn create_new_cpu() -> CHIP8 {
        CHIP8::new()
    }
    pub fn fetch_opcode(chip8: &mut CHIP8) -> OpCode {
        OpCode::decode(chip8.fetch())
    }
    pub fn execution(chip8: &mut CHIP8, opcode: OpCode) {
        chip8.execute(opcode);
    }
    pub fn load_rom(chip8: &mut CHIP8, rom: &[u8]) {
        chip8.load_rom(rom);
    }
    pub fn read_new_file(path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(fs::read(path)?)
    }
    pub fn map_keys_for_games(window: &Window, cpu: &mut CHIP8) {
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
    pub fn create_new_window() -> Result<Window, minifb::Error> {
        let width = 64 * 10;
        let height = 32 * 10;
        let window = Window::new("Chip-8 Emulator", width, height, WindowOptions::default())?;
        Ok(window)
    }
    pub fn cycle(chip8: &mut CHIP8, opcodes_per_frame: i32) {
        for _ in 0..opcodes_per_frame {
            let opcode = fetch_opcode(chip8);
            execution(chip8, opcode);
        }
        if chip8.delay_timer > 0 { chip8.delay_timer -= 1; }
        if chip8.sound_timer > 0 { chip8.sound_timer -= 1; }
    }
}