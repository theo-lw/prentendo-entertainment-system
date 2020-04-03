use super::{Instruction, InstructionName, Read};
use crate::address::AddressMap;
use crate::bitops::BitOps;
use crate::cpu::state::CPU;
use crate::cpu::variables::{Flag, Get};
use std::{cell::RefCell, rc::Rc};

/// Represents the 'compare' instructions (CMP, CPX, CPY)
/// (http://www.obelisk.me.uk/6502/reference.html#CMP)
/// (http://www.obelisk.me.uk/6502/reference.html#CPX)
/// (http://www.obelisk.me.uk/6502/reference.html#CPY)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CP<T: Get>(pub T);

impl<T: Get> Instruction for CP<T> {
    fn name(&self) -> InstructionName {
        InstructionName::CP(self.0.name())
    }
}

impl<T: Get> Read for CP<T> {
    fn execute(&self, cpu: &Rc<RefCell<CPU>>, addr: u16) {
        let byte: u8 = cpu.borrow().memory.get(addr);
        let register: u8 = self.0.get(cpu);
        let (result, overflow): (u8, bool) = register.overflowing_sub(byte);
        if !overflow {
            cpu.borrow_mut().registers.set_flag(Flag::C);
        }
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
    use crate::cpu::variables::{a_register::A, x_register::X, y_register::Y};

    #[test]
    fn test_cp_c() {
        let mut cpu = CPU::mock();
        cpu.memory.set(0x4304, 13);
        cpu.registers.x = 2;
        cpu.registers.clear_flag(Flag::C);
        let cpu = Rc::new(RefCell::new(cpu));
        CP(X).execute(&cpu, 0x4304);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::C), false);
        cpu.borrow_mut().registers.x = 30;
        CP(X).execute(&cpu, 0x4304);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::C), true);
    }

    #[test]
    fn test_cp_z() {
        let mut cpu = CPU::mock();
        cpu.memory.set(0x4304, 30);
        cpu.registers.y = 50;
        cpu.registers.clear_flag(Flag::Z);
        let cpu = Rc::new(RefCell::new(cpu));
        CP(Y).execute(&cpu, 0x4304);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::Z), false);
        cpu.borrow_mut().memory.set(0x4304, 50);
        CP(Y).execute(&cpu, 0x4304);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::Z), true);
    }

    #[test]
    fn test_cp_n() {
        let mut cpu = CPU::mock();
        cpu.memory.set(0x4304, 40);
        cpu.registers.a = 45;
        cpu.registers.clear_flag(Flag::N);
        let cpu = Rc::new(RefCell::new(cpu));
        CP(A).execute(&cpu, 0x4304);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::N), false);
        cpu.borrow_mut().registers.a = 12;
        CP(A).execute(&cpu, 0x4304);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::N), true);
    }
}
