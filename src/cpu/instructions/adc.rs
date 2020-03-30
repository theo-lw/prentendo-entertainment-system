use super::{Instruction, InstructionName, Read};
use crate::address::AddressMap;
use crate::cpu::state::{registers::Flag, CPU};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ADC;

impl Instruction for ADC {
    fn name(&self) -> InstructionName {
        InstructionName::ADC
    }
}

impl Read for ADC {
    fn execute(&self, cpu: &Rc<RefCell<CPU>>, addr: u16) {
        let byte: u8 = cpu.borrow().memory.get(addr);
        let c: u8 = cpu.borrow().registers.get_flag(Flag::C);
        let a: u8 = cpu.borrow().registers.a;
        let (result, overflow1): (u8, bool) = a.overflowing_add(byte);
        let (result, overflow2): (u8, bool) = result.overflowing_add(c);
        if result & (1 << 7) != 0 {
            cpu.borrow_mut().registers.set_flag(Flag::N);
        }
        if result == 0 {
            cpu.borrow_mut().registers.set_flag(Flag::Z);
        }
        if overflow1 || overflow2 {
            cpu.borrow_mut().registers.set_flag(Flag::C);
        }
        // if result's sign is opposite to a and byte has the same sign as a
        if (result ^ a) & !(byte ^ a) & 0b1000_0000 != 0 {
            cpu.borrow_mut().registers.set_flag(Flag::V);
        }
        cpu.borrow_mut().registers.a = result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adc_read() {
        let mut cpu: CPU = CPU::mock();
        cpu.registers.p = 0b0010_0000;
        cpu.registers.set_flag(Flag::C);
        cpu.registers.a = 132;
        cpu.memory.set(cpu.registers.pc, 40);
        let cpu = Rc::new(RefCell::new(cpu));
        let addr: u16 = cpu.borrow().registers.pc;
        ADC.execute(&cpu, addr);
        assert_eq!(cpu.borrow().registers.a, 173);
    }

    #[test]
    fn test_adc_read_n() {
        let mut cpu: CPU = CPU::mock();
        cpu.registers.p = 0b0010_0000;
        cpu.registers.a = 0b0100_0000;
        cpu.memory.set(cpu.registers.pc, 0b1000_0000);
        let cpu = Rc::new(RefCell::new(cpu));
        let addr: u16 = cpu.borrow().registers.pc;
        ADC.execute(&cpu, addr);
        assert_eq!(cpu.borrow().registers.get_flag(Flag::N), 1);
        cpu.borrow_mut().registers.p = 0b0010_0000;
        cpu.borrow_mut().memory.set(addr, 0b0100_0000);
        cpu.borrow_mut().registers.a = 0b0010_0000;
        ADC.execute(&cpu, addr);
        assert_eq!(cpu.borrow().registers.get_flag(Flag::N), 0);
    }

    #[test]
    fn test_adc_read_z() {
        let mut cpu: CPU = CPU::mock();
        cpu.registers.p = 0b0010_0000;
        cpu.registers.a = 0b0100_0000;
        cpu.memory.set(cpu.registers.pc, 0b1000_0000);
        let cpu = Rc::new(RefCell::new(cpu));
        let addr: u16 = cpu.borrow().registers.pc;
        ADC.execute(&cpu, addr);
        assert_eq!(cpu.borrow().registers.get_flag(Flag::Z), 0);
        cpu.borrow_mut().registers.p = 0b0010_0000;
        cpu.borrow_mut().memory.set(addr, 0);
        cpu.borrow_mut().registers.a = 0;
        ADC.execute(&cpu, addr);
        assert_eq!(cpu.borrow().registers.get_flag(Flag::Z), 1);
    }

    #[test]
    fn test_adc_read_c() {
        let mut cpu: CPU = CPU::mock();
        cpu.registers.p = 0b0010_0000;
        cpu.registers.a = 0b1111_1111;
        cpu.memory.set(cpu.registers.pc, 0b1000_0000);
        let cpu = Rc::new(RefCell::new(cpu));
        let addr: u16 = cpu.borrow().registers.pc;
        ADC.execute(&cpu, addr);
        assert_eq!(cpu.borrow().registers.get_flag(Flag::C), 1);
        cpu.borrow_mut().registers.p = 0b0010_0000;
        cpu.borrow_mut().memory.set(addr, 0b0100_0000);
        cpu.borrow_mut().registers.a = 0b0010_0000;
        ADC.execute(&cpu, addr);
        assert_eq!(cpu.borrow().registers.get_flag(Flag::C), 0);
    }

    #[test]
    fn test_adc_read_v() {
        let mut cpu: CPU = CPU::mock();
        cpu.registers.p = 0b0010_0000;
        cpu.registers.a = 64i8 as u8;
        cpu.memory.set(cpu.registers.pc, 72i8 as u8);
        let cpu = Rc::new(RefCell::new(cpu));
        let addr: u16 = cpu.borrow().registers.pc;
        ADC.execute(&cpu, addr);
        assert_eq!(cpu.borrow().registers.get_flag(Flag::V), 1);
        cpu.borrow_mut().registers.p = 0b0010_0000;
        cpu.borrow_mut().memory.set(addr, 4i8 as u8);
        cpu.borrow_mut().registers.a = 43i8 as u8;
        ADC.execute(&cpu, addr);
        assert_eq!(cpu.borrow().registers.get_flag(Flag::N), 0);
    }
}
