use super::{Instruction, InstructionName, PullStack};
use crate::cpu::state::{registers::Flag, CPU};
use std::{cell::RefCell, rc::Rc};

/// Represents the PLA instruction (http://www.obelisk.me.uk/6502/reference.html#PLA)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PLA;

impl Instruction for PLA {
    fn name(&self) -> InstructionName {
        InstructionName::PLA
    }
}

impl PullStack for PLA {
    fn set(&self, cpu: &Rc<RefCell<CPU>>, val: u8) {
        cpu.borrow_mut().registers.a = val;
        if val == 0 {
            cpu.borrow_mut().registers.set_flag(Flag::Z);
        }
        if val & 0b1000_0000 != 0 {
            cpu.borrow_mut().registers.set_flag(Flag::N);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pla() {
        let mut cpu = CPU::mock();
        cpu.registers.a = 0;
        let cpu = Rc::new(RefCell::new(cpu));
        PLA.set(&cpu, 0b0101_1110);
        assert_eq!(cpu.borrow().registers.a, 0b0101_1110);
    }

    #[test]
    fn test_pla_z() {
        let mut cpu = CPU::mock();
        cpu.registers.clear_flag(Flag::Z);
        let cpu = Rc::new(RefCell::new(cpu));
        PLA.set(&cpu, 12);
        assert_eq!(cpu.borrow().registers.get_flag(Flag::Z), 0);
        PLA.set(&cpu, 0);
        assert_eq!(cpu.borrow().registers.get_flag(Flag::Z), 1);
    }

    #[test]
    fn test_pla_n() {
        let mut cpu = CPU::mock();
        cpu.registers.clear_flag(Flag::N);
        let cpu = Rc::new(RefCell::new(cpu));
        PLA.set(&cpu, 0);
        assert_eq!(cpu.borrow().registers.get_flag(Flag::N), 0);
        PLA.set(&cpu, 0b1100_0010);
        assert_eq!(cpu.borrow().registers.get_flag(Flag::N), 1);
    }
}
