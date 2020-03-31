pub mod adc;
pub mod and;
pub mod asl;
pub mod bcc;
pub mod bcs;
pub mod beq;
pub mod bit;
pub mod bmi;
pub mod bne;
pub mod bpl;
pub mod bvc;
pub mod bvs;
pub mod pha;
pub mod php;
pub mod pla;
pub mod plp;
pub mod sta;

use crate::cpu::state::CPU;
use std::{cell::RefCell, fmt::Debug, rc::Rc};

/// This module contains instruction-related code. I have categorized instructions into the
/// following traits:

/// A trait representing an instruction.
/// The `name` method is used largely for debugging purposes.
pub trait Instruction {
    fn name(&self) -> InstructionName;
}

/// A trait implemented by 'Read' instructions.
/// The `execute` method should execute the instruction.
pub trait Read: Instruction {
    fn execute(&self, cpu: &Rc<RefCell<CPU>>, addr: u16);
}

/// A trait implemnted by 'Write' instructions.
/// The `execute` method should execute the instruction.
pub trait Write: Instruction {
    fn execute(&self, cpu: &Rc<RefCell<CPU>>, addr: u16);
}

/// A trait implemented by 'Modify' instructions.
/// The `execute` method should execute the instruction.
pub trait Modify: Instruction {
    fn execute(&self, cpu: &Rc<RefCell<CPU>>, addr: u16, old_val: u8);
}

/// A trait implemented by miscellaneous instructions with Implied/Accumulator addressing.
/// The `execute` method should execute the instruction.
pub trait Implied: Instruction {
    fn execute(&self, cpu: &Rc<RefCell<CPU>>);
}

/// A trait implmented by instructions that push a value to the stack.
/// The `get` method should return the value to be pushed to the stack.
pub trait PushStack: Instruction {
    fn get(&self, cpu: &Rc<RefCell<CPU>>) -> u8;
}

/// A trait implemented by instructions that pull values from the stack.
/// The `set` method takes in an argument `val` that holds the value pulled from the stack.
/// It should use that argument to set a register accordingly.
pub trait PullStack: Instruction {
    fn set(&self, cpu: &Rc<RefCell<CPU>>, val: u8);
}

/// A trait implemnted by branching instructions.
/// The `should_branch` method should return `true` if a branch should occur and `false` otherwise.
pub trait Branch: Instruction {
    fn should_branch(&self, cpu: &Rc<RefCell<CPU>>) -> bool;
}

/// An enum holding instruction names
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum InstructionName {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    RTI,
    RTS,
    PHA,
    PHP,
    PLA,
    PLP,
    JSR,
    JMP,
    STA,
}
