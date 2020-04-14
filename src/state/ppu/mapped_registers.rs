use crate::bitops::BitOps;
use std::cell::Cell;

#[derive(Debug, Clone)]
pub struct MappedRegisters {
    pub ppu_ctrl: u8,
    pub ppu_mask: u8,
    pub ppu_status: u8,
    pub ppu_scroll: u8,
    pub ppu_addr: u8,
    pub ppu_data: u8,
    pub open_bus: Cell<u8>,
}

impl MappedRegisters {
    pub fn new() -> Self {
        MappedRegisters {
            ppu_ctrl: 0,
            ppu_mask: 0,
            ppu_status: 0,
            ppu_scroll: 0,
            ppu_addr: 0,
            ppu_data: 0,
            open_bus: Cell::new(0)
        }
    }

    pub fn get_vram_increment(&self) -> u16 {
        if self.ppu_ctrl.is_bit_set(2) {
            32
        } else {
            1
        }
    }

    pub fn get_sprite_pattern_table_addr(&self) -> u16 {
        if self.ppu_ctrl.is_bit_set(3) {
            0x1000
        } else {
            0
        }
    }

    pub fn get_background_pattern_table_addr(&self) -> u16 {
        if self.ppu_ctrl.is_bit_set(4) {
            0x1000
        } else {
            0
        }
    }

    pub fn should_render_background(&self) -> bool {
        self.ppu_addr.is_bit_set(3)
    }

    pub fn should_render_sprites(&self) -> bool {
        self.ppu_addr.is_bit_set(4)
    }
}
