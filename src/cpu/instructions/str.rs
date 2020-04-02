use super::{Instruction, InstructionName, Write};
use crate::address::AddressMap;
use crate::cpu::state::CPU;
use crate::cpu::variables::Get;
use std::{cell::RefCell, rc::Rc};

/// Represents the ST instruction (http://www.obelisk.me.uk/6502/reference.html#ST)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ST<T: Get>(pub T);

impl<T: Get> Instruction for ST<T> {
    fn name(&self) -> InstructionName {
        InstructionName::ST(self.0.name())
    }
}

impl<T: Get> Write for ST<T> {
    fn execute(&self, cpu: &Rc<RefCell<CPU>>, addr: u16) {
        let register: u8 = self.0.get(cpu);
        cpu.borrow_mut().memory.set(addr, register)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::variables::a_register::A;

    #[test]
    fn test_st() {
        let cpu = Rc::new(RefCell::new(CPU::mock()));
        cpu.borrow_mut().registers.a = 23;
        ST(A).execute(&cpu, 0x4031);
        assert_eq!(cpu.borrow().memory.get(0x4031), 23);
    }
}
