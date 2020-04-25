use super::{Interrupt, InterruptState};
use crate::cpu::variables::Flag;
use crate::state::cpu::Registers;
use crate::state::NES;

impl Interrupt for NES {
    fn get_pending_interrupt(&self) -> InterruptState {
        self.cpu.pending_interrupt
    }
    fn trigger_nmi(&mut self) {
        self.cpu.pending_interrupt = InterruptState::NMI;
    }
    fn trigger_irq(&mut self) {
        if self.cpu.pending_interrupt == InterruptState::NMI && !self.is_flag_set(Flag::I) {
            return;
        }
        self.cpu.pending_interrupt = InterruptState::IRQ;
    }
    fn clear_interrupt(&mut self) {
        self.cpu.pending_interrupt = InterruptState::None;
    }
}
