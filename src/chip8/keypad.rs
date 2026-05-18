pub struct Keypad {
    key_pressed: u16,
    last_pressed: Option<u8>
}

impl Keypad {
    pub fn new() -> Self {
        Self {
            key_pressed: 0,
            last_pressed: None
        }
    }
    pub fn press_key(&mut self, key: u8) {
        self.key_pressed |= 1 << key;
        self.last_pressed = Some(key)
    } 

    pub fn release_key(&mut self, key: u8) {
        self.key_pressed &= !(1 << key);
    }

    pub fn is_key_pressed(&self, key: u8) -> bool {
        self.key_pressed & (1 << key) != 0
    }

    pub fn is_any_key_pressed(&self) -> bool {
        self.key_pressed != 0
    }

    pub fn get_last_pressed_key(&self) -> u8 {
        self.last_pressed.unwrap()
    }

}