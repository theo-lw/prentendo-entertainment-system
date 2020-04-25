use super::VBlank;
use crate::state::cpu::Interrupt;
use crate::state::NES;

impl VBlank for NES {
    fn start_vblank(&mut self) {
        self.ppu.status.vblank.set(true);
        if self.ppu.ctrl.should_output_nmi() {
            self.trigger_nmi();
        }
    }

    fn end_vlbank(&mut self) {
        self.ppu.status.vblank.set(false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_vblank() {
        let mut nes = NES::mock();
        nes.start_vblank();
        assert_eq!(nes.ppu.status.vblank.get(), true);
    }
}
