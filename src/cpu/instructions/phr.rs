use super::{Instruction, InstructionName, PushStack};
use crate::cpu::state::CPU;
use crate::cpu::variables::Get;
use std::{cell::RefCell, rc::Rc};

/// Represents the 'push register' instructions
/// (http://www.obelisk.me.uk/6502/reference.html#PHA)
/// (http://www.obelisk.me.uk/6502/reference.html#PHP)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PH<T: Get>(pub T);

impl<T: Get> Instruction for PH<T> {
    fn name(&self) -> InstructionName {
        InstructionName::PH(self.0.name())
    }
}

impl<T: Get> PushStack for PH<T> {
    fn get(&self, cpu: &Rc<RefCell<CPU>>) -> u8 {
        self.0.get(cpu)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::variables::{a_register::A, p_register::P};

    #[test]
    fn test_pha() {
        let mut cpu = CPU::mock();
        cpu.registers.a = 30;
        assert_eq!(PH(A).get(&Rc::new(RefCell::new(cpu))), 30);
    }

    #[test]
    fn test_php() {
        let mut cpu = CPU::mock();
        cpu.registers.p = 0b0110_0000;
        assert_eq!(PH(P).get(&Rc::new(RefCell::new(cpu))), 0b0111_0000);
    }
}
