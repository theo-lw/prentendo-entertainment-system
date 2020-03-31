use super::{Implied, Instruction, InstructionName, Modify};
use crate::address::AddressMap;
use crate::cpu::state::{registers::Flag, CPU};
use std::{cell::RefCell, rc::Rc};

/// Represents the ASL instruction (http://www.obelisk.me.uk/6502/reference.html#ASL)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ASL;

impl ASL {
    fn set_flags_and_return(cpu: &Rc<RefCell<CPU>>, arg: u8) -> u8 {
        let result: u8 = arg << 1;
        if arg & 0b1000_0000 == 0 {
            cpu.borrow_mut().registers.clear_flag(Flag::C);
        } else {
            cpu.borrow_mut().registers.set_flag(Flag::C);
        }
        if result == 0 {
            cpu.borrow_mut().registers.set_flag(Flag::Z);
        }
        if result & 0b1000_0000 != 0 {
            cpu.borrow_mut().registers.set_flag(Flag::N);
        }
        result
    }
}

impl Instruction for ASL {
    fn name(&self) -> InstructionName {
        InstructionName::ASL
    }
}

impl Modify for ASL {
    fn execute(&self, cpu: &Rc<RefCell<CPU>>, addr: u16, old_val: u8) {
        let new_val: u8 = Self::set_flags_and_return(cpu, old_val);
        cpu.borrow_mut().memory.set(addr, new_val);
    }
}

impl Implied for ASL {
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
    fn test_asl_c() {
        let mut cpu: CPU = CPU::mock();
        cpu.registers.clear_flag(Flag::C);
        let cpu = Rc::new(RefCell::new(cpu));
        ASL::set_flags_and_return(&cpu, 0b1100_0001);
        assert_eq!(cpu.borrow().registers.get_flag(Flag::C), 1);
        ASL::set_flags_and_return(&cpu, 0b0010_1011);
        assert_eq!(cpu.borrow().registers.get_flag(Flag::C), 0);
    }

    #[test]
    fn test_asl_z() {
        let mut cpu: CPU = CPU::mock();
        cpu.registers.clear_flag(Flag::Z);
        let cpu = Rc::new(RefCell::new(cpu));
        ASL::set_flags_and_return(&cpu, 0b1000_0000);
        assert_eq!(cpu.borrow().registers.get_flag(Flag::Z), 1);
        cpu.borrow_mut().registers.clear_flag(Flag::Z);
        ASL::set_flags_and_return(&cpu, 0b0010_1011);
        assert_eq!(cpu.borrow().registers.get_flag(Flag::Z), 0);
    }

    #[test]
    fn test_asl_n() {
        let mut cpu: CPU = CPU::mock();
        cpu.registers.clear_flag(Flag::N);
        let cpu = Rc::new(RefCell::new(cpu));
        ASL::set_flags_and_return(&cpu, 0b0100_0000);
        assert_eq!(cpu.borrow().registers.get_flag(Flag::N), 1);
        cpu.borrow_mut().registers.clear_flag(Flag::N);
        ASL::set_flags_and_return(&cpu, 0b1010_1011);
        assert_eq!(cpu.borrow().registers.get_flag(Flag::N), 0);
    }

    #[test]
    fn test_asl_modify() {
        let cpu = Rc::new(RefCell::new(CPU::mock()));
        Modify::execute(&ASL, &cpu, 0x2013, 0b1001_1100);
        assert_eq!(cpu.borrow().memory.get(0x2013), 0b0011_1000);
    }

    #[test]
    fn test_asl_implied() {
        let mut cpu: CPU = CPU::mock();
        cpu.registers.a = 0b0110_0010;
        let cpu = Rc::new(RefCell::new(cpu));
        Implied::execute(&ASL, &cpu);
        assert_eq!(cpu.borrow().registers.a, 0b1100_0100);
    }
}
