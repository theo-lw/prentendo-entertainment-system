use crate::bitops::BitOps;
use std::cell::Cell;

/// Represents the internal PPU registers
#[derive(Debug, Clone, PartialEq)]
pub struct InternalRegisters {
    pub v: Cell<u16>,
    pub t: u16,
    pub x: u8,
    pub w: Cell<bool>,
}

impl InternalRegisters {
    pub fn new() -> Self {
        InternalRegisters {
            v: Cell::new(0),
            t: 0,
            x: 0,
            w: Cell::new(false),
        }
    }

    pub fn increment_x(&self) {
        if (self.v.get() & 0b11111) == 0b11111 {
            self.v.set(self.v.get() & !0b11111);
            self.v.set(self.v.get() ^ 0b1_00000_00000);
        } else {
            self.v.set(self.v.get() + 1);
        }
    }

    pub fn increment_y(&self) {
        if (self.v.get() & 0x7000) != 0x7000 {
            self.v.set(self.v.get() + 0x1000);
        } else {
            self.v.set(self.v.get() & !0x7000);
            let mut y = self.v.get() & 0b11111_00000 >> 5;
            if y == 29 {
                y = 0;
                self.v.set(self.v.get() ^ 0b10_00000_00000);
            } else if y == 31 {
                y = 0;
            } else {
                y += 1;
            }
            self.v.set(self.v.get().replace_bits(0b11111_00000, y << 5));
        }
    }

    pub fn reset_x(&mut self) {
        self.v
            .set(self.v.get().replace_bits(0b1_00000_11111, self.t));
    }

    pub fn reset_y(&mut self) {
        self.v
            .set(self.v.get().replace_bits(0b111_10_11111_00000, self.t));
    }

    pub fn get_fine_y(&self) -> u16 {
        (self.v.get() & 0b111_00_00000_00000) >> 12
    }
}
