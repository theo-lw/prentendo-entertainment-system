use super::{Instruction, InstructionName, PullStack};
use crate::cpu::state::{CPU, registers::Flag};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PLA;

impl Instruction for PLA {
    fn name(&self) -> InstructionName {
        InstructionName::PLA
    }
}

impl PullStack for PLA {
    fn set(&self, cpu: &Rc<RefCell<CPU>>, val: u8) {
        cpu.borrow_mut().registers.a = val;
        if val == 0 {
            cpu.borrow_mut().registers.set_flag(Flag::Z);
        }
        if val & 0b1000_0000 != 0 { 
            cpu.borrow_mut().registers.set_flag(Flag::N);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
