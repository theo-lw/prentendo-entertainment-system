mod background;
mod cycle;
mod cycle_status;
mod internal_registers;
mod mapped_registers;
mod memory;
mod ram;

use crate::bitops::BitOps;
use cycle_status::CycleStatus;
use internal_registers::InternalRegisters;
use mapped_registers::MappedRegisters;
use ram::RAM;

pub trait Background {
    fn get_nametable_addr(&self) -> u16;
    fn get_attribute_addr(&self) -> u16;
    fn get_background_tile_addr_low(&self, index: u8) -> u16;
    fn get_background_tile_addr_high(&self, index: u8) -> u16;
}

pub trait Memory {
    fn get(&self, addr: u16) -> u8;
    fn set(&mut self, addr: u16, val: u8);
}

pub trait Cycle {
    fn next(&mut self);
    fn get_scanline(&self) -> usize;
    fn get_cycle(&self) -> usize;
}

/// Represents the PPU's state
pub struct PPUState {
    ram: RAM,
    current_cycle: CycleStatus,
    internal_registers: InternalRegisters,
    mapped_registers: MappedRegisters,
}

impl PPUState {
    #[cfg(test)]
    pub fn mock() -> Self {
        Self::new()
    }

    pub fn new() -> Self {
        PPUState {
            ram: RAM::new(),
            current_cycle: CycleStatus::new(),
            internal_registers: InternalRegisters::new(),
            mapped_registers: MappedRegisters::new(),
        }
    }

    pub fn cpu_get(&self, addr: u16) -> u8 {
        match addr {
            0x2000 => self.mapped_registers.ppu_ctrl,
            0x2001 => self.mapped_registers.ppu_mask,
            0x2002 => {
                self.internal_registers.w.set(false);
                self.mapped_registers.ppu_status
            }
            0x2003 => self.mapped_registers.oam_addr,
            0x2004 => self.mapped_registers.oam_data,
            0x2005 => self.mapped_registers.ppu_scroll,
            0x2006 => self.mapped_registers.ppu_addr,
            0x2007 => {
                if self.current_cycle.is_on_render_line() {
                    self.internal_registers.increment_y();
                    self.internal_registers.increment_x();
                } else {
                    self.internal_registers.v.set(
                        self.internal_registers
                            .v
                            .get()
                            .wrapping_add(self.mapped_registers.get_vram_increment()),
                    );
                }
                self.mapped_registers.ppu_data
            }
            0x4014 => self.mapped_registers.oam_dma,
            _ => unreachable!(),
        }
    }

    pub fn cpu_set(&mut self, addr: u16, val: u8) {
        match addr {
            0x2000 => {
                self.internal_registers
                    .t
                    .replace_bits(0b11_00000_00000, u16::from(val & 0b11) << 10);
                self.mapped_registers.ppu_ctrl = val;
            }
            0x2001 => self.mapped_registers.ppu_mask = val,
            0x2002 => self.mapped_registers.ppu_status = val,
            0x2003 => self.mapped_registers.oam_addr = val,
            0x2004 => self.mapped_registers.oam_data = val,
            0x2005 => {
                if self.internal_registers.w.get() {
                    self.internal_registers.t.replace_bits(
                        0b111_00_11111_00000,
                        (u16::from(val & 0b111) << 12) + (u16::from(val & 0b11111_000) << 5),
                    );
                    self.internal_registers.w.set(false);
                } else {
                    self.internal_registers
                        .t
                        .replace_bits(0b11111, u16::from(val & 0b11111_000) >> 3);
                    self.internal_registers.x = val & 0b111;
                    self.internal_registers.w.set(true);
                }
                self.mapped_registers.ppu_scroll = val;
            }
            0x2006 => {
                if self.internal_registers.w.get() {
                    self.internal_registers
                        .t
                        .replace_bits(0b111_11111, val as u16);
                    self.internal_registers.v.set(self.internal_registers.t);
                    self.internal_registers.w.set(false);
                } else {
                    self.internal_registers
                        .t
                        .replace_bits(0b111_11_11000_00000, u16::from(val & 0b1_11111) << 8);
                    self.internal_registers.w.set(true);
                }
                self.mapped_registers.ppu_addr = val;
            }
            0x2007 => {
                if self.current_cycle.is_on_render_line() {
                    self.internal_registers.increment_y();
                    self.internal_registers.increment_x();
                } else {
                    self.internal_registers.v.set(
                        self.internal_registers
                            .v
                            .get()
                            .wrapping_add(self.mapped_registers.get_vram_increment()),
                    );
                }
                self.mapped_registers.ppu_data = val;
            }
            0x4014 => self.mapped_registers.oam_dma = val,
            _ => unreachable!(),
        }
    }
}
