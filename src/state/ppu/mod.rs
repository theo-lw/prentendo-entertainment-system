mod background;
mod cycle;
mod cycle_status;
mod internal_registers;
mod mapped_registers;
mod memory;
pub mod oam;
mod ram;
mod sprites;
mod vblank;

use cycle_status::CycleStatus;
use internal_registers::InternalRegisters;
use mapped_registers::{SpriteHeight, PPUCTRL, PPUMASK, PPUSTATUS};
use oam::OAM;
use ram::RAM;
use std::cell::Cell;

pub trait DebugRegisters {
    fn get_2002(&self) -> u8;
    fn get_2007(&self) -> u8;
    fn get_v(&self) -> u16;
    fn get_t(&self) -> u16;
}

pub trait MappedRegisters {
    fn get_ppu_ctrl(&self) -> u8;
    fn set_ppu_ctrl(&mut self, val: u8);
    fn get_ppu_mask(&self) -> u8;
    fn set_ppu_mask(&mut self, val: u8);
    fn get_ppu_status(&self) -> u8;
    fn set_ppu_status(&mut self, val: u8);
    fn get_oam_addr(&self) -> u8;
    fn set_oam_addr(&mut self, val: u8);
    fn get_oam_data(&self) -> u8;
    fn set_oam_data(&mut self, val: u8);
    fn get_ppu_scroll(&self) -> u8;
    fn set_ppu_scroll(&mut self, val: u8);
    fn get_ppu_addr(&self) -> u8;
    fn set_ppu_addr(&mut self, val: u8);
    fn get_ppu_data(&self) -> u8;
    fn set_ppu_data(&mut self, val: u8);
}

pub trait Background {
    fn should_render_background(&self) -> bool;
    fn get_nametable_addr(&self) -> u16;
    fn get_attribute_addr(&self) -> u16;
    fn get_attribute_shift(&self) -> u16;
    fn get_background_tile_addr_low(&self, index: u8) -> u16;
    fn get_background_tile_addr_high(&self, index: u8) -> u16;
    fn get_fine_x(&self) -> u8;
    fn get_fine_y(&self) -> u8;
}

pub trait Sprites {
    fn should_render_sprites(&self) -> bool;
    fn get_sprite_height(&self) -> SpriteHeight;
    fn get_sprite_tile_addr_low(&self, sprite_index: u8, y_offset: u8) -> u16;
    fn get_sprite_tile_addr_high(&self, sprite_index: u8, y_offset: u8) -> u16;
    fn read_oam(&self, tile: u8, offset: u8) -> u8;
    fn trigger_sprite_overflow(&mut self);
    fn trigger_sprite_zero(&mut self);
    fn clear_sprite_overflow(&mut self);
    fn clear_sprite_zero(&mut self);
}

pub trait Memory {
    fn get(&self, addr: u16) -> u8;
    fn set(&mut self, addr: u16, val: u8);
}

pub trait Cycle {
    fn update_cycle(&mut self);
    fn get_scanline(&self) -> usize;
    fn get_tick(&self) -> usize;
    fn increment_x(&mut self);
    fn increment_y(&mut self);
    fn reset_x(&mut self);
    fn reset_y(&mut self);
}

pub trait VBlank {
    fn start_vblank(&mut self);
    fn end_vlbank(&mut self);
}

/// Represents the PPU's state
pub struct PPUState {
    ram: RAM,
    pub oam: OAM,
    current_cycle: CycleStatus,
    internal_registers: InternalRegisters,
    ctrl: PPUCTRL,
    mask: PPUMASK,
    status: PPUSTATUS,
    data_buffer: Cell<u8>,
    pub open_bus: Cell<u8>,
}

impl PPUState {
    #[cfg(test)]
    pub fn mock() -> Self {
        Self::new()
    }

    pub fn new() -> Self {
        PPUState {
            ram: RAM::new(),
            oam: OAM::new(),
            current_cycle: CycleStatus::new(),
            internal_registers: InternalRegisters::new(),
            ctrl: PPUCTRL::new(),
            mask: PPUMASK::new(),
            status: PPUSTATUS::new(),
            data_buffer: Cell::new(0),
            open_bus: Cell::new(0),
        }
    }
}
