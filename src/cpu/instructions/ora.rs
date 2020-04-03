use super::{Instruction, InstructionName, Read};
use crate::address::AddressMap;
use crate::bitops::BitOps;
use crate::cpu::state::CPU;
use crate::cpu::variables::Flag;
use std::{cell::RefCell, rc::Rc};

/// Represents the ORA instruction (http://www.obelisk.me.uk/6502/reference.html#ORA)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ORA;

impl Instruction for ORA {
    fn name(&self) -> InstructionName {
        InstructionName::ORA
    }
}

impl Read for ORA {
    fn execute(&self, cpu: &Rc<RefCell<CPU>>, addr: u16) {
        let byte: u8 = cpu.borrow().memory.get(addr);
        cpu.borrow_mut().registers.a |= byte;
        if cpu.borrow().registers.a == 0 {
            cpu.borrow_mut().registers.set_flag(Flag::Z);
        }
        if cpu.borrow().registers.a.is_bit_set(7) {
            cpu.borrow_mut().registers.set_flag(Flag::N);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ora() {
        let mut cpu = CPU::mock();
        cpu.memory.set(0x4304, 0b1001_0110);
        cpu.registers.a = 0b1000_0101;
        let cpu = Rc::new(RefCell::new(cpu));
        ORA.execute(&cpu, 0x4304);
        assert_eq!(cpu.borrow().registers.a, 0b1001_0111);
    }

    #[test]
    fn test_ora_z() {
        let mut cpu = CPU::mock();
        cpu.memory.set(0x4304, 0b1001_0110);
        cpu.registers.a = 0b1000_0101;
        cpu.registers.clear_flag(Flag::Z);
        let cpu = Rc::new(RefCell::new(cpu));
        ORA.execute(&cpu, 0x4304);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::Z), false);
        cpu.borrow_mut().memory.set(0x4304, 0);
        cpu.borrow_mut().registers.a = 0;
        ORA.execute(&cpu, 0x4304);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::Z), true);
    }

    #[test]
    fn test_ora_n() {
        let mut cpu = CPU::mock();
        cpu.memory.set(0x4304, 0b0101_0110);
        cpu.registers.a = 0b0000_0101;
        cpu.registers.clear_flag(Flag::N);
        let cpu = Rc::new(RefCell::new(cpu));
        ORA.execute(&cpu, 0x4304);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::N), false);
        cpu.borrow_mut().memory.set(0x4304, 0b1011_0001);
        ORA.execute(&cpu, 0x4304);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::N), true);
    }
}
