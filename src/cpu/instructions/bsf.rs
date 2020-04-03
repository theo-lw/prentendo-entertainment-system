use super::{Branch, Instruction, InstructionName};
use crate::cpu::state::CPU;
use crate::cpu::variables::Flag;
use std::{cell::RefCell, rc::Rc};

/// Represents the 'branch if set' instructions
/// (http://www.obelisk.me.uk/6502/reference.html#BCS)
/// (http://www.obelisk.me.uk/6502/reference.html#BEQ)
/// (http://www.obelisk.me.uk/6502/reference.html#BMI)
/// (http://www.obelisk.me.uk/6502/reference.html#BVS)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BS(pub Flag);

impl Instruction for BS {
    fn name(&self) -> InstructionName {
        InstructionName::BS(self.0)
    }
}

impl Branch for BS {
    fn should_branch(&self, cpu: &Rc<RefCell<CPU>>) -> bool {
        cpu.borrow().registers.is_flag_set(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bs() {
        let mut cpu = CPU::mock();
        cpu.registers.clear_flag(Flag::C);
        let cpu = Rc::new(RefCell::new(cpu));
        assert_eq!(BS(Flag::C).should_branch(&cpu), false);
        cpu.borrow_mut().registers.set_flag(Flag::C);
        assert_eq!(BS(Flag::C).should_branch(&cpu), true);
    }
}
