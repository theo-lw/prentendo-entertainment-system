use super::{Implied, Instruction, InstructionName};
use crate::cpu::state::CPU;
use crate::cpu::variables::Flag;
use std::{cell::RefCell, rc::Rc};

/// Represents the 'set flag' instructions
/// (http://www.obelisk.me.uk/6502/reference.html#SE)
/// (http://www.obelisk.me.uk/6502/reference.html#SED)
/// (http://www.obelisk.me.uk/6502/reference.html#SEI)
/// (http://www.obelisk.me.uk/6502/reference.html#SEV)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SE(pub Flag);

impl Instruction for SE {
    fn name(&self) -> InstructionName {
        InstructionName::SE(self.0)
    }
}

impl Implied for SE {
    fn execute(&self, cpu: &Rc<RefCell<CPU>>) {
        cpu.borrow_mut().registers.set_flag(self.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sec() {
        let mut cpu = CPU::mock();
        cpu.registers.clear_flag(Flag::C);
        let cpu = Rc::new(RefCell::new(cpu));
        SE(Flag::C).execute(&cpu);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::C), true);
        SE(Flag::Z).execute(&cpu);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::C), true);
    }
}
