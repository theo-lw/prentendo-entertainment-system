use super::{Instruction, InstructionName, PushStack};
use crate::cpu::state::CPU;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PHA;

impl Instruction for PHA {
    fn name(&self) -> InstructionName {
        InstructionName::PHA
    }
}

impl PushStack for PHA {
    fn get(&self, cpu: &Rc<RefCell<CPU>>) -> u8 {
        cpu.borrow().registers.a
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pha() {
        let mut cpu = CPU::mock();
        cpu.registers.a = 30;
        assert_eq!(PHA.get(&Rc::new(RefCell::new(cpu))), 30);
    }
}
