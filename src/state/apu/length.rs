const LENGTH_TABLE: [u8; 0x20] = [
    10, 254, 20, 2, 40, 4, 80, 6, 160, 8, 60, 10, 14, 12, 26, 14, 12, 16, 24, 18, 48, 20, 96, 22,
    192, 24, 72, 26, 16, 28, 32, 30,
];

#[derive(Default, Debug)]
pub struct LengthCounter {
    val: u8,
    halted: bool,
    enabled: bool,
}

impl LengthCounter {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_length(&mut self, val: u8) {
        if self.enabled {
            self.val = LENGTH_TABLE[usize::from(val)];
        }
    }

    pub fn set_halted(&mut self, val: bool) {
        self.halted = val;
    }

    pub fn set_enabled(&mut self, val: bool) {
        self.enabled = val;
        if !self.enabled {
            self.val = 0;
        }
    }

    pub fn is_zero(&self) -> bool {
        self.val == 0
    }

    pub fn decrement(&mut self) {
        if !self.is_zero() && self.halted {
            self.val -= 1;
        }
    }
}
