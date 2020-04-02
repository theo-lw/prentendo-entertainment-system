use super::{Instruction, InstructionName, Implied};
use crate::cpu::state::CPU;
use std::{cell::RefCell, rc::Rc};

/// Represents the NOP instruction (http://www.obelisk.me.uk/6502/reference.html#NOP)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct NOP;

impl Instruction for NOP {
    fn name(&self) -> InstructionName {
        InstructionName::NOP
    }
}

impl Implied for NOP {
    fn execute(&self, _: &Rc<RefCell<CPU>>) {
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nop() {
        let mut cpu = CPU::mock();
        cpu.registers.x = 100;
        cpu.registers.y = 23;
        cpu.registers.a = 96;
        let cpu = Rc::new(RefCell::new(cpu));
        NOP.execute(&cpu);
        assert_eq!(cpu.borrow().registers.x, 100);
        assert_eq!(cpu.borrow().registers.y, 23);
        assert_eq!(cpu.borrow().registers.a, 96);
    }
}
