mod cpu;
mod display;
mod input;
mod sound;

use cpu::Cpu;
use display::Display;
use input::Input;
use sound::Sound;
use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <rom_file>", args[0]);
        return;
    }

    let rom_path = &args[1];
    let rom_data = match fs::read(rom_path) {
        Ok(data) => data,
        Err(e) => {
            println!("Error reading ROM: {}", e);
            return;
        }
    };

    let mut chip8 = Cpu::new();
    let mut display = Display::new(64, 32);
    let mut input = Input::new();
    let mut sound = Sound::new();

    chip8.reset();
    chip8.load_rom(&rom_data);

    println!("CHIP-8 emulator loaded ROM: {}", rom_path);
    println!("Running emulation...");

    // Main emulation loop
    let mut cycles = 0;
    loop {
        chip8.cycle();
        cycles += 1;

        // Update display every few cycles
        if cycles % 10 == 0 {
            display.update_from_chip8(chip8.get_display());
        }

        // Update sound
        sound.update(chip8.get_sound_timer());

        // Copy input state to CPU
        for i in 0..16 {
            chip8.set_key(i, input.is_key_pressed(i));
        }

        // Add timing control
        std::thread::sleep(std::time::Duration::from_millis(2));

        // Show display periodically or break after many cycles
        if cycles % 500 == 0 {
            display.print_ascii();
            println!("Cycles: {}", cycles);
        }

        if cycles >= 10000 {
            break;
        }
    }

    println!("Emulation finished after {} cycles", cycles);
    display.print_ascii();
}