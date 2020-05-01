use crate::bitops::BitOps;
use std::cell::Cell;

pub struct FrameCounter {
    cpu_cycle: u32,
    irq_pending: Cell<bool>,
    pub irq_triggered: bool,
    irq_inhibit: bool,
    mode: FrameCounterMode,
}

impl FrameCounter {
    pub fn new() -> Self {
        FrameCounter {
            cpu_cycle: 0,
            irq_pending: Cell::new(false),
            irq_triggered: false,
            irq_inhibit: true,
            mode: FrameCounterMode::FourStep,
        }
    }

    fn is_last_cycle(&self) -> bool {
        (self.cpu_cycle == 29829 && self.mode == FrameCounterMode::FourStep)
            || (self.cpu_cycle == 37281 && self.mode == FrameCounterMode::FiveStep)
    }

    pub fn is_quarter_frame(&self) -> bool {
        self.cpu_cycle == 7457
            || self.cpu_cycle == 14913
            || self.cpu_cycle == 22371
            || self.is_last_cycle()
    }

    pub fn is_half_frame(&self) -> bool {
        self.cpu_cycle == 14913 || self.is_last_cycle()
    }

    pub fn is_even_cycle(&self) -> bool {
        (self.cpu_cycle % 2) == 0
    }

    pub fn is_output_cycle(&self) -> bool {
        (self.cpu_cycle % 40) == 0
    }

    pub fn get_irq_pending(&self) -> bool {
        let result = self.irq_pending.get();
        self.irq_pending.set(false);
        result
    }

    pub fn clock_cpu_cycle(&mut self) {
        if !self.is_last_cycle() {
            self.cpu_cycle += 1;
            return;
        }
        if self.mode == FrameCounterMode::FourStep && !self.irq_inhibit {
            self.irq_pending.set(true);
            self.irq_triggered = true;
        }
        self.cpu_cycle = 0;
    }

    pub fn set(&mut self, val: u8) {
        self.mode = if val.is_bit_set(7) {
            FrameCounterMode::FiveStep
        } else {
            FrameCounterMode::FourStep
        };
        self.irq_inhibit = val.is_bit_set(6);
        if self.irq_inhibit {
            self.irq_pending.set(false);
        }
    }
}

#[derive(PartialEq)]
enum FrameCounterMode {
    FourStep,
    FiveStep,
}
