use super::{Branch, Instruction, InstructionName};
use crate::cpu::state::{registers::Flag, CPU};
use std::{cell::RefCell, rc::Rc};

/// Represents the BCC instruction (http://www.obelisk.me.uk/6502/reference.html#BCC)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BCC;

impl Instruction for BCC {
    fn name(&self) -> InstructionName {
        InstructionName::BCC
    }
}

impl Branch for BCC {
    fn should_branch(&self, cpu: &Rc<RefCell<CPU>>) -> bool {
        cpu.borrow().registers.get_flag(Flag::C) == 0
    }
}
