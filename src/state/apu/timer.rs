#[derive(Default)]
pub struct Timer {
    current: u16,
    period: u16,
}

impl Timer {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set(&mut self, val: u16) {
        self.period = val;
        self.current = val;
    }

    pub fn get_period(&self) -> u16 {
        self.period
    }

    pub fn is_zero(&self) -> bool {
        self.current == 0
    }

    pub fn decrement(&mut self) {
        if self.is_zero() {
            self.current = self.period;
        } else {
            self.current -= 1;
        }
    }
}
