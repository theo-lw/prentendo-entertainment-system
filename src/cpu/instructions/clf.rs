use super::{Implied, Instruction, InstructionName};
use crate::cpu::state::CPU;
use crate::cpu::variables::Flag;
use std::{cell::RefCell, rc::Rc};

/// Represents the 'clear' instructions 
/// (http://www.obelisk.me.uk/6502/reference.html#CL)
/// (http://www.obelisk.me.uk/6502/reference.html#CLD)
/// (http://www.obelisk.me.uk/6502/reference.html#CLI)
/// (http://www.obelisk.me.uk/6502/reference.html#CLV)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CL(pub Flag);

impl Instruction for CL {
    fn name(&self) -> InstructionName {
        InstructionName::CL(self.0)
    }
}

impl Implied for CL {
    fn execute(&self, cpu: &Rc<RefCell<CPU>>) {
        cpu.borrow_mut().registers.clear_flag(self.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clc() {
        let mut cpu = CPU::mock();
        cpu.registers.clear_flag(Flag::C);
        let cpu = Rc::new(RefCell::new(cpu));
        CL(Flag::C).execute(&cpu);
        assert_eq!(cpu.borrow().registers.get_flag(Flag::C), 0);
        cpu.borrow_mut().registers.set_flag(Flag::Z);
        CL(Flag::Z).execute(&cpu);
        assert_eq!(cpu.borrow().registers.get_flag(Flag::C), 0);
    }
}
