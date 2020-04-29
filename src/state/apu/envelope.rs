const INITIAL_DECAY_LEVEL: u8 = 15;

#[derive(Default)]
pub struct Envelope {
    decay_level: u8,
    divider: u8,
    current_divider: u8,
    start: bool,
    envelope_loop: bool,
    constant_volume: bool,
}

impl Envelope {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn clock(&mut self) {
        if self.start {
            self.start = false;
            self.decay_level = INITIAL_DECAY_LEVEL;
            self.current_divider = self.divider;
            return;
        }
        if self.current_divider != 0 {
            self.current_divider -= 1;
            return;
        }
        self.current_divider = self.divider;
        if self.decay_level != 0 {
            self.decay_level -= 1;
        } else if self.envelope_loop {
            self.decay_level = INITIAL_DECAY_LEVEL;
        }
    }

    pub fn get_volume(&self) -> u8 {
        if self.constant_volume {
            self.divider
        } else {
            self.decay_level
        }
    }

    pub fn set_divider(&mut self, val: u8) {
        self.divider = val;
    }

    pub fn set_loop(&mut self, val: bool) {
        self.envelope_loop = val;
    }

    pub fn set_constant_volume(&mut self, val: bool) {
        self.constant_volume = val;
    }

    pub fn set_start(&mut self, val: bool) {
        self.start = val;
    }
}
