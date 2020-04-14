use super::Cycle;
use super::CycleStatus;
use crate::state::NES;

impl Cycle for NES {
    fn update_cycle(&mut self) {
        self.ppu.current_cycle.tick = (self.ppu.current_cycle.tick + 1) % CycleStatus::MAX_TICKS;
        if self.ppu.current_cycle.tick == 0 {
            self.ppu.current_cycle.scanline =
                (self.ppu.current_cycle.scanline + 1) % CycleStatus::MAX_SCANLINES;
            if self.ppu.current_cycle.scanline != 0 {
                return;
            }
            self.ppu.current_cycle.is_odd_frame = !self.ppu.current_cycle.is_odd_frame;
            if self.ppu.current_cycle.is_odd_frame
                && self.ppu.mapped_registers.should_render_sprites()
                || self.ppu.mapped_registers.should_render_background()
            {
                self.ppu.current_cycle.scanline += 1;
            }
        }
    }

    fn get_scanline(&self) -> usize {
        self.ppu.current_cycle.scanline
    }

    fn get_cycle(&self) -> usize {
        self.ppu.current_cycle.tick
    }
}
