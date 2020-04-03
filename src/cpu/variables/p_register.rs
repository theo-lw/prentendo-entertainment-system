use super::{Get, Register, RegisterName, Set};
use crate::cpu::state::CPU;
use std::cell::RefCell;
use std::rc::Rc;

/// Represents the P register
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct P;

impl Register for P {
    fn name(&self) -> RegisterName {
        RegisterName::P
    }
}

impl Get for P {
    fn get(&self, cpu: &Rc<RefCell<CPU>>) -> u8 {
        cpu.borrow().registers.p | 0b0011_0000
    }
}

impl Set for P {
    fn set(&self, cpu: &Rc<RefCell<CPU>>, val: u8) {
        cpu.borrow_mut().registers.p = val;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let mut cpu = CPU::mock();
        cpu.registers.p = 0b0110_1001;
        let cpu = Rc::new(RefCell::new(cpu));
        assert_eq!(P.get(&cpu), 0b0111_1001);
    }

    #[test]
    fn test_set() {
        let mut cpu = CPU::mock();
        cpu.registers.p = 40;
        let cpu = Rc::new(RefCell::new(cpu));
        P.set(&cpu, 94);
        assert_eq!(cpu.borrow().registers.p, 94);
    }
}
