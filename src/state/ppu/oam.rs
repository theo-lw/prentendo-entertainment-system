pub struct OAM {
    memory: [u8; 256],
    pub addr: u8,
}

impl OAM {
    pub fn new() -> Self {
        OAM {
            memory: [0; 256],
            addr: 0,
        }
    }

    pub fn read(&self) -> u8 {
        self.memory[usize::from(self.addr)]
    }

    pub fn write(&mut self, val: u8) {
        self.memory[usize::from(self.addr)] = val;
        self.addr = self.addr.wrapping_add(1);
    }
}
