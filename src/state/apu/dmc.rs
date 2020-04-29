use super::timer::Timer;
use crate::bitops::BitOps;

const DMC_RATE: [u16; 0x10] = [
    428, 380, 340, 320, 286, 254, 226, 214, 190, 160, 142, 128, 106, 84, 72, 54,
];

#[derive(Default)]
pub struct DMC {
    timer: Timer,
    irq_enable: bool,
    irq_pending: bool,
    loop_flag: bool,
    shift_register: u8,
    bits_remaining: usize,
    output_level: u8,
    silent: bool,
    sample_buffer: u8,
    sample_empty: bool,
    sample_addr: u16,
    sample_length: u16,
}

impl DMC {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn clock(&mut self) {
        if !self.timer.is_zero() {
            self.timer.decrement();
            return;
        }
        if !self.silent {
            if self.shift_register.is_bit_set(0) && self.output_level <= 125 {
                self.output_level += 2;
            } else if !self.shift_register.is_bit_set(0) && self.output_level >= 2 {
                self.output_level -= 2;
            }
        }

        self.shift_register >>= 1;
        self.bits_remaining -= 1;

        if self.bits_remaining == 0 {
            self.bits_remaining = 8;
            self.shift_register = self.sample_buffer;
            self.silent = self.sample_empty;
            self.sample_empty = true;
        }
    }

    pub fn get_volume(&self) -> u8 {
        self.output_level
    }

    pub fn set_flags(&mut self, val: u8) {
        self.irq_enable = val.is_bit_set(7);
        if !self.irq_enable {
            self.irq_pending = false;
        }
        self.loop_flag = val.is_bit_set(6);
        self.timer.set(DMC_RATE[usize::from(val & 0b1111)]);
    }

    pub fn set_output(&mut self, val: u8) {
        self.output_level = val & 0b111_1111;
    }

    pub fn set_addr(&mut self, val: u8) {
        self.sample_addr = 0xC000 + (u16::from(val) * 64);
    }

    pub fn set_length(&mut self, val: u8) {
        self.sample_length = (u16::from(val) * 16) + 1;
    }
}
