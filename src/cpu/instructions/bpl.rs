use super::{Branch, Instruction, InstructionName};
use crate::cpu::state::{registers::Flag, CPU};
use std::{cell::RefCell, rc::Rc};

/// Represents the BPL instruction (http://www.obelisk.me.uk/6502/reference.html#BPL)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BPL;

impl Instruction for BPL {
    fn name(&self) -> InstructionName {
        InstructionName::BPL
    }
}

impl Branch for BPL {
    fn should_branch(&self, cpu: &Rc<RefCell<CPU>>) -> bool {
        cpu.borrow().registers.get_flag(Flag::N) == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bpl() {
        let mut cpu = CPU::mock();
        cpu.registers.clear_flag(Flag::N);
        let cpu = Rc::new(RefCell::new(cpu));
        assert_eq!(BPL.should_branch(&cpu), true);
        cpu.borrow_mut().registers.set_flag(Flag::N);
        assert_eq!(BPL.should_branch(&cpu), false);
    }
}
