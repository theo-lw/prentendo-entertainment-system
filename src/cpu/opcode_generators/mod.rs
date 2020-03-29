pub mod absolute;
pub mod absolute_x;
pub mod absolute_y;
pub mod immediate;
pub mod implied;
pub mod indirect;
pub mod indirect_x;
pub mod indirect_y;
pub mod zero;
pub mod zero_x;
pub mod zero_y;

use crate::cpu::instructions::InstructionName;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CPUCycle {
    pub instruction: InstructionName,
    pub mode: AddressingMode,
    pub cycle: u8,
}

impl CPUCycle {
    pub fn next(&mut self) {
        self.cycle += 1;
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AddressingMode {
    AbsoluteX,
    AbsoluteY,
    Absolute,
    Zero,
    ZeroX,
    ZeroY,
    Immediate,
    Implied,
    Indirect,
    IndirectX,
    IndirectY,
}
