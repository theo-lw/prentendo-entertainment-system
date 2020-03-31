use super::{Branch, Instruction, InstructionName};
use crate::cpu::state::{registers::Flag, CPU};
use std::{cell::RefCell, rc::Rc};

/// Represents the BVS instruction (http://www.obelisk.me.uk/6502/reference.html#BVS)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BVS;

impl Instruction for BVS {
    fn name(&self) -> InstructionName {
        InstructionName::BVS
    }
}

impl Branch for BVS {
    fn should_branch(&self, cpu: &Rc<RefCell<CPU>>) -> bool {
        cpu.borrow().registers.get_flag(Flag::V) == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bne() {
        let mut cpu = CPU::mock();
        cpu.registers.clear_flag(Flag::V);
        let cpu = Rc::new(RefCell::new(cpu));
        assert_eq!(BVS.should_branch(&cpu), false);
        cpu.borrow_mut().registers.set_flag(Flag::V);
        assert_eq!(BVS.should_branch(&cpu), true);
    }
}
