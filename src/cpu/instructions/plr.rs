use super::{Instruction, InstructionName, PullStack};
use crate::bitops::BitOps;
use crate::cpu::state::CPU;
use crate::cpu::variables::{Flag, Set};
use std::{cell::RefCell, rc::Rc};

/// Represents the PL instruction (http://www.obelisk.me.uk/6502/reference.html#PL)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PL<T: Set>(pub T);

impl<T: Set> Instruction for PL<T> {
    fn name(&self) -> InstructionName {
        InstructionName::PL(self.0.name())
    }
}

impl<T: Set> PullStack for PL<T> {
    fn set(&self, cpu: &Rc<RefCell<CPU>>, val: u8) {
        if val == 0 {
            cpu.borrow_mut().registers.set_flag(Flag::Z);
        }
        if val.is_bit_set(7) {
            cpu.borrow_mut().registers.set_flag(Flag::N);
        }
        self.0.set(cpu, val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::variables::{a_register::A, p_register::P};

    #[test]
    fn test_plp() {
        let mut cpu = CPU::mock();
        cpu.registers.p = 0;
        let cpu = Rc::new(RefCell::new(cpu));
        PL(P).set(&cpu, 0b0101_1110);
        assert_eq!(cpu.borrow().registers.p, 0b0101_1110);
    }

    #[test]
    fn test_pla_z() {
        let mut cpu = CPU::mock();
        cpu.registers.clear_flag(Flag::Z);
        let cpu = Rc::new(RefCell::new(cpu));
        PL(A).set(&cpu, 12);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::Z), false);
        PL(A).set(&cpu, 0);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::Z), true);
    }

    #[test]
    fn test_pl_n() {
        let mut cpu = CPU::mock();
        cpu.registers.clear_flag(Flag::N);
        let cpu = Rc::new(RefCell::new(cpu));
        PL(A).set(&cpu, 0);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::N), false);
        PL(A).set(&cpu, 0b1100_0010);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::N), true);
    }
}
