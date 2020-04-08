use crate::state::NES;
use super::Memory;

impl Memory for NES {
    fn get_mem(&self, addr: u16) -> u8 {
        match addr {
            0..=0x1FFF => self.cpu.internal_ram[usize::from(addr % 0x800)],
            _ => unimplemented!()
        }
    }

    fn set_mem(&mut self, addr: u16, val: u8) {
        match addr {
            0..=0x1FFF => self.cpu.internal_ram[usize::from(addr % 0x800)] = val,
            _ => unimplemented!()
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
