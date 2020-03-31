use super::{Branch, Instruction, InstructionName};
use crate::cpu::state::{registers::Flag, CPU};
use std::{cell::RefCell, rc::Rc};

/// Represents the BCS instruction (http://www.obelisk.me.uk/6502/reference.html#BCS)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BCS;

impl Instruction for BCS {
    fn name(&self) -> InstructionName {
        InstructionName::BCS
    }
}

impl Branch for BCS {
    fn should_branch(&self, cpu: &Rc<RefCell<CPU>>) -> bool {
        cpu.borrow().registers.get_flag(Flag::C) == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bcs() {
        let mut cpu = CPU::mock();
        cpu.registers.clear_flag(Flag::C);
        let cpu = Rc::new(RefCell::new(cpu));
        assert_eq!(BCS.should_branch(&cpu), false);
        cpu.borrow_mut().registers.set_flag(Flag::C);
        assert_eq!(BCS.should_branch(&cpu), true);
    }
}
