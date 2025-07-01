use rand::Rng;

const MEMORY_SIZE: usize = 4096;
const REGISTER_COUNT: usize = 16;
const STACK_SIZE: usize = 16;
const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;
const FONT_SET_SIZE: usize = 80;

// Built-in font set for hex digits 0-F
const FONT_SET: [u8; FONT_SET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

pub struct Cpu {
    // Memory and registers
    memory: [u8; MEMORY_SIZE],
    v: [u8; REGISTER_COUNT],     // V0-VF registers
    i: u16,                      // Index register
    pc: u16,                     // Program counter

    // Stack
    stack: [u16; STACK_SIZE],
    sp: u8,                      // Stack pointer

    // Timers
    delay_timer: u8,
    sound_timer: u8,

    // Display
    display: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT],

    // Input
    keypad: [bool; 16],

    // RNG
    rng: rand::rngs::ThreadRng,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            memory: [0; MEMORY_SIZE],
            v: [0; REGISTER_COUNT],
            i: 0,
            pc: 0x200, // Program starts at 0x200
            stack: [0; STACK_SIZE],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            display: [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
            keypad: [false; 16],
            rng: rand::thread_rng(),
        }
    }

    pub fn reset(&mut self) {
        // Clear memory
        self.memory = [0; MEMORY_SIZE];

        // Load font set into memory starting at 0x50
        for (i, &byte) in FONT_SET.iter().enumerate() {
            self.memory[0x50 + i] = byte;
        }

        // Reset registers and pointers
        self.v = [0; REGISTER_COUNT];
        self.i = 0;
        self.pc = 0x200;
        self.sp = 0;

        // Reset timers
        self.delay_timer = 0;
        self.sound_timer = 0;

        // Clear display
        self.display = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT];

        // Clear keypad
        self.keypad = [false; 16];
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        for (i, &byte) in rom.iter().enumerate() {
            if 0x200 + i < MEMORY_SIZE {
                self.memory[0x200 + i] = byte;
            }
        }
    }

    pub fn fetch(&self) -> u16 {
        let high_byte = self.memory[self.pc as usize] as u16;
        let low_byte = self.memory[(self.pc + 1) as usize] as u16;
        high_byte << 8 | low_byte
    }

    pub fn execute(&mut self, instruction: u16) {
        let nibbles = (
            (instruction & 0xF000) >> 12,
            (instruction & 0x0F00) >> 8,
            (instruction & 0x00F0) >> 4,
            instruction & 0x000F,
        );

        let x = nibbles.1 as usize;
        let y = nibbles.2 as usize;
        let n = nibbles.3 as u8;
        let nn = (instruction & 0x00FF) as u8;
        let nnn = instruction & 0x0FFF;

        match nibbles {
            (0x0, 0x0, 0xE, 0x0) => self.cls(),
            (0x0, 0x0, 0xE, 0xE) => self.ret(),
            (0x1, _, _, _) => self.jp(nnn),
            (0x2, _, _, _) => self.call(nnn),
            (0x3, _, _, _) => self.se_vx_byte(x, nn),
            (0x4, _, _, _) => self.sne_vx_byte(x, nn),
            (0x5, _, _, 0x0) => self.se_vx_vy(x, y),
            (0x6, _, _, _) => self.ld_vx_byte(x, nn),
            (0x7, _, _, _) => self.add_vx_byte(x, nn),
            (0x8, _, _, 0x0) => self.ld_vx_vy(x, y),
            (0x8, _, _, 0x1) => self.or_vx_vy(x, y),
            (0x8, _, _, 0x2) => self.and_vx_vy(x, y),
            (0x8, _, _, 0x3) => self.xor_vx_vy(x, y),
            (0x8, _, _, 0x4) => self.add_vx_vy(x, y),
            (0x8, _, _, 0x5) => self.sub_vx_vy(x, y),
            (0x8, _, _, 0x6) => self.shr_vx(x),
            (0x8, _, _, 0x7) => self.subn_vx_vy(x, y),
            (0x8, _, _, 0xE) => self.shl_vx(x),
            (0x9, _, _, 0x0) => self.sne_vx_vy(x, y),
            (0xA, _, _, _) => self.ld_i_addr(nnn),
            (0xB, _, _, _) => self.jp_v0_addr(nnn),
            (0xC, _, _, _) => self.rnd_vx_byte(x, nn),
            (0xD, _, _, _) => self.drw_vx_vy_n(x, y, n),
            (0xE, _, 0x9, 0xE) => self.skp_vx(x),
            (0xE, _, 0xA, 0x1) => self.sknp_vx(x),
            (0xF, _, 0x0, 0x7) => self.ld_vx_dt(x),
            (0xF, _, 0x0, 0xA) => self.ld_vx_k(x),
            (0xF, _, 0x1, 0x5) => self.ld_dt_vx(x),
            (0xF, _, 0x1, 0x8) => self.ld_st_vx(x),
            (0xF, _, 0x1, 0xE) => self.add_i_vx(x),
            (0xF, _, 0x2, 0x9) => self.ld_f_vx(x),
            (0xF, _, 0x3, 0x3) => self.ld_b_vx(x),
            (0xF, _, 0x5, 0x5) => self.ld_i_vx(x),
            (0xF, _, 0x6, 0x5) => self.ld_vx_i(x),
            _ => {} // Unknown instruction
        }
    }

    pub fn cycle(&mut self) {
        let instruction = self.fetch();
        self.pc += 2;
        self.execute(instruction);

        // Update timers
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    // Instruction implementations
    fn cls(&mut self) {
        self.display = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
    }

    fn ret(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
    }

    fn jp(&mut self, addr: u16) {
        self.pc = addr;
    }

    fn call(&mut self, addr: u16) {
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = addr;
    }

    fn se_vx_byte(&mut self, x: usize, byte: u8) {
        if self.v[x] == byte {
            self.pc += 2;
        }
    }

    fn sne_vx_byte(&mut self, x: usize, byte: u8) {
        if self.v[x] != byte {
            self.pc += 2;
        }
    }

    fn se_vx_vy(&mut self, x: usize, y: usize) {
        if self.v[x] == self.v[y] {
            self.pc += 2;
        }
    }

    fn ld_vx_byte(&mut self, x: usize, byte: u8) {
        self.v[x] = byte;
    }

    fn add_vx_byte(&mut self, x: usize, byte: u8) {
        self.v[x] = self.v[x].wrapping_add(byte);
    }

    fn ld_vx_vy(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[y];
    }

    fn or_vx_vy(&mut self, x: usize, y: usize) {
        self.v[x] |= self.v[y];
    }

    fn and_vx_vy(&mut self, x: usize, y: usize) {
        self.v[x] &= self.v[y];
    }

    fn xor_vx_vy(&mut self, x: usize, y: usize) {
        self.v[x] ^= self.v[y];
    }

    fn add_vx_vy(&mut self, x: usize, y: usize) {
        let (result, overflow) = self.v[x].overflowing_add(self.v[y]);
        self.v[x] = result;
        self.v[0xF] = if overflow { 1 } else { 0 };
    }

    fn sub_vx_vy(&mut self, x: usize, y: usize) {
        let (result, borrow) = self.v[x].overflowing_sub(self.v[y]);
        self.v[x] = result;
        self.v[0xF] = if borrow { 0 } else { 1 };
    }

    fn shr_vx(&mut self, x: usize) {
        self.v[0xF] = self.v[x] & 0x1;
        self.v[x] >>= 1;
    }

    fn subn_vx_vy(&mut self, x: usize, y: usize) {
        let (result, borrow) = self.v[y].overflowing_sub(self.v[x]);
        self.v[x] = result;
        self.v[0xF] = if borrow { 0 } else { 1 };
    }

    fn shl_vx(&mut self, x: usize) {
        self.v[0xF] = (self.v[x] & 0x80) >> 7;
        self.v[x] <<= 1;
    }

    fn sne_vx_vy(&mut self, x: usize, y: usize) {
        if self.v[x] != self.v[y] {
            self.pc += 2;
        }
    }

    fn ld_i_addr(&mut self, addr: u16) {
        self.i = addr;
    }

    fn jp_v0_addr(&mut self, addr: u16) {
        self.pc = addr + self.v[0] as u16;
    }

    fn rnd_vx_byte(&mut self, x: usize, byte: u8) {
        let random: u8 = self.rng.gen();
        self.v[x] = random & byte;
    }

    fn drw_vx_vy_n(&mut self, x: usize, y: usize, n: u8) {
        let x_pos = self.v[x] as usize % DISPLAY_WIDTH;
        let y_pos = self.v[y] as usize % DISPLAY_HEIGHT;

        self.v[0xF] = 0;

        for row in 0..n as usize {
            if y_pos + row >= DISPLAY_HEIGHT {
                break;
            }

            let sprite_byte = self.memory[self.i as usize + row];

            for col in 0..8 {
                if x_pos + col >= DISPLAY_WIDTH {
                    break;
                }

                let pixel = (sprite_byte >> (7 - col)) & 1;
                if pixel == 1 {
                    if self.display[y_pos + row][x_pos + col] {
                        self.v[0xF] = 1; // Collision
                    }
                    self.display[y_pos + row][x_pos + col] ^= true;
                }
            }
        }
    }

    fn skp_vx(&mut self, x: usize) {
        if self.keypad[self.v[x] as usize] {
            self.pc += 2;
        }
    }

    fn sknp_vx(&mut self, x: usize) {
        if !self.keypad[self.v[x] as usize] {
            self.pc += 2;
        }
    }

    fn ld_vx_dt(&mut self, x: usize) {
        self.v[x] = self.delay_timer;
    }

    fn ld_vx_k(&mut self, x: usize) {
        // Wait for key press - in real implementation, this would halt execution
        for (i, &pressed) in self.keypad.iter().enumerate() {
            if pressed {
                self.v[x] = i as u8;
                return;
            }
        }
        self.pc -= 2; // Repeat instruction until key is pressed
    }

    fn ld_dt_vx(&mut self, x: usize) {
        self.delay_timer = self.v[x];
    }

    fn ld_st_vx(&mut self, x: usize) {
        self.sound_timer = self.v[x];
    }

    fn add_i_vx(&mut self, x: usize) {
        self.i += self.v[x] as u16;
    }

    fn ld_f_vx(&mut self, x: usize) {
        self.i = 0x50 + (self.v[x] as u16) * 5;
    }

    fn ld_b_vx(&mut self, x: usize) {
        let value = self.v[x];
        self.memory[self.i as usize] = value / 100;
        self.memory[self.i as usize + 1] = (value / 10) % 10;
        self.memory[self.i as usize + 2] = value % 10;
    }

    fn ld_i_vx(&mut self, x: usize) {
        for i in 0..=x {
            self.memory[self.i as usize + i] = self.v[i];
        }
    }

    fn ld_vx_i(&mut self, x: usize) {
        for i in 0..=x {
            self.v[i] = self.memory[self.i as usize + i];
        }
    }

    pub fn get_display(&self) -> &[[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT] {
        &self.display
    }

    pub fn set_key(&mut self, key: u8, pressed: bool) {
        if key < 16 {
            self.keypad[key as usize] = pressed;
        }
    }

    pub fn get_sound_timer(&self) -> u8 {
        self.sound_timer
    }
}