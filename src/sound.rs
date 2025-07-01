pub struct Sound {
    is_playing: bool,
}

impl Sound {
    pub fn new() -> Self {
        Sound {
            is_playing: false,
        }
    }

    pub fn start(&mut self) {
        if !self.is_playing {
            self.is_playing = true;
            // Simple beep representation
            print!("\x07"); // Bell character
        }
    }

    pub fn stop(&mut self) {
        self.is_playing = false;
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing
    }

    pub fn update(&mut self, sound_timer: u8) {
        if sound_timer > 0 {
            self.start();
        } else {
            self.stop();
        }
    }
}