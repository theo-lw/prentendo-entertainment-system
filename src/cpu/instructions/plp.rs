use super::{Instruction, InstructionName, PullStack};
use crate::cpu::state::CPU;
use std::{cell::RefCell, rc::Rc};

/// Represents the PLP instruction (http://www.obelisk.me.uk/6502/reference.html#PLP)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PLP;

impl Instruction for PLP {
    fn name(&self) -> InstructionName {
        InstructionName::PLP
    }
}

impl PullStack for PLP {
    fn set(&self, cpu: &Rc<RefCell<CPU>>, val: u8) {
        cpu.borrow_mut().registers.p = val;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plp() {
        let cpu = Rc::new(RefCell::new(CPU::mock()));
        PLP.set(&cpu, 0b1001_0100);
        assert_eq!(cpu.borrow().registers.p, 0b1001_0100);
    }
}
