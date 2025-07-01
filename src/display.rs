pub struct Display {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>,
}

impl Display {
    pub fn new(width: usize, height: usize) -> Self {
        Display {
            width,
            height,
            pixels: vec![0; width * height * 3], // RGB
        }
    }

    pub fn clear(&mut self) {
        self.pixels.fill(0);
    }

    pub fn update_from_chip8(&mut self, chip8_display: &[[bool; 64]; 32]) {
        for y in 0..32 {
            for x in 0..64 {
                let pixel_index = (y * 64 + x) * 3;
                let color = if chip8_display[y][x] { 255 } else { 0 };

                self.pixels[pixel_index] = color;     // R
                self.pixels[pixel_index + 1] = color; // G
                self.pixels[pixel_index + 2] = color; // B
            }
        }
    }

    pub fn print_ascii(&self) {
        println!("\x1B[2J\x1B[H"); // Clear screen and move cursor to top
        for y in 0..32 {
            for x in 0..64 {
                let pixel_index = (y * 64 + x) * 3;
                let pixel = self.pixels[pixel_index];
                print!("{}", if pixel > 0 { "█" } else { " " });
            }
            println!();
        }
    }
}