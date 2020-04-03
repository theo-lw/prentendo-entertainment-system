use super::{Get, Register, RegisterName, Set};
use crate::cpu::state::CPU;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Y;

impl Register for Y {
    fn name(&self) -> RegisterName {
        RegisterName::Y
    }
}

impl Get for Y {
    fn get(&self, cpu: &Rc<RefCell<CPU>>) -> u8 {
        cpu.borrow().registers.y
    }
}

impl Set for Y {
    fn set(&self, cpu: &Rc<RefCell<CPU>>, val: u8) {
        cpu.borrow_mut().registers.y = val;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let mut cpu = CPU::mock();
        cpu.registers.y = 23;
        let cpu = Rc::new(RefCell::new(cpu));
        assert_eq!(Y.get(&cpu), 23);
    }

    #[test]
    fn test_set() {
        let mut cpu = CPU::mock();
        cpu.registers.y = 40;
        let cpu = Rc::new(RefCell::new(cpu));
        Y.set(&cpu, 94);
        assert_eq!(cpu.borrow().registers.y, 94);
    }
}
