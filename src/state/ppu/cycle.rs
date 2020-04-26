use super::cycle_status::CycleStatus;
use super::{Background, Cycle, Sprites};
use crate::state::NES;

impl Cycle for NES {
    fn update_cycle(&mut self) {
        if self.ppu.current_cycle.tick == CycleStatus::MAX_TICKS
            && self.ppu.current_cycle.scanline == CycleStatus::MAX_SCANLINES
        {
            self.ppu.current_cycle.scanline = 0;
            self.ppu.current_cycle.tick = 0;
            self.ppu.current_cycle.is_odd_frame = !self.ppu.current_cycle.is_odd_frame;
            if self.ppu.current_cycle.is_odd_frame
                && self.should_render_sprites()
                && self.should_render_background()
            {
                self.ppu.current_cycle.tick += 1;
            }
        } else if self.ppu.current_cycle.tick == CycleStatus::MAX_TICKS {
            self.ppu.current_cycle.tick = 0;
            self.ppu.current_cycle.scanline += 1;
        } else {
            self.ppu.current_cycle.tick += 1;
        }
    }

    fn get_scanline(&self) -> usize {
        self.ppu.current_cycle.scanline
    }

    fn get_tick(&self) -> usize {
        self.ppu.current_cycle.tick
    }

    fn increment_x(&mut self) {
        self.ppu.internal_registers.increment_x();
    }

    fn increment_y(&mut self) {
        self.ppu.internal_registers.increment_y();
    }

    fn reset_x(&mut self) {
        self.ppu.internal_registers.reset_x();
    }

    fn reset_y(&mut self) {
        self.ppu.internal_registers.reset_y();
    }
}
