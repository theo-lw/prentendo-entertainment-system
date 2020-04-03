use super::{Get, Register, RegisterName, Set};
use crate::cpu::state::CPU;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct S;

impl Register for S {
    fn name(&self) -> RegisterName {
        RegisterName::S
    }
}

impl Get for S {
    fn get(&self, cpu: &Rc<RefCell<CPU>>) -> u8 {
        cpu.borrow().registers.s
    }
}

impl Set for S {
    fn set(&self, cpu: &Rc<RefCell<CPU>>, val: u8) {
        cpu.borrow_mut().registers.s = val;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let mut cpu = CPU::mock();
        cpu.registers.s = 23;
        let cpu = Rc::new(RefCell::new(cpu));
        assert_eq!(S.get(&cpu), 23);
    }

    #[test]
    fn test_set() {
        let mut cpu = CPU::mock();
        cpu.registers.s = 40;
        let cpu = Rc::new(RefCell::new(cpu));
        S.set(&cpu, 94);
        assert_eq!(cpu.borrow().registers.s, 94);
    }
}
