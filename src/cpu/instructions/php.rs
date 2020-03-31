use super::{Instruction, InstructionName, PushStack};
use crate::cpu::state::{registers::Flag, CPU};
use std::{cell::RefCell, rc::Rc};

/// Represents the PHP instruction (http://www.obelisk.me.uk/6502/reference.html#PHP)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PHP;

impl Instruction for PHP {
    fn name(&self) -> InstructionName {
        InstructionName::PHP
    }
}

impl PushStack for PHP {
    fn get(&self, cpu: &Rc<RefCell<CPU>>) -> u8 {
        cpu.borrow_mut().registers.set_flag(Flag::B);
        cpu.borrow().registers.p
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_php() {
        let mut cpu = CPU::mock();
        cpu.registers.p = 0b0010_0000;
        assert_eq!(PHP.get(&Rc::new(RefCell::new(cpu))), 0b0011_0000);
    }
}
