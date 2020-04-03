use super::{Implied, Instruction, InstructionName, Modify};
use crate::address::AddressMap;
use crate::bitops::BitOps;
use crate::cpu::state::CPU;
use crate::cpu::variables::Flag;
use std::{cell::RefCell, rc::Rc};

/// Represents the LSR instruction (http://www.obelisk.me.uk/6502/reference.html#LSR)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct LSR;

impl LSR {
    fn set_flags_and_return(cpu: &Rc<RefCell<CPU>>, arg: u8) -> u8 {
        let result: u8 = arg >> 1;
        if arg.is_bit_set(0) {
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

impl Instruction for LSR {
    fn name(&self) -> InstructionName {
        InstructionName::LSR
    }
}

impl Modify for LSR {
    fn execute(&self, cpu: &Rc<RefCell<CPU>>, addr: u16, old_val: u8) {
        let new_val: u8 = Self::set_flags_and_return(cpu, old_val);
        cpu.borrow_mut().memory.set(addr, new_val);
    }
}

impl Implied for LSR {
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
    fn test_lsr_c() {
        let mut cpu: CPU = CPU::mock();
        cpu.registers.clear_flag(Flag::C);
        let cpu = Rc::new(RefCell::new(cpu));
        LSR::set_flags_and_return(&cpu, 0b0100_0001);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::C), true);
        LSR::set_flags_and_return(&cpu, 0b1010_1010);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::C), false);
    }

    #[test]
    fn test_lsr_z() {
        let mut cpu: CPU = CPU::mock();
        cpu.registers.clear_flag(Flag::Z);
        let cpu = Rc::new(RefCell::new(cpu));
        LSR::set_flags_and_return(&cpu, 0b0000_0001);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::Z), true);
        cpu.borrow_mut().registers.clear_flag(Flag::Z);
        LSR::set_flags_and_return(&cpu, 0b1000_0000);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::Z), false);
    }

    #[test]
    fn test_lsr_n() {
        let mut cpu: CPU = CPU::mock();
        cpu.registers.clear_flag(Flag::N);
        let cpu = Rc::new(RefCell::new(cpu));
        LSR::set_flags_and_return(&cpu, 0b0100_0001);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::N), false);
    }

    #[test]
    fn test_lsr_modify() {
        let cpu = Rc::new(RefCell::new(CPU::mock()));
        Modify::execute(&LSR, &cpu, 0x2013, 0b1001_1100);
        assert_eq!(cpu.borrow().memory.get(0x2013), 0b0100_1110);
    }

    #[test]
    fn test_lsr_implied() {
        let mut cpu: CPU = CPU::mock();
        cpu.registers.a = 0b0110_0010;
        let cpu = Rc::new(RefCell::new(cpu));
        Implied::execute(&LSR, &cpu);
        assert_eq!(cpu.borrow().registers.a, 0b0011_0001);
    }
}
