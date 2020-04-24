use super::Background;
use crate::state::NES;

impl Background for NES {
    fn should_render_background(&self) -> bool {
        self.ppu.mask.should_render_background()
    }
    fn get_nametable_addr(&self) -> u16 {
        0x2000 | (self.ppu.internal_registers.v.get() & 0x0FFF)
    }
    fn get_attribute_addr(&self) -> u16 {
        0x23C0
            | (self.ppu.internal_registers.v.get() & 0x0C00)
            | ((self.ppu.internal_registers.v.get() >> 4) & 0x38)
            | ((self.ppu.internal_registers.v.get() >> 2) & 0x07)
    }
    fn get_attribute_shift(&self) -> u16 {
        ((self.ppu.internal_registers.v.get() >> 4) & 4) | (self.ppu.internal_registers.v.get() & 2)
    }
    fn get_background_tile_addr_low(&self, index: u8) -> u16 {
        self.ppu.ctrl.get_background_pattern_table_addr()
            + ((index as u16) << 4)
            + self.ppu.internal_registers.get_fine_y()
    }
    fn get_background_tile_addr_high(&self, index: u8) -> u16 {
        self.get_background_tile_addr_low(index) + 0b1000
    }
    fn get_fine_x(&self) -> u8 {
        self.ppu.internal_registers.x
    }
    fn get_fine_y(&self) -> u8 {
        self.ppu.internal_registers.get_fine_y() as u8
    }
}
