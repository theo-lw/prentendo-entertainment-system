use super::{Branch, Instruction, InstructionName};
use crate::cpu::state::{registers::Flag, CPU};
use std::{cell::RefCell, rc::Rc};

/// Represents the BNE instruction (http://www.obelisk.me.uk/6502/reference.html#BNE)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BNE;

impl Instruction for BNE {
    fn name(&self) -> InstructionName {
        InstructionName::BNE
    }
}

impl Branch for BNE {
    fn should_branch(&self, cpu: &Rc<RefCell<CPU>>) -> bool {
        cpu.borrow().registers.get_flag(Flag::Z) == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bne() {
        let mut cpu = CPU::mock();
        cpu.registers.clear_flag(Flag::Z);
        let cpu = Rc::new(RefCell::new(cpu));
        assert_eq!(BNE.should_branch(&cpu), true);
        cpu.borrow_mut().registers.set_flag(Flag::Z);
        assert_eq!(BNE.should_branch(&cpu), false);
    }
}
