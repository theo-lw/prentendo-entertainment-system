use super::envelope::Envelope;
use super::length::LengthCounter;
use super::timer::Timer;
use crate::bitops::BitOps;

const DUTY_LENGTH: usize = 8;

const DUTY_SEQUENCES: [[u8; DUTY_LENGTH]; 4] = [
    [0, 1, 0, 0, 0, 0, 0, 0],
    [0, 1, 1, 0, 0, 0, 0, 0],
    [0, 1, 1, 1, 1, 0, 0, 0],
    [1, 0, 0, 1, 1, 1, 1, 1],
];

const SWEEP_MAX: u16 = 0x7FF;

pub struct Pulse {
    duty: [u8; DUTY_LENGTH],
    duty_index: usize,
    pub length_counter: LengthCounter,
    envelope: Envelope,
    timer: Timer,
    negation: Negation,
    sweep_enable: bool,
    sweep_reload: bool,
    sweep_counter: u8,
    sweep_period: u8,
    sweep_negate: bool,
    sweep_shift: u8,
}

impl Pulse {
    pub fn new(negation: Negation) -> Self {
        Pulse {
            duty: DUTY_SEQUENCES[0],
            duty_index: 0,
            length_counter: LengthCounter::new(),
            envelope: Envelope::new(),
            timer: Timer::new(),
            negation,
            sweep_enable: false,
            sweep_reload: false,
            sweep_period: 0,
            sweep_counter: 0,
            sweep_negate: false,
            sweep_shift: 0,
        }
    }

    pub fn get_volume(&self) -> u8 {
        if self.length_counter.is_zero()
            || self.timer.get_period() < 8
            || self.get_target_period() > SWEEP_MAX
        {
            0
        } else {
            self.duty[self.duty_index] * self.envelope.get_volume()
        }
    }

    pub fn clock(&mut self) {
        if self.timer.is_zero() {
            self.duty_index = (self.duty_index + 1) % 8;
        }
        self.timer.decrement();
    }

    fn get_target_period(&self) -> u16 {
        let change_amount = self.timer.get_period() >> self.sweep_shift;
        if self.sweep_negate {
            self.timer
                .get_period()
                .wrapping_sub(self.negation.apply(change_amount))
        } else {
            self.timer.get_period().wrapping_add(change_amount)
        }
    }

    pub fn sweep(&mut self) {
        if self.sweep_enable && self.sweep_counter == 0 && self.get_target_period() <= SWEEP_MAX {
            self.timer.set(self.get_target_period());
        }

        if self.sweep_counter == 0 || self.sweep_reload {
            self.sweep_counter = self.sweep_period;
            self.sweep_reload = false;
        } else {
            self.sweep_counter -= 1;
        }
    }

    pub fn set_flags(&mut self, val: u8) {
        self.duty = DUTY_SEQUENCES[usize::from(val & 0b1100_0000) >> 6];
        self.length_counter.set_halted(val.is_bit_set(5));
        self.envelope.set_constant_volume(val.is_bit_set(4));
        self.envelope.set_divider(val & 0b1111);
    }

    pub fn set_sweep(&mut self, val: u8) {
        self.sweep_enable = val.is_bit_set(7);
        self.sweep_period = (val & 0b0111_0000) >> 4;
        self.sweep_negate = val.is_bit_set(3);
        self.sweep_shift = val & 0b111;
        self.sweep_reload = true;
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
        self.duty_index = 0;
        self.envelope.set_start(true);
    }
}

pub enum Negation {
    OnesComplement,
    TwosComplement,
}

impl Negation {
    fn apply(&self, val: u16) -> u16 {
        match self {
            Negation::OnesComplement => val + 1,
            Negation::TwosComplement => val,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_negation() {
        assert_eq!(40 - Negation::OnesComplement.apply(20), 19);
        assert_eq!(40 - Negation::TwosComplement.apply(20), 20);
    }
}
