use super::{Get, Set, Register, RegisterName};
use std::cell::RefCell;
use std::rc::Rc;
use crate::cpu::state::CPU;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SP;

impl Register for SP {
    fn name(&self) -> RegisterName {
        RegisterName::SP
    }
}

impl Get for SP {
    fn get(&self, cpu: &Rc<RefCell<CPU>>) -> u8 {
        cpu.borrow().registers.s
    }
}

impl Set for SP {
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
        assert_eq!(SP.get(&cpu), 23);
    }

    #[test]
    fn test_set() {
        let mut cpu = CPU::mock();
        cpu.registers.s = 40;
        let cpu = Rc::new(RefCell::new(cpu));
        SP.set(&cpu, 94);
        assert_eq!(cpu.borrow().registers.s, 94);
    }
}
