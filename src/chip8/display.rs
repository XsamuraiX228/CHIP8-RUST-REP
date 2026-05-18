const WIDTH: usize = 64;
const HEIGHT: usize = 32;


const BYTE_PER_BITS: u8 = 8;
#[allow(dead_code)]
pub struct Display {
    display: [u64; HEIGHT],
    high_res: bool,
}

impl Display {
    pub fn new() -> Self {
        Self { display: [0; HEIGHT], high_res: false }
    }

    pub fn clear(&mut self) {
        self.display = [0u64; HEIGHT];
    }

    pub fn drawing(&mut self, bit_mask: &[u8], x: usize, y: usize,) -> bool {
        let mut collision: bool = false;
        
        for (idx, &sprite) in bit_mask.iter().enumerate() {
            if idx + y >= HEIGHT {
                break;
            }

            let shift = WIDTH as i8 - x as i8 - BYTE_PER_BITS as i8;
            let extended: u64;
            
            if shift > 0 {
                extended = (sprite as u64) << shift;
            } else {
                extended = (sprite as u64) >> -shift;
            }

            collision |= extended & self.display[idx + y] > 0;
            self.display[idx + y] ^= extended;
        }
        collision
    }

    pub fn render(&self) -> [u32; WIDTH * HEIGHT] {
        let mut result = [0u32; WIDTH * HEIGHT];
        
        for (row, &line) in self.display.iter().enumerate() {
            for col in 0..WIDTH {
                let bit = (line >> (WIDTH - 1 - col)) & 1;
                result[row * WIDTH + col] = if bit == 1 {0xFFFFFF} else {0x000000};
            }
        }
        result
    }
}
