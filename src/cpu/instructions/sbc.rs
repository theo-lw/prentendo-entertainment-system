use super::{Instruction, InstructionName, Read};
use crate::address::AddressMap;
use crate::bitops::BitOps;
use crate::cpu::state::CPU;
use crate::cpu::variables::Flag;
use std::{cell::RefCell, rc::Rc};

/// Represents the SBC instruction (http://www.obelisk.me.uk/6502/reference.html#SBC)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SBC;

impl Instruction for SBC {
    fn name(&self) -> InstructionName {
        InstructionName::SBC
    }
}

impl Read for SBC {
    fn execute(&self, cpu: &Rc<RefCell<CPU>>, addr: u16) {
        let byte: u8 = cpu.borrow().memory.get(addr);
        let carry: u8 = if cpu.borrow().registers.is_flag_set(Flag::C) {
            0
        } else {
            1
        };
        let a_register: u8 = cpu.borrow().registers.a;
        let (result, overflow1): (u8, bool) = a_register.overflowing_sub(byte);
        let (result, overflow2): (u8, bool) = result.overflowing_sub(carry);
        if result.is_bit_set(7) {
            cpu.borrow_mut().registers.set_flag(Flag::N);
        }
        if result == 0 {
            cpu.borrow_mut().registers.set_flag(Flag::Z);
        }
        if overflow1 || overflow2 {
            cpu.borrow_mut().registers.clear_flag(Flag::C);
        }
        // if result's sign is opposite to a's and byte's sign is opposite to a's
        if ((result ^ a_register) & (byte ^ a_register)).is_bit_set(7) {
            cpu.borrow_mut().registers.set_flag(Flag::V);
        }
        cpu.borrow_mut().registers.a = result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sbc() {
        let mut cpu: CPU = CPU::mock();
        cpu.registers.clear_flag(Flag::C);
        cpu.registers.a = 132;
        cpu.memory.set(cpu.registers.pc, 40);
        let cpu = Rc::new(RefCell::new(cpu));
        let addr: u16 = cpu.borrow().registers.pc;
        SBC.execute(&cpu, addr);
        assert_eq!(cpu.borrow().registers.a, 91);
    }

    #[test]
    fn test_sbc_n() {
        let mut cpu: CPU = CPU::mock();
        cpu.registers.clear_flag(Flag::N);
        cpu.registers.a = 0b0100_0000;
        cpu.memory.set(cpu.registers.pc, 0b1000_0000);
        let cpu = Rc::new(RefCell::new(cpu));
        let addr: u16 = cpu.borrow().registers.pc;
        SBC.execute(&cpu, addr);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::N), true);
        cpu.borrow_mut().registers.clear_flag(Flag::N);
        cpu.borrow_mut().memory.set(addr, 0b0100_0000);
        cpu.borrow_mut().registers.a = 0b1010_0000;
        SBC.execute(&cpu, addr);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::N), false);
    }

    #[test]
    fn test_sbc_z() {
        let mut cpu: CPU = CPU::mock();
        cpu.registers.clear_flag(Flag::Z);
        cpu.registers.a = 0b0100_0000;
        cpu.memory.set(cpu.registers.pc, 0b1000_0000);
        let cpu = Rc::new(RefCell::new(cpu));
        let addr: u16 = cpu.borrow().registers.pc;
        SBC.execute(&cpu, addr);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::Z), false);
        cpu.borrow_mut().registers.clear_flag(Flag::Z);
        cpu.borrow_mut().registers.set_flag(Flag::C);
        cpu.borrow_mut().memory.set(addr, 0b1001_0010);
        cpu.borrow_mut().registers.a = 0b1001_0010;
        SBC.execute(&cpu, addr);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::Z), true);
    }

    #[test]
    fn test_sbc_c() {
        let mut cpu: CPU = CPU::mock();
        cpu.registers.set_flag(Flag::C);
        cpu.registers.a = 0b1000_0000;
        cpu.memory.set(cpu.registers.pc, 0b0010_0000);
        let cpu = Rc::new(RefCell::new(cpu));
        let addr: u16 = cpu.borrow().registers.pc;
        SBC.execute(&cpu, addr);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::C), true);
        cpu.borrow_mut().memory.set(addr, 0b1100_0000);
        cpu.borrow_mut().registers.a = 0b1010_0000;
        SBC.execute(&cpu, addr);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::C), false);
    }

    #[test]
    fn test_sbc_v() {
        let mut cpu: CPU = CPU::mock();
        cpu.registers.clear_flag(Flag::V);
        cpu.registers.a = 64i8 as u8;
        cpu.memory.set(cpu.registers.pc, -72i8 as u8);
        let cpu = Rc::new(RefCell::new(cpu));
        let addr: u16 = cpu.borrow().registers.pc;
        SBC.execute(&cpu, addr);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::V), true);
        cpu.borrow_mut().registers.clear_flag(Flag::V);
        cpu.borrow_mut().memory.set(addr, 4i8 as u8);
        cpu.borrow_mut().registers.a = 43i8 as u8;
        SBC.execute(&cpu, addr);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::V), false);
    }
}
