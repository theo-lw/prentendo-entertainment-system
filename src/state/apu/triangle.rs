use super::length::LengthCounter;
use super::timer::Timer;
use crate::bitops::BitOps;

const TRIANGLE_SEQUENCE_LENGTH: usize = 32;

const TRIANGLE_SEQUENCE: [u8; TRIANGLE_SEQUENCE_LENGTH] = [
    15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12,
    13, 14, 15,
];

#[derive(Default)]
pub struct Triangle {
    sequence_index: usize,
    timer: Timer,
    pub length_counter: LengthCounter,
    linear_control: bool,
    linear_counter: u8,
    linear_reload_val: u8,
    linear_reload_flag: bool,
}

impl Triangle {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn clock(&mut self) {
        if self.linear_counter == 0 || self.length_counter.is_zero() {
            return;
        }
        if self.timer.is_zero() {
            self.sequence_index = (self.sequence_index + 1) % TRIANGLE_SEQUENCE_LENGTH;
        }
        self.timer.decrement();
    }

    pub fn get_volume(&self) -> u8 {
        if self.linear_counter == 0 || self.length_counter.is_zero() || self.timer.get_period() < 2
        {
            0
        } else {
            TRIANGLE_SEQUENCE[self.sequence_index]
        }
    }

    pub fn decrement_linear(&mut self) {
        if self.linear_reload_flag {
            self.linear_counter = self.linear_reload_val;
        } else if self.linear_counter != 0 {
            self.linear_counter -= 1;
        }

        if !self.linear_control {
            self.linear_reload_flag = false;
        }
    }

    pub fn set_linear(&mut self, val: u8) {
        self.linear_control = val.is_bit_set(7);
        self.length_counter.set_halted(val.is_bit_set(7));
        self.linear_reload_val = val & 0b111_1111;
    }

    pub fn set_timer_low(&mut self, val: u8) {
        self.timer.set(
            self.timer
                .get_period()
                .replace_bits(0b1111_1111, u16::from(val)),
        );
    }

    pub fn set_length(&mut self, val: u8) {
        self.length_counter.set_length(val >> 3);
        self.timer.set(
            self.timer
                .get_period()
                .replace_bits(0b111_0000_0000, u16::from(val) << 8),
        );
        self.linear_reload_flag = true;
    }
}
