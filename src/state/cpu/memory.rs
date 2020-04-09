use super::Memory;
use crate::state::NES;

impl Memory for NES {
    fn get_mem(&self, addr: u16) -> u8 {
        match addr {
            0..=0x1FFF => self.cpu.internal_ram[usize::from(addr % 0x800)],
            0x2000..=0x3FFF => match (addr - 0x2000) % 8 {
                0 => self.ppu.ppu_ctrl,
                1 => self.ppu.ppu_mask,
                2 => self.ppu.ppu_status,
                3 => self.ppu.oam_addr,
                4 => self.ppu.oam_data,
                5 => self.ppu.ppu_scroll,
                6 => self.ppu.ppu_addr,
                7 => self.ppu.ppu_data,
                _ => unreachable!(),
            },
            0x4000 => self.apu.sq1_vol,
            0x4001 => self.apu.sq1_sweep,
            0x4002 => self.apu.sq1_lo,
            0x4003 => self.apu.sq1_hi,
            0x4004 => self.apu.sq2_vol,
            0x4005 => self.apu.sq2_sweep,
            0x4006 => self.apu.sq2_lo,
            0x4007 => self.apu.sq2_hi,
            0x4008 => self.apu.tri_linear,
            0x4009 => self.io.unused1,
            0x400A => self.apu.tri_lo,
            0x400B => self.apu.tri_hi,
            0x400C => self.apu.noise_vol,
            0x400D => self.io.unused2,
            0x400E => self.apu.noise_lo,
            0x400F => self.apu.noise_hi,
            0x4010 => self.apu.dmc_freq,
            0x4011 => self.apu.dmc_raw,
            0x4012 => self.apu.dmc_start,
            0x4013 => self.apu.dmc_len,
            0x4014 => self.ppu.oam_dma,
            0x4015 => self.apu.snd_chn,
            0x4016 => self.io.joy1,
            0x4017 => self.io.joy2,
            0x4018..=0x401F => self.io.test_data[usize::from(addr - 0x4018)],
            0x4020..=0xFFFF => self.cartridge.as_cpu_mapper().get(addr),
        }
    }

    fn set_mem(&mut self, addr: u16, val: u8) {
        match addr {
            0..=0x1FFF => self.cpu.internal_ram[usize::from(addr % 0x800)] = val,
            0x2000..=0x3FFF => match (addr - 0x2000) % 8 {
                0 => self.ppu.ppu_ctrl = val,
                1 => self.ppu.ppu_mask = val,
                2 => self.ppu.ppu_status = val,
                3 => self.ppu.oam_addr = val,
                4 => self.ppu.oam_data = val,
                5 => self.ppu.ppu_scroll = val,
                6 => self.ppu.ppu_addr = val,
                7 => self.ppu.ppu_data = val,
                _ => unreachable!(),
            },
            0x4000 => self.apu.sq1_vol = val,
            0x4001 => self.apu.sq1_sweep = val,
            0x4002 => self.apu.sq1_lo = val,
            0x4003 => self.apu.sq1_hi = val,
            0x4004 => self.apu.sq2_vol = val,
            0x4005 => self.apu.sq2_sweep = val,
            0x4006 => self.apu.sq2_lo = val,
            0x4007 => self.apu.sq2_hi = val,
            0x4008 => self.apu.tri_linear = val,
            0x4009 => self.io.unused1 = val,
            0x400A => self.apu.tri_lo = val,
            0x400B => self.apu.tri_hi = val,
            0x400C => self.apu.noise_vol = val,
            0x400D => self.io.unused2 = val,
            0x400E => self.apu.noise_lo = val,
            0x400F => self.apu.noise_hi = val,
            0x4010 => self.apu.dmc_freq = val,
            0x4011 => self.apu.dmc_raw = val,
            0x4012 => self.apu.dmc_start = val,
            0x4013 => self.apu.dmc_len = val,
            0x4014 => self.ppu.oam_dma = val,
            0x4015 => self.apu.snd_chn = val,
            0x4016 => self.io.joy1 = val,
            0x4017 => self.io.joy2 = val,
            0x4018..=0x401F => self.io.test_data[usize::from(addr - 0x4018)] = val,
            0x4020..=0xFFFF => self.cartridge.as_cpu_mapper_mut().set(addr, val),
        }
    }
}

/*
use crate::address::AddressMap;
use crate::cartridge::CPUMapper;
use std::{cell::RefCell, rc::Rc};

#[cfg(test)]
use crate::cartridge::mapper0::Mapper0;

/// Represents the memory map of the NES CPU
pub struct Memory {
    internal_ram: [u8; 0x800],
    ppu_registers: Rc<RefCell<[u8; 0x8]>>,
    apu_and_io: Rc<RefCell<[u8; 0x18]>>,
    apu_and_io_disabled: [u8; 0x8],
    cartridge: Rc<RefCell<dyn CPUMapper>>,
}

impl Memory {
    #[cfg(test)]
    pub fn mock() -> Self {
        Memory {
            internal_ram: [0; 0x800],
            ppu_registers: Rc::new(RefCell::new([0; 0x8])),
            apu_and_io: Rc::new(RefCell::new([0; 0x18])),
            apu_and_io_disabled: [0; 0x8],
            cartridge: Rc::new(RefCell::new(Mapper0::mock())),
        }
    }

    pub fn new(mapper: Rc<RefCell<dyn CPUMapper>>) -> Self {
        Memory {
            internal_ram: [0; 0x800],
            ppu_registers: Rc::new(RefCell::new([0; 0x8])),
            apu_and_io: Rc::new(RefCell::new([0; 0x18])),
            apu_and_io_disabled: [0; 0x8],
            cartridge: mapper,
        }
    }
}

impl AddressMap for Memory {
    fn get(&self, index: u16) -> u8 {
        match index {
            0..=0x1FFF => self.internal_ram[usize::from(index % 0x800)],
            0x2000..=0x3FFF => self.ppu_registers.borrow()[usize::from((index - 0x2000) % 0x8)],
            0x4000..=0x4017 => self.apu_and_io.borrow()[usize::from(index - 0x4000)],
            0x4018..=0x401F => self.apu_and_io_disabled[usize::from(index - 0x4018)],
            0x4020..=0xFFFF => self.cartridge.borrow().get(index),
        }
    }

    fn set(&mut self, index: u16, val: u8) {
        match index {
            0..=0x1FFF => self.internal_ram[usize::from(index % 0x800)] = val,
            0x2000..=0x3FFF => {
                self.ppu_registers.borrow_mut()[usize::from((index - 0x2000) % 0x8)] = val
            }
            0x4000..=0x4017 => self.apu_and_io.borrow_mut()[usize::from(index - 0x4000)] = val,
            0x4018..=0x401F => self.apu_and_io_disabled[usize::from(index - 0x4018)] = val,
            0x4020..=0xFFFF => self.cartridge.borrow_mut().set(index, val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_get_memory() {
        let mut memory = Memory::mock();
        memory.set(0x0304, 12);
        assert_eq!(memory.get(0x0304), 12);
        memory.set(0x2033, 5);
        assert_eq!(memory.get(0x2033), 5);
        memory.set(0x4001, 8);
        assert_eq!(memory.get(0x4001), 8);
        memory.set(0x4019, 30);
        assert_eq!(memory.get(0x4019), 30);
    }
}
*/
