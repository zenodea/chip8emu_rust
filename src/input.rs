use std::io::{self, Read, Write};

pub struct Input {
    keypad: [bool; 16],
}

impl Input {
    pub fn new() -> Self {
        Input {
            keypad: [false; 16],
        }
    }

    pub fn is_key_pressed(&self, key: u8) -> bool {
        if key < 16 {
            self.keypad[key as usize]
        } else {
            false
        }
    }

    pub fn set_key(&mut self, key: u8, pressed: bool) {
        if key < 16 {
            self.keypad[key as usize] = pressed;
        }
    }

    pub fn wait_for_key(&mut self) -> Option<u8> {
        // Simple blocking input for testing
        print!("Press a key (0-9, a-f): ");
        io::stdout().flush().unwrap();

        let mut buffer = [0; 1];
        io::stdin().read_exact(&mut buffer).ok()?;

        let key = match buffer[0] as char {
            '0' => Some(0x0),
            '1' => Some(0x1),
            '2' => Some(0x2),
            '3' => Some(0x3),
            '4' => Some(0x4),
            '5' => Some(0x5),
            '6' => Some(0x6),
            '7' => Some(0x7),
            '8' => Some(0x8),
            '9' => Some(0x9),
            'a' | 'A' => Some(0xA),
            'b' | 'B' => Some(0xB),
            'c' | 'C' => Some(0xC),
            'd' | 'D' => Some(0xD),
            'e' | 'E' => Some(0xE),
            'f' | 'F' => Some(0xF),
            _ => None,
        };

        if let Some(k) = key {
            self.set_key(k, true);
        }

        key
    }

    pub fn get_keypad_state(&self) -> &[bool; 16] {
        &self.keypad
    }
}