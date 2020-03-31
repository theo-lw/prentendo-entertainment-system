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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bcc() {
        let mut cpu = CPU::mock();
        cpu.registers.clear_flag(Flag::C);
        let cpu = Rc::new(RefCell::new(cpu));
        assert_eq!(BCC.should_branch(&cpu), true);
        cpu.borrow_mut().registers.set_flag(Flag::C);
        assert_eq!(BCC.should_branch(&cpu), false);
    }
}
