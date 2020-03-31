use super::{Branch, Instruction, InstructionName};
use crate::cpu::state::{registers::Flag, CPU};
use std::{cell::RefCell, rc::Rc};

/// Represents the BMI instruction (http://www.obelisk.me.uk/6502/reference.html#BMI)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BMI;

impl Instruction for BMI {
    fn name(&self) -> InstructionName {
        InstructionName::BMI
    }
}

impl Branch for BMI {
    fn should_branch(&self, cpu: &Rc<RefCell<CPU>>) -> bool {
        cpu.borrow().registers.get_flag(Flag::N) == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bmi() {
        let mut cpu = CPU::mock();
        cpu.registers.clear_flag(Flag::N);
        let cpu = Rc::new(RefCell::new(cpu));
        assert_eq!(BMI.should_branch(&cpu), false);
        cpu.borrow_mut().registers.set_flag(Flag::N);
        assert_eq!(BMI.should_branch(&cpu), true);
    }
}
