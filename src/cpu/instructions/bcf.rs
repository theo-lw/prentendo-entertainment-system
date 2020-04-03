use super::{Branch, Instruction, InstructionName};
use crate::cpu::state::CPU;
use crate::cpu::variables::Flag;
use std::{cell::RefCell, rc::Rc};

/// Represents the 'branch if clear' instructions
/// (http://www.obelisk.me.uk/6502/reference.html#BCC)
/// (http://www.obelisk.me.uk/6502/reference.html#BNE)
/// (http://www.obelisk.me.uk/6502/reference.html#BPL)
/// (http://www.obelisk.me.uk/6502/reference.html#BVC)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BC(pub Flag);

impl Instruction for BC {
    fn name(&self) -> InstructionName {
        InstructionName::BC(self.0)
    }
}

impl Branch for BC {
    fn should_branch(&self, cpu: &Rc<RefCell<CPU>>) -> bool {
        !cpu.borrow().registers.is_flag_set(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bc() {
        let mut cpu = CPU::mock();
        cpu.registers.clear_flag(Flag::Z);
        let cpu = Rc::new(RefCell::new(cpu));
        assert_eq!(BC(Flag::Z).should_branch(&cpu), true);
        cpu.borrow_mut().registers.set_flag(Flag::Z);
        assert_eq!(BC(Flag::Z).should_branch(&cpu), false);
    }
}
