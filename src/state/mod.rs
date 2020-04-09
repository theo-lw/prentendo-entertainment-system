pub mod apu;
pub mod cpu;
pub mod io;
pub mod ppu;

use crate::cartridge::Mapper;
use apu::APUState;
use cpu::CPUState;
use io::IOState;
use ppu::PPUState;

#[cfg(test)]
use crate::cartridge::mapper0::Mapper0;

/// This module holds code related to the NES's state.

pub trait CPU: cpu::Registers + cpu::Memory + cpu::Stack {
    fn get_and_increment_pc(&mut self) -> u8 {
        let result: u8 = self.get_mem(self.get_pc());
        self.increment_pc();
        result
    }
}

/// Represents the NES's state
pub struct NES {
    cpu: CPUState,
    ppu: PPUState,
    apu: APUState,
    io: IOState,
    cartridge: Box<dyn Mapper>,
}

impl NES {
    #[cfg(test)]
    pub fn mock() -> Self {
        NES {
            cpu: CPUState::mock(),
            ppu: PPUState::mock(),
            apu: APUState::mock(),
            io: IOState::mock(),
            cartridge: Box::new(Mapper0::mock()),
        }
    }

    pub fn new(cartridge: Box<dyn Mapper>) -> Self {
        NES {
            cpu: CPUState::new(),
            ppu: PPUState::new(),
            apu: APUState::new(),
            io: IOState::new(),
            cartridge,
        }
    }
}

impl CPU for NES {}

/*
impl NES {
    #[cfg(test)]
    pub fn mock() -> Self {
        NES {
            registers: Registers::mock(),
            memory: Memory::mock(),
        }
    }

    const STACK_PAGE: u8 = 0x01;

    /// Pushes a value onto the stack
    pub fn push_stack(&mut self, val: u8) {
        self.memory.set(
            u16::from_be_bytes([Self::STACK_PAGE, self.registers.s]),
            val,
        );
        self.registers.s = self.registers.s.wrapping_sub(1);
    }

    /// Retrieves the top of the stack (i.e, what's pointed to by the stack pointer)
    pub fn top_stack(&self) -> u8 {
        self.memory
            .get(u16::from_be_bytes([Self::STACK_PAGE, self.registers.s]))
    }

    /// Removes the top of the stack by decrementing the stack pointer
    pub fn pop_stack(&mut self) {
        self.registers.s = self.registers.s.wrapping_add(1);
    }

    /// Gets byte at PC and increments PC
    pub fn get_and_increment_pc(&mut self) -> u8 {
        let result: u8 = self.memory.get(self.registers.pc);
        self.registers.increment_pc();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack() {
        let mut cpu = CPU::mock();
        cpu.registers.s = 0xFF;
        cpu.push_stack(13);
        assert_eq!(cpu.memory.get(0x01FF), 13);
        assert_eq!(cpu.registers.s, 0xFE);
        cpu.pop_stack();
        assert_eq!(cpu.registers.s, 0xFF);
        assert_eq!(cpu.memory.get(0x01FF), 13);
        assert_eq!(cpu.top_stack(), 13);
    }

    #[test]
    fn test_get_and_increment_pc() {
        let mut cpu = CPU::mock();
        cpu.registers.pc = 3;
        cpu.memory.set(3, 14);
        assert_eq!(cpu.get_and_increment_pc(), 14);
        assert_eq!(cpu.registers.pc, 4);
    }
}
*/
