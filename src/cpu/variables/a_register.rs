use super::{Get, Set, Register, RegisterName};
use std::cell::RefCell;
use std::rc::Rc;
use crate::cpu::state::CPU;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct A;

impl Register for A {
    fn name(&self) -> RegisterName {
        RegisterName::A
    }
}

impl Get for A {
    fn get(&self, cpu: &Rc<RefCell<CPU>>) -> u8 {
        cpu.borrow().registers.a
    }
}

impl Set for A {
    fn set(&self, cpu: &Rc<RefCell<CPU>>, val: u8) {
        cpu.borrow_mut().registers.a = val;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let mut cpu = CPU::mock();
        cpu.registers.a = 23;
        let cpu = Rc::new(RefCell::new(cpu));
        assert_eq!(A.get(&cpu), 23);
    }

    #[test]
    fn test_set() {
        let mut cpu = CPU::mock();
        cpu.registers.a = 40;
        let cpu = Rc::new(RefCell::new(cpu));
        A.set(&cpu, 94);
        assert_eq!(cpu.borrow().registers.a, 94);
    }
}
