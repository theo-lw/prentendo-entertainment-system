use super::{Get, Set, Register, RegisterName};
use std::cell::RefCell;
use std::rc::Rc;
use crate::cpu::state::CPU;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct X;

impl Register for X {
    fn name(&self) -> RegisterName {
        RegisterName::X
    }
}

impl Get for X {
    fn get(&self, cpu: &Rc<RefCell<CPU>>) -> u8 {
        cpu.borrow().registers.x
    }
}

impl Set for X {
    fn set(&self, cpu: &Rc<RefCell<CPU>>, val: u8) {
        cpu.borrow_mut().registers.x = val;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let mut cpu = CPU::mock();
        cpu.registers.x = 23;
        let cpu = Rc::new(RefCell::new(cpu));
        assert_eq!(X.get(&cpu), 23);
    }

    #[test]
    fn test_set() {
        let mut cpu = CPU::mock();
        cpu.registers.x = 40;
        let cpu = Rc::new(RefCell::new(cpu));
        X.set(&cpu, 94);
        assert_eq!(cpu.borrow().registers.x, 94);
    }
}
