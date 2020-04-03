use super::{Implied, Instruction, InstructionName};
use crate::bitops::BitOps;
use crate::cpu::state::CPU;
use crate::cpu::variables::{Flag, Get, Set};
use std::{cell::RefCell, rc::Rc};

/// Represents the 'increment register' instructions
/// (http://www.obelisk.me.uk/6502/reference.html#INX)
/// (http://www.obelisk.me.uk/6502/reference.html#INY)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct IN<T: Get + Set>(pub T);

impl<T: Get + Set> Instruction for IN<T> {
    fn name(&self) -> InstructionName {
        InstructionName::IN(self.0.name())
    }
}

impl<T: Get + Set> Implied for IN<T> {
    fn execute(&self, cpu: &Rc<RefCell<CPU>>) {
        let result: u8 = self.0.get(cpu).wrapping_add(1);
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
    fn test_in() {
        let mut cpu = CPU::mock();
        cpu.registers.x = 100;
        let cpu = Rc::new(RefCell::new(cpu));
        IN(X).execute(&cpu);
        assert_eq!(cpu.borrow().registers.x, 101);
    }

    #[test]
    fn test_in_z() {
        let mut cpu = CPU::mock();
        cpu.registers.clear_flag(Flag::Z);
        cpu.registers.y = 100;
        let cpu = Rc::new(RefCell::new(cpu));
        IN(Y).execute(&cpu);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::Z), false);
        cpu.borrow_mut().registers.y = 255;
        IN(Y).execute(&cpu);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::Z), true);
    }

    #[test]
    fn test_in_n() {
        let mut cpu = CPU::mock();
        cpu.registers.clear_flag(Flag::N);
        cpu.registers.x = 100;
        let cpu = Rc::new(RefCell::new(cpu));
        IN(X).execute(&cpu);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::N), false);
        cpu.borrow_mut().registers.x = 235;
        IN(X).execute(&cpu);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::N), true);
    }
}
