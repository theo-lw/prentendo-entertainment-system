pub mod absolute;
pub mod absolute_x;
pub mod absolute_y;
pub mod immediate;
pub mod implied;
pub mod indirect;
pub mod indirect_x;
pub mod indirect_y;
pub mod relative;
pub mod zero;
pub mod zero_x;
pub mod zero_y;

use crate::cpu::instructions::InstructionName;

/// This module holds code related to addressing modes. You'll notice that this module is a
/// collection of functions that take in a CPU struct, an instruction, and return a generator.
/// The returned generator represents an opcode - hence the name `opcode_generators`. A CPU cycle is
/// executed every time the generator is resumed. When the generator is completed, an entire opcode
/// will have been executed.
///
/// What does this have to do with addressing modes? Well, the addressing mode decides the sequence
/// of reads and writes on each cycle, and therefore the contents of each generator.

/// Struct holding information about a CPU cycle
/// Largely used for debugging purposes
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CPUCycle {
    pub instruction: InstructionName,
    pub mode: AddressingMode,
    pub cycle: u8,
}

impl CPUCycle {
    /// Advances the cycle count
    pub fn next(&mut self) {
        self.cycle += 1;
    }
}

/// Represents the addressing mode of an opcode
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
    Relative,
}
