use super::{MappedRegisters, Memory};
use crate::bitops::BitOps;
use crate::state::cpu::Interrupt;
use crate::state::NES;
use std::cell::Cell;

impl MappedRegisters for NES {
    fn get_ppu_ctrl(&self) -> u8 {
        self.ppu.open_bus.get()
    }
    fn set_ppu_ctrl(&mut self, val: u8) {
        self.ppu
            .internal_registers
            .t
            .replace_bits(0b11_00000_00000, u16::from(val & 0b11) << 10);
        let old_nmi_output = self.ppu.ctrl.should_output_nmi();
        self.ppu.ctrl.set(val);
        self.ppu.open_bus.set(val);
        if !old_nmi_output
            && self.ppu.ctrl.should_output_nmi()
            && self.ppu.status.vblank_started.get()
        {
            self.trigger_nmi();
        }
    }
    fn get_ppu_mask(&self) -> u8 {
        self.ppu.open_bus.get()
    }
    fn set_ppu_mask(&mut self, val: u8) {
        self.ppu.open_bus.set(val);
        self.ppu.mask.set(val);
    }
    fn get_ppu_status(&self) -> u8 {
        let mut result = self.ppu.open_bus.get();
        result.assign_bit(7, self.ppu.status.vblank_started.get());
        result.assign_bit(6, self.ppu.status.sprite0_hit);
        result.assign_bit(5, self.ppu.status.sprite_overflow);
        self.ppu.status.vblank_started.set(false);
        self.ppu.internal_registers.w.set(false);
        self.ppu.open_bus.set(result);
        result
    }
    fn set_ppu_status(&mut self, val: u8) {} // writes don't do anything
    fn get_oam_addr(&self) -> u8 {
        self.ppu.open_bus.get()
    }
    fn set_oam_addr(&mut self, val: u8) {
        self.ppu.oam.addr = val;
        self.ppu.open_bus.set(val);
    }
    fn get_oam_data(&self) -> u8 {
        self.ppu.open_bus.set(self.ppu.oam.read());
        self.ppu.open_bus.get()
    }
    fn set_oam_data(&mut self, val: u8) {
        // "for emulation purposes it's probably best to ignore writes during rendering"
        if self.ppu.current_cycle.is_on_render_line() {
            return;
        }
        self.ppu.open_bus.set(val);
        self.ppu.oam.write(val);
    }
    fn get_ppu_scroll(&self) -> u8 {
        self.ppu.open_bus.get()
    }
    fn set_ppu_scroll(&mut self, val: u8) {
        if self.ppu.internal_registers.w.get() {
            self.ppu.internal_registers.t.replace_bits(
                0b111_00_11111_00000,
                (u16::from(val & 0b111) << 12) + (u16::from(val & 0b11111_000) << 5),
            );
            self.ppu.internal_registers.w.set(false);
        } else {
            self.ppu
                .internal_registers
                .t
                .replace_bits(0b11111, u16::from(val & 0b11111_000) >> 3);
            self.ppu.internal_registers.x = val & 0b111;
            self.ppu.internal_registers.w.set(true);
        }
        self.ppu.open_bus.set(val);
    }
    fn get_ppu_addr(&self) -> u8 {
        self.ppu.open_bus.get()
    }
    fn set_ppu_addr(&mut self, val: u8) {
        if self.ppu.internal_registers.w.get() {
            self.ppu
                .internal_registers
                .t
                .replace_bits(0b111_11111, val as u16);
            self.ppu
                .internal_registers
                .v
                .set(self.ppu.internal_registers.t);
            self.ppu.internal_registers.w.set(false);
        } else {
            self.ppu
                .internal_registers
                .t
                .replace_bits(0b111_11_11000_00000, u16::from(val & 0b1_11111) << 8);
            self.ppu.internal_registers.w.set(true);
        }
        self.ppu.open_bus.set(val);
    }
    fn get_ppu_data(&self) -> u8 {
        let vram_addr: u16 = self.ppu.internal_registers.v.get();
        let result: u8 = if vram_addr < 0x3EFF {
            let val = self.ppu.open_bus.get();
            self.ppu.open_bus.set(self.get(vram_addr));
            val
        } else {
            let val = self.get(self.ppu.internal_registers.v.get());
            self.ppu.open_bus.set(self.get(vram_addr - 0x100));
            val
        };
        self.ppu.open_bus.set(result);
        result
    }
    fn set_ppu_data(&mut self, val: u8) {
        self.set(self.ppu.internal_registers.v.get(), val);
        if self.ppu.current_cycle.is_on_render_line() {
            self.ppu.internal_registers.increment_y();
            self.ppu.internal_registers.increment_x();
        } else {
            self.ppu.internal_registers.v.set(
                self.ppu
                    .internal_registers
                    .v
                    .get()
                    .wrapping_add(self.ppu.ctrl.get_vram_increment()),
            );
        }
        self.ppu.open_bus.set(val);
    }
}

pub struct PPUCTRL {
    register: u8,
}

impl PPUCTRL {
    pub fn new() -> Self {
        PPUCTRL { register: 0 }
    }

    pub fn set(&mut self, val: u8) {
        self.register = val;
    }

    pub fn get_vram_increment(&self) -> u16 {
        if self.register.is_bit_set(2) {
            32
        } else {
            1
        }
    }

    pub fn get_sprite_pattern_table_addr(&self) -> u16 {
        if self.register.is_bit_set(3) {
            0x1000
        } else {
            0
        }
    }

    pub fn get_background_pattern_table_addr(&self) -> u16 {
        if self.register.is_bit_set(4) {
            0x1000
        } else {
            0
        }
    }

    pub fn should_output_nmi(&self) -> bool {
        self.register.is_bit_set(7)
    }
}

pub struct PPUMASK {
    register: u8,
}

impl PPUMASK {
    pub fn new() -> Self {
        PPUMASK { register: 0 }
    }

    pub fn set(&mut self, val: u8) {
        self.register = val;
    }

    pub fn should_render_background(&self) -> bool {
        self.register.is_bit_set(3)
    }

    pub fn should_render_sprites(&self) -> bool {
        self.register.is_bit_set(4)
    }
}

pub struct PPUSTATUS {
    pub vblank_started: Cell<bool>,
    pub sprite0_hit: bool,
    pub sprite_overflow: bool,
}

impl PPUSTATUS {
    pub fn new() -> Self {
        PPUSTATUS {
            vblank_started: Cell::new(false),
            sprite0_hit: false,
            sprite_overflow: false,
        }
    }
}
