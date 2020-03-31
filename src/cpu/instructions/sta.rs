use super::{Instruction, InstructionName, Write};
use crate::address::AddressMap;
use crate::cpu::state::CPU;
use std::{cell::RefCell, rc::Rc};

/// Represents the STA instruction (http://www.obelisk.me.uk/6502/reference.html#STA)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct STA;

impl Instruction for STA {
    fn name(&self) -> InstructionName {
        InstructionName::STA
    }
}

impl Write for STA {
    fn execute(&self, cpu: &Rc<RefCell<CPU>>, addr: u16) {
        let a: u8 = cpu.borrow().registers.a;
        cpu.borrow_mut().memory.set(addr, a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sta() {
        let cpu = Rc::new(RefCell::new(CPU::mock()));
        cpu.borrow_mut().registers.a = 23;
        STA.execute(&cpu, 0x4031);
        assert_eq!(cpu.borrow().memory.get(0x4031), 23);
    }
}
