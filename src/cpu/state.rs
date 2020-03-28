use crate::address::AddressMap;
use std::{cell::RefCell, rc::Rc};

/// Represents the CPU's state
#[derive(Default)]
pub struct CPU {
    pub registers: Registers,
    pub memory: Memory,
}

impl CPU {
    #[cfg(test)]
    pub fn mock() -> Self {
        Default::default()
    }
}

/// Represents the memory map of the NES CPU
pub struct Memory {
    internal_ram: [u8; 0x800],
    ppu_registers: Rc<RefCell<[u8; 0x8]>>,
    apu_and_io: Rc<RefCell<[u8; 0x18]>>,
    apu_and_io_disabled: [u8; 0x8],
    cartridge_space: [u8; 0xBFE0],
}

impl Default for Memory {
    fn default() -> Self {
        Memory {
            internal_ram: [0; 0x800],
            ppu_registers: Rc::new(RefCell::new([0; 0x8])),
            apu_and_io: Rc::new(RefCell::new([0; 0x18])),
            apu_and_io_disabled: [0; 0x8],
            cartridge_space: [0; 0xBFE0],
        }
    }
}

impl AddressMap for Memory {
    fn get(&self, index: u16) -> u8 {
        match index {
            0..=0x1FFF => self.internal_ram[usize::from(index % 0x800)],
            0x2000..=0x3FFF => {
                self.ppu_registers.borrow()[usize::from((index - 0x2000) % 0x8)].clone()
            }
            0x4000..=0x4017 => self.apu_and_io.borrow()[usize::from(index - 0x4000)].clone(),
            0x4018..=0x401F => self.apu_and_io_disabled[usize::from(index - 0x4018)],
            0x4020..=0xFFFF => self.cartridge_space[usize::from(index - 0x4020)],
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
            0x4020..=0xFFFF => self.cartridge_space[usize::from(index - 0x4020)] = val,
        }
    }
}

/// Represents the registers of the NES CPU
#[derive(Debug)]
pub struct Registers {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub pc: u16,
    pub s: u8,
    pub p: u8,
}

impl Default for Registers {
    fn default() -> Self {
        Registers {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            s: 0,
            p: 0b0010_0000,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Flag {
    C = 0,
    Z = 1,
    I = 2,
    D = 3,
    V = 6,
    N = 7,
    B = 4,
}

impl Registers {
    pub fn set_flag(&mut self, flag: Flag) {
        self.p |= 0b1 << (flag as u8);
    }

    pub fn clear_flag(&mut self, flag: Flag) {
        self.p &= !(0b1 << (flag as u8));
    }

    pub fn get_flag(&self, flag: Flag) -> u8 {
        (self.p & (1 << (flag as u8))) >> (flag as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_flag() {
        let mut cpu: CPU = CPU::mock();
        cpu.registers.p = 0b0010_0000;
        cpu.registers.set_flag(Flag::B);
        assert_eq!(cpu.registers.p, 0b0011_0000);
        cpu.registers.set_flag(Flag::B);
        assert_eq!(cpu.registers.p, 0b0011_0000);
    }

    #[test]
    fn test_get_flag() {
        let mut cpu: CPU = CPU::mock();
        cpu.registers.p = 0b0010_0000;
        assert_eq!(cpu.registers.get_flag(Flag::C), 0);
        cpu.registers.p = 0b0010_0001;
        assert_eq!(cpu.registers.get_flag(Flag::C), 1);
    }

    #[test]
    fn test_clear_flag() {
        let mut cpu: CPU = CPU::mock();
        cpu.registers.p = 0b1010_0000;
        cpu.registers.clear_flag(Flag::N);
        assert_eq!(cpu.registers.p, 0b0010_0000);
        cpu.registers.clear_flag(Flag::N);
        assert_eq!(cpu.registers.p, 0b0010_0000);
    }
}
