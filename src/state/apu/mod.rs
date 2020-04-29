mod dmc;
mod envelope;
mod length;
mod noise;
mod pulse;
mod timer;
mod triangle;

use dmc::DMC;
use noise::Noise;
use pulse::{Negation, Pulse};
use triangle::Triangle;

pub trait APU<'a> {
    fn apu_cycle(&mut self);
    fn get_buffer(&'a self) -> &'a [u8];
}

/// Represents the APU's internal state
pub struct APUState {
    buffer: Vec<f32>,
    pulse1: Pulse,
    pulse2: Pulse,
    triangle: Triangle,
    noise: Noise,
    dmc: DMC,
    inhibit_irq: bool,
    frame_mode: FrameCounterMode,
    cycles: u32,
}

impl APUState {
    #[cfg(test)]
    pub fn mock() -> Self {
        Self::new()
    }

    pub fn new() -> Self {
        APUState {
            buffer: Vec::new(),
            pulse1: Pulse::new(Negation::OnesComplement),
            pulse2: Pulse::new(Negation::TwosComplement),
            triangle: Triangle::new(),
            noise: Noise::new(),
            dmc: DMC::new(),
            inhibit_irq: false,
            frame_mode: FrameCounterMode::FourStep,
            cycles: 0,
        }
    }
}

enum FrameCounterMode {
    FourStep,
    FiveStep,
}
