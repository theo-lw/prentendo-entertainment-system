use super::{Branch, Instruction, InstructionName};
use crate::cpu::state::{registers::Flag, CPU};
use std::{cell::RefCell, rc::Rc};

/// Represents the BEQ instruction (http://www.obelisk.me.uk/6502/reference.html#BEQ)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BEQ;

impl Instruction for BEQ {
    fn name(&self) -> InstructionName {
        InstructionName::BEQ
    }
}

impl Branch for BEQ {
    fn should_branch(&self, cpu: &Rc<RefCell<CPU>>) -> bool {
        cpu.borrow().registers.get_flag(Flag::Z) == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_beq() {
        let mut cpu = CPU::mock();
        cpu.registers.clear_flag(Flag::Z);
        let cpu = Rc::new(RefCell::new(cpu));
        assert_eq!(BEQ.should_branch(&cpu), false);
        cpu.borrow_mut().registers.set_flag(Flag::Z);
        assert_eq!(BEQ.should_branch(&cpu), true);
    }
}
