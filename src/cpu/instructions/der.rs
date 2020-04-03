use super::{Implied, Instruction, InstructionName};
use crate::bitops::BitOps;
use crate::cpu::state::CPU;
use crate::cpu::variables::{Flag, Get, Set};
use std::{cell::RefCell, rc::Rc};

/// Represents the 'decrement register' instructions
/// (http://www.obelisk.me.uk/6502/reference.html#DEX)
/// (http://www.obelisk.me.uk/6502/reference.html#DEY)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct DE<T: Get + Set>(pub T);

impl<T: Get + Set> Instruction for DE<T> {
    fn name(&self) -> InstructionName {
        InstructionName::DE(self.0.name())
    }
}

impl<T: Get + Set> Implied for DE<T> {
    fn execute(&self, cpu: &Rc<RefCell<CPU>>) {
        let result: u8 = self.0.get(cpu).wrapping_sub(1);
        self.0.set(cpu, result);
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
    use crate::cpu::variables::{x_register::X, y_register::Y};

    #[test]
    fn test_dex() {
        let mut cpu = CPU::mock();
        cpu.registers.x = 100;
        let cpu = Rc::new(RefCell::new(cpu));
        DE(X).execute(&cpu);
        assert_eq!(cpu.borrow().registers.x, 99);
    }

    #[test]
    fn test_dex_z() {
        let mut cpu = CPU::mock();
        cpu.registers.clear_flag(Flag::Z);
        cpu.registers.x = 100;
        let cpu = Rc::new(RefCell::new(cpu));
        DE(X).execute(&cpu);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::Z), false);
        cpu.borrow_mut().registers.x = 1;
        DE(X).execute(&cpu);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::Z), true);
    }

    #[test]
    fn test_dex_n() {
        let mut cpu = CPU::mock();
        cpu.registers.clear_flag(Flag::N);
        cpu.registers.y = 100;
        let cpu = Rc::new(RefCell::new(cpu));
        DE(Y).execute(&cpu);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::N), false);
        cpu.borrow_mut().registers.y = 0;
        DE(Y).execute(&cpu);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::N), true);
    }
}
