use super::{Instruction, InstructionName, Read};
use crate::bitops::BitOps;
use crate::address::AddressMap;
use crate::cpu::state::CPU;
use crate::cpu::variables::Flag;
use std::{cell::RefCell, rc::Rc};

/// Represents the BIT instruction (http://www.obelisk.me.uk/6502/reference.html#BIT)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BIT;

impl Instruction for BIT {
    fn name(&self) -> InstructionName {
        InstructionName::BIT
    }
}

impl Read for BIT {
    fn execute(&self, cpu: &Rc<RefCell<CPU>>, addr: u16) {
        let byte: u8 = cpu.borrow().memory.get(addr);
        if cpu.borrow().registers.a & byte == 0 {
            cpu.borrow_mut().registers.set_flag(Flag::Z);
        }
        if byte.is_bit_set(7) {
            cpu.borrow_mut().registers.set_flag(Flag::N);
        } else {
            cpu.borrow_mut().registers.clear_flag(Flag::N);
        }
        if byte.is_bit_set(6) {
            cpu.borrow_mut().registers.set_flag(Flag::V);
        } else {
            cpu.borrow_mut().registers.clear_flag(Flag::V);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_z() {
        let mut cpu = CPU::mock();
        cpu.registers.a = 0b1001_1110;
        cpu.memory.set(0x4032, 0b0110_0001);
        cpu.memory.set(0x2234, 0b1000_0000);
        cpu.registers.clear_flag(Flag::Z);
        let cpu = Rc::new(RefCell::new(cpu));
        BIT.execute(&cpu, 0x2234);
        assert_eq!(cpu.borrow().registers.get_flag(Flag::Z), 0);
        BIT.execute(&cpu, 0x4032);
        assert_eq!(cpu.borrow().registers.get_flag(Flag::Z), 1);
    }

    #[test]
    fn test_bit_n() {
        let mut cpu = CPU::mock();
        cpu.registers.a = 0b1001_1110;
        cpu.memory.set(0x4032, 0b0110_0001);
        cpu.memory.set(0x2234, 0b1000_0000);
        cpu.registers.clear_flag(Flag::N);
        let cpu = Rc::new(RefCell::new(cpu));
        BIT.execute(&cpu, 0x2234);
        assert_eq!(cpu.borrow().registers.get_flag(Flag::N), 1);
        BIT.execute(&cpu, 0x4032);
        assert_eq!(cpu.borrow().registers.get_flag(Flag::N), 0);
    }

    #[test]
    fn test_bit_v() {
        let mut cpu = CPU::mock();
        cpu.registers.a = 0b1001_1110;
        cpu.memory.set(0x4032, 0b0110_0001);
        cpu.memory.set(0x2234, 0b1000_0000);
        cpu.registers.clear_flag(Flag::V);
        let cpu = Rc::new(RefCell::new(cpu));
        BIT.execute(&cpu, 0x4032);
        assert_eq!(cpu.borrow().registers.get_flag(Flag::V), 1);
        BIT.execute(&cpu, 0x2234);
        assert_eq!(cpu.borrow().registers.get_flag(Flag::V), 0);
    }
}
