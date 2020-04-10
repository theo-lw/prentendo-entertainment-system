mod memory;
mod registers;
mod stack;

use crate::cpu::variables::Flag;

/// Trait representing CPU registers
pub trait Registers {
    fn get_a(&self) -> u8;
    fn get_x(&self) -> u8;
    fn get_y(&self) -> u8;
    fn get_pc(&self) -> u16;
    fn get_pch(&self) -> u8;
    fn get_pcl(&self) -> u8;
    fn get_s(&self) -> u8;
    fn get_p(&self) -> u8;
    fn set_a(&mut self, val: u8);
    fn set_x(&mut self, val: u8);
    fn set_y(&mut self, val: u8);
    fn set_pc(&mut self, val: u16);
    fn set_pch(&mut self, val: u8);
    fn set_pcl(&mut self, val: u8);
    fn set_s(&mut self, val: u8);
    fn set_p(&mut self, val: u8);
    fn increment_pc(&mut self);
    fn is_flag_set(&self, flag: Flag) -> bool;

    /// If `val` is true, the flag is set. Otherwise, the flag is cleared
    fn assign_flag(&mut self, flag: Flag, val: bool);
}

/// Trait representing the CPU's memory
pub trait Memory {
    fn get_mem(&self, addr: u16) -> u8;
    fn set_mem(&mut self, addr: u16, val: u8);

    /// Gets the byte at the address specified by the PC, then increments the PC
    fn get_and_increment_pc(&mut self) -> u8;
}

/// Trait representing the CPU's stack
pub trait Stack {
    /// Pushes a value onto the stack
    fn push_stack(&mut self, val: u8);

    /// Retrieves the top of the stack (i.e, what's pointed to by the stack pointer)
    fn top_stack(&self) -> u8;

    /// Removes the top of the stack by decrementing the stack pointer
    fn pop_stack(&mut self);
}

/// Represents the CPU's internal state
pub struct CPUState {
    a: u8,
    x: u8,
    y: u8,
    pc: u16,
    s: u8,
    p: u8,
    internal_ram: [u8; 0x800],
}

impl CPUState {
    #[cfg(test)]
    pub fn mock() -> Self {
        Self::new()
    }

    pub fn new() -> Self {
        CPUState {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            s: 0xFD,
            p: 0b0010_0100,
            internal_ram: [0; 0x800],
        }
    }
}
