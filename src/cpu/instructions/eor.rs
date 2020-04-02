use super::{Instruction, InstructionName, Read};
use crate::address::AddressMap;
use crate::bitops::BitOps;
use crate::cpu::state::CPU;
use crate::cpu::variables::Flag;
use std::{cell::RefCell, rc::Rc};

/// Represents the EOR instruction (http://www.obelisk.me.uk/6502/reference.html#EOR)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct EOR;

impl Instruction for EOR {
    fn name(&self) -> InstructionName {
        InstructionName::EOR
    }
}

impl Read for EOR {
    fn execute(&self, cpu: &Rc<RefCell<CPU>>, addr: u16) {
        let byte: u8 = cpu.borrow().memory.get(addr);
        cpu.borrow_mut().registers.a ^= byte;
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
    fn test_eor() {
        let mut cpu = CPU::mock();
        cpu.memory.set(0x4304, 0b1001_0110);
        cpu.registers.a = 0b1000_0101;
        let cpu = Rc::new(RefCell::new(cpu));
        EOR.execute(&cpu, 0x4304);
        assert_eq!(cpu.borrow().registers.a, 0b0001_0011);
    }

    #[test]
    fn test_eor_z() {
        let mut cpu = CPU::mock();
        cpu.memory.set(0x4304, 0b1001_0110);
        cpu.registers.a = 0b1000_0101;
        cpu.registers.clear_flag(Flag::Z);
        let cpu = Rc::new(RefCell::new(cpu));
        EOR.execute(&cpu, 0x4304);
        assert_eq!(cpu.borrow().registers.get_flag(Flag::Z), 0);
        cpu.borrow_mut().registers.a = 0b1011_0101;
        cpu.borrow_mut().memory.set(0x4304, 0b1011_0101);
        EOR.execute(&cpu, 0x4304);
        assert_eq!(cpu.borrow().registers.get_flag(Flag::Z), 1);
    }

    #[test]
    fn test_eor_n() {
        let mut cpu = CPU::mock();
        cpu.memory.set(0x4304, 0b1101_0110);
        cpu.registers.a = 0b1000_0101;
        cpu.registers.clear_flag(Flag::N);
        let cpu = Rc::new(RefCell::new(cpu));
        EOR.execute(&cpu, 0x4304);
        assert_eq!(cpu.borrow().registers.get_flag(Flag::N), 0);
        cpu.borrow_mut().registers.a = 0b0001_0010;
        cpu.borrow_mut().memory.set(0x4304, 0b1011_0001);
        EOR.execute(&cpu, 0x4304);
        assert_eq!(cpu.borrow().registers.get_flag(Flag::N), 1);
    }
}
