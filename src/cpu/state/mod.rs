pub mod memory;
pub mod registers;

use crate::address::AddressMap;
use memory::Memory;
use registers::Registers;

/// Represents the CPU's state
#[derive(Default)]
pub struct CPU {
    pub registers: Registers,
    pub memory: Memory,
}

impl CPU {
    #[cfg(test)]
    pub fn mock() -> Self {
        CPU {
            registers: Default::default(),
            memory: Memory::mock()
        }
    }

    /// Pushes a value onto the stack
    pub fn push_stack(&mut self, val: u8) {
        self.memory
            .set(u16::from_be_bytes([0x01, self.registers.s]), val);
        self.registers.s = self.registers.s.wrapping_sub(1);
    }

    /// Retrieves the top of the stack
    pub fn top_stack(&self) -> u8 {
        self.memory
            .get(u16::from_be_bytes([0x01, self.registers.s]))
    }

    /// Removes the top of the stack
    pub fn pop_stack(&mut self) {
        self.registers.s = self.registers.s.wrapping_add(1);
    }

    /// Gets byte at PC and increments PC
    pub fn get_and_increment_pc(&mut self) -> u8 {
        let result: u8 = self.memory.get(self.registers.pc);
        self.registers.increment_pc();
        return result;
    }
}
