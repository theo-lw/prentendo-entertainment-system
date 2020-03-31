use super::{Branch, Instruction, InstructionName};
use crate::cpu::state::{registers::Flag, CPU};
use std::{cell::RefCell, rc::Rc};

/// Represents the BVC instruction (http://www.obelisk.me.uk/6502/reference.html#BVC)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BVC;

impl Instruction for BVC {
    fn name(&self) -> InstructionName {
        InstructionName::BVC
    }
}

impl Branch for BVC {
    fn should_branch(&self, cpu: &Rc<RefCell<CPU>>) -> bool {
        cpu.borrow().registers.get_flag(Flag::V) == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bvc() {
        let mut cpu = CPU::mock();
        cpu.registers.clear_flag(Flag::V);
        let cpu = Rc::new(RefCell::new(cpu));
        assert_eq!(BVC.should_branch(&cpu), true);
        cpu.borrow_mut().registers.set_flag(Flag::V);
        assert_eq!(BVC.should_branch(&cpu), false);
    }
}
