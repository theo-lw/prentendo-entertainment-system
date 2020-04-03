use super::{Implied, Instruction, InstructionName, Modify};
use crate::address::AddressMap;
use crate::bitops::BitOps;
use crate::cpu::state::CPU;
use crate::cpu::variables::Flag;
use std::{cell::RefCell, rc::Rc};

/// Represents the ROL instruction (http://www.obelisk.me.uk/6502/reference.html#ROL)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ROL;

impl ROL {
    fn set_flags_and_return(cpu: &Rc<RefCell<CPU>>, arg: u8) -> u8 {
        let mut result: u8 = arg << 1;
        if cpu.borrow().registers.is_flag_set(Flag::C) {
            result.set_bit(0);
        } else {
            result.clear_bit(0);
        }
        if arg.is_bit_set(7) {
            cpu.borrow_mut().registers.set_flag(Flag::C);
        } else {
            cpu.borrow_mut().registers.clear_flag(Flag::C);
        }
        if result == 0 {
            cpu.borrow_mut().registers.set_flag(Flag::Z);
        }
        if result.is_bit_set(7) {
            cpu.borrow_mut().registers.set_flag(Flag::N);
        }
        result
    }
}

impl Instruction for ROL {
    fn name(&self) -> InstructionName {
        InstructionName::ROL
    }
}

impl Modify for ROL {
    fn execute(&self, cpu: &Rc<RefCell<CPU>>, addr: u16, old_val: u8) {
        let new_val: u8 = Self::set_flags_and_return(cpu, old_val);
        cpu.borrow_mut().memory.set(addr, new_val);
    }
}

impl Implied for ROL {
    fn execute(&self, cpu: &Rc<RefCell<CPU>>) {
        let a: u8 = cpu.borrow().registers.a;
        let new_val: u8 = Self::set_flags_and_return(cpu, a);
        cpu.borrow_mut().registers.a = new_val;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rol_c() {
        let mut cpu: CPU = CPU::mock();
        cpu.registers.clear_flag(Flag::C);
        let cpu = Rc::new(RefCell::new(cpu));
        ROL::set_flags_and_return(&cpu, 0b1100_0000);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::C), true);
        ROL::set_flags_and_return(&cpu, 0b0010_1011);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::C), false);
    }

    #[test]
    fn test_rol_z() {
        let mut cpu: CPU = CPU::mock();
        cpu.registers.clear_flag(Flag::Z);
        let cpu = Rc::new(RefCell::new(cpu));
        ROL::set_flags_and_return(&cpu, 0b1000_0000);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::Z), true);
        cpu.borrow_mut().registers.clear_flag(Flag::Z);
        ROL::set_flags_and_return(&cpu, 0b0010_1011);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::Z), false);
    }

    #[test]
    fn test_rol_n() {
        let mut cpu: CPU = CPU::mock();
        cpu.registers.clear_flag(Flag::N);
        let cpu = Rc::new(RefCell::new(cpu));
        ROL::set_flags_and_return(&cpu, 0b0100_0000);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::N), true);
        cpu.borrow_mut().registers.clear_flag(Flag::N);
        ROL::set_flags_and_return(&cpu, 0b1010_1011);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::N), false);
    }

    #[test]
    fn test_rol_modify() {
        let mut cpu = CPU::mock();
        cpu.registers.set_flag(Flag::C);
        let cpu = Rc::new(RefCell::new(cpu));
        Modify::execute(&ROL, &cpu, 0x2013, 0b1001_1100);
        assert_eq!(cpu.borrow().memory.get(0x2013), 0b0011_1001);
    }

    #[test]
    fn test_rol_implied() {
        let mut cpu: CPU = CPU::mock();
        cpu.registers.clear_flag(Flag::C);
        cpu.registers.a = 0b0110_0010;
        let cpu = Rc::new(RefCell::new(cpu));
        Implied::execute(&ROL, &cpu);
        assert_eq!(cpu.borrow().registers.a, 0b1100_0100);
    }
}
