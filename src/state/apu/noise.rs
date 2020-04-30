use super::envelope::Envelope;
use super::length::LengthCounter;
use super::timer::Timer;
use crate::bitops::BitOps;

const NOISE_RATE: [u16; 0x10] = [
    4, 8, 16, 32, 64, 96, 128, 160, 202, 254, 380, 508, 762, 1016, 2034, 4068,
];

#[derive(Default)]
pub struct Noise {
    envelope: Envelope,
    timer: Timer,
    pub length_counter: LengthCounter,
    shift_register: u16,
    mode: bool,
}

impl Noise {
    pub fn new() -> Self {
        Noise {
            shift_register: 1,
            ..Default::default()
        }
    }

    fn shift(&mut self) {
        let bit = if self.mode { 6 } else { 1 };
        let feedback = self.shift_register.is_bit_set(bit) ^ self.shift_register.is_bit_set(0);
        self.shift_register >>= 1;
        self.shift_register.assign_bit(14, feedback);
    }

    pub fn get_volume(&self) -> u8 {
        if self.shift_register.is_bit_set(0) || self.length_counter.is_zero() {
            0
        } else {
            self.envelope.get_volume()
        }
    }

    pub fn clock(&mut self) {
        if self.timer.is_zero() {
            self.shift();
        }
        self.timer.decrement()
    }

    pub fn set_flags(&mut self, val: u8) {
        self.length_counter.set_halted(val.is_bit_set(5));
        self.envelope.set_constant_volume(val.is_bit_set(4));
        self.envelope.set_divider(val & 0b1111);
    }

    pub fn set_period(&mut self, val: u8) {
        self.mode = val.is_bit_set(7);
        self.timer.set(NOISE_RATE[usize::from(val & 0b1111)]);
    }

    pub fn set_length(&mut self, val: u8) {
        self.length_counter.set_length(val >> 3);
        self.envelope.set_start(true);
    }
}
