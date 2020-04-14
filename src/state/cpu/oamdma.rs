use super::OAMDMA;
use crate::state::ppu::oam::OAM;
use crate::state::NES;

impl OAMDMA for NES {
    fn is_oam_dma_triggered(&self) -> bool {
        self.cpu.oam_dma_triggered
    }

    fn get_oam_dma(&self) -> u8 {
        self.cpu.oam_dma
    }

    fn write_oam(&mut self, offset: usize, val: u8) {
        self.ppu.oam.memory[offset.wrapping_add(usize::from(self.ppu.oam.addr)) % OAM::SIZE] = val;
    }

    fn toggle_odd_even(&mut self) {
        self.cpu.odd_cycle = !self.cpu.odd_cycle;
    }

    fn is_odd_cycle(&self) -> bool {
        self.cpu.odd_cycle
    }

    fn untrigger_oam_dma(&mut self) {
        self.cpu.oam_dma_triggered = false;
    }
}
