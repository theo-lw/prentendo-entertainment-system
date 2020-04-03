use super::{Instruction, InstructionName, Read};
use crate::address::AddressMap;
use crate::bitops::BitOps;
use crate::cpu::state::CPU;
use crate::cpu::variables::{Flag, Set};
use std::{cell::RefCell, rc::Rc};

/// Represents the LD instructions
/// (http://www.obelisk.me.uk/6502/reference.html#LDA)
/// (http://www.obelisk.me.uk/6502/reference.html#LDX)
/// (http://www.obelisk.me.uk/6502/reference.html#LDY)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct LD<T: Set>(pub T);

impl<T: Set> Instruction for LD<T> {
    fn name(&self) -> InstructionName {
        InstructionName::LD(self.0.name())
    }
}

impl<T: Set> Read for LD<T> {
    fn execute(&self, cpu: &Rc<RefCell<CPU>>, addr: u16) {
        let byte: u8 = cpu.borrow().memory.get(addr);
        self.0.set(cpu, byte);
        if byte == 0 {
            cpu.borrow_mut().registers.set_flag(Flag::Z);
        }
        if byte.is_bit_set(7) {
            cpu.borrow_mut().registers.set_flag(Flag::N);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::variables::{a_register::A, x_register::X, y_register::Y};

    #[test]
    fn test_ld() {
        let mut cpu = CPU::mock();
        cpu.memory.set(0x4304, 0b1001_0110);
        cpu.registers.a = 0b1000_0101;
        let cpu = Rc::new(RefCell::new(cpu));
        LD(A).execute(&cpu, 0x4304);
        assert_eq!(cpu.borrow().registers.a, 0b1001_0110);
    }

    #[test]
    fn test_ld_z() {
        let mut cpu = CPU::mock();
        cpu.memory.set(0x4304, 0b1001_0110);
        cpu.registers.x = 0b1000_0101;
        cpu.registers.clear_flag(Flag::Z);
        let cpu = Rc::new(RefCell::new(cpu));
        LD(X).execute(&cpu, 0x4304);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::Z), false);
        cpu.borrow_mut().memory.set(0x4304, 0);
        LD(X).execute(&cpu, 0x4304);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::Z), true);
    }

    #[test]
    fn test_ld_n() {
        let mut cpu = CPU::mock();
        cpu.memory.set(0x4304, 0b0101_0110);
        cpu.registers.y = 0b1000_0101;
        cpu.registers.clear_flag(Flag::N);
        let cpu = Rc::new(RefCell::new(cpu));
        LD(Y).execute(&cpu, 0x4304);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::N), false);
        cpu.borrow_mut().registers.a = 0b1001_0010;
        cpu.borrow_mut().memory.set(0x4304, 0b1011_0001);
        LD(Y).execute(&cpu, 0x4304);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::N), true);
    }
}
