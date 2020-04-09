pub mod adc;
pub mod and;
pub mod asl;
pub mod bcf;
pub mod bit;
pub mod bsf;
pub mod clf;
pub mod cpr;
pub mod dec;
pub mod der;
pub mod eor;
pub mod inc;
pub mod inr;
pub mod ldr;
pub mod lsr;
pub mod nop;
pub mod ora;
pub mod phr;
pub mod plr;
pub mod rol;
pub mod ror;
pub mod sbc;
pub mod sef;
pub mod str;
pub mod trr;

use crate::cpu::variables::{Flag, RegisterName};
use crate::state::CPU;
use std::fmt::Debug;

/// This module contains instruction-related code. I have categorized instructions into the
/// following traits:

/// A trait representing an instruction.
/// The `name` method is used largely for debugging purposes.
pub trait Instruction {
    fn name(&self) -> InstructionName;
}

/// A trait implemented by 'Read' instructions.
/// The `execute` method should execute the instruction.
pub trait Read<T: CPU>: Instruction {
    fn execute(&self, cpu: &mut T, addr: u16);
}

/// A trait implemnted by 'Write' instructions.
/// The `execute` method should execute the instruction.
pub trait Write<T: CPU>: Instruction {
    fn execute(&self, cpu: &mut T, addr: u16);
}

/// A trait implemented by 'Modify' instructions.
/// The `execute` method should execute the instruction.
pub trait Modify<T: CPU>: Instruction {
    fn execute(&self, cpu: &mut T, addr: u16, old_val: u8);
}

/// A trait implemented by miscellaneous instructions with Implied/Accumulator addressing.
/// The `execute` method should execute the instruction.
pub trait Implied<T: CPU>: Instruction {
    fn execute(&self, cpu: &mut T);
}

/// A trait implmented by instructions that push a value to the stack.
/// The `get` method should return the value to be pushed to the stack.
pub trait PushStack<T: CPU>: Instruction {
    fn get(&self, cpu: &T) -> u8;
}

/// A trait implemented by instructions that pull values from the stack.
/// The `set` method takes in an argument `val` that holds the value pulled from the stack.
/// It should use that argument to set a register accordingly.
pub trait PullStack<T: CPU>: Instruction {
    fn set(&self, cpu: &mut T, val: u8);
}

/// A trait implemnted by branching instructions.
/// The `should_branch` method should return `true` if a branch should occur and `false` otherwise.
pub trait Branch<T: CPU>: Instruction {
    fn should_branch(&self, cpu: &T) -> bool;
}

/// An enum holding instruction names
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum InstructionName {
    ADC,
    AND,
    ASL,
    BC(Flag),
    BS(Flag),
    BIT,
    BRK,
    CL(Flag),
    CLI,
    CLV,
    CP(RegisterName),
    DEC,
    DE(RegisterName),
    EOR,
    INC,
    IN(RegisterName),
    JSR,
    JMP,
    LD(RegisterName),
    LSR,
    NOP,
    ORA,
    PH(RegisterName),
    PL(RegisterName),
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SE(Flag),
    ST(RegisterName),
    T(RegisterName, RegisterName),
}
