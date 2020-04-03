use super::{Implied, Instruction, InstructionName};
use crate::bitops::BitOps;
use crate::cpu::state::CPU;
use crate::cpu::variables::{Flag, Get, Set};
use std::{cell::RefCell, rc::Rc};

/// Represents the 'transfer' instructions
/// (http://www.obelisk.me.uk/6502/reference.html#TAX)
/// (http://www.obelisk.me.uk/6502/reference.html#TAY)
/// (http://www.obelisk.me.uk/6502/reference.html#TSX)
/// (http://www.obelisk.me.uk/6502/reference.html#TXA)
/// (http://www.obelisk.me.uk/6502/reference.html#TXS)
/// (http://www.obelisk.me.uk/6502/reference.html#TYA)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct T<U: Get, V: Set>(pub U, pub V);

impl<U: Get, V: Set> Instruction for T<U, V> {
    fn name(&self) -> InstructionName {
        InstructionName::T(self.0.name(), self.1.name())
    }
}

impl<U: Get, V: Set> Implied for T<U, V> {
    fn execute(&self, cpu: &Rc<RefCell<CPU>>) {
        let result: u8 = self.0.get(cpu);
        self.1.set(cpu, result);
        if result == 0 {
            cpu.borrow_mut().registers.set_flag(Flag::Z);
        }
        if result.is_bit_set(7) {
            cpu.borrow_mut().registers.set_flag(Flag::N);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::variables::{a_register::A, stack_pointer::S, x_register::X, y_register::Y};

    #[test]
    fn test_txs() {
        let mut cpu = CPU::mock();
        cpu.registers.x = 42;
        cpu.registers.s = 30;
        let cpu = Rc::new(RefCell::new(cpu));
        T(X, S).execute(&cpu);
        assert_eq!(cpu.borrow().registers.s, 42);
    }

    #[test]
    fn test_tay_z() {
        let mut cpu = CPU::mock();
        cpu.registers.clear_flag(Flag::Z);
        cpu.registers.a = 198;
        cpu.registers.x = 12;
        let cpu = Rc::new(RefCell::new(cpu));
        T(A, Y).execute(&cpu);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::Z), false);
        cpu.borrow_mut().registers.a = 0;
        T(A, Y).execute(&cpu);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::Z), true);
    }

    #[test]
    fn test_txa_n() {
        let mut cpu = CPU::mock();
        cpu.registers.clear_flag(Flag::N);
        cpu.registers.a = 198;
        cpu.registers.x = 12;
        let cpu = Rc::new(RefCell::new(cpu));
        T(X, A).execute(&cpu);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::N), false);
        cpu.borrow_mut().registers.x = -42i8 as u8;
        T(X, A).execute(&cpu);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::N), true);
    }
}
