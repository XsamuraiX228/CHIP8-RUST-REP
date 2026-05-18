pub mod chip8;

pub mod cpu {
    use crate::chip8::cpu::Emulator;
    use crate::chip8::module::Versions;
    use std::error::Error;
    use std::fs;
    use minifb::{Key, Window, WindowOptions};

    pub fn create_emulator(mode: Versions) -> Emulator {
        Emulator::new(mode)
    }

    pub fn read_rom(path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(fs::read(path)?)
    }

    pub fn map_keys(window: &Window, emulator: &mut Emulator) {
        let keys = [
            (Key::X, 0x0), (Key::Key1, 0x1), (Key::Key2, 0x2), (Key::Key3, 0x3),
            (Key::Q, 0x4), (Key::W, 0x5),    (Key::E, 0x6),    (Key::A, 0x7),
            (Key::S, 0x8), (Key::D, 0x9),    (Key::Z, 0xA),    (Key::C, 0xB),
            (Key::Key4, 0xC), (Key::R, 0xD), (Key::F, 0xE),    (Key::V, 0xF),
        ];

        for (key, idx) in keys {
            if window.is_key_down(key) {
                emulator.press_key(idx);
            } else {
                emulator.release_key(idx);
            }
        }
    }

    pub fn create_window() -> Result<Window, minifb::Error> {
        let window = Window::new(
            "Chip-8 Emulator",
            64 * 10,
            32 * 10,
            WindowOptions::default()
        )?;
        Ok(window)
    }
}