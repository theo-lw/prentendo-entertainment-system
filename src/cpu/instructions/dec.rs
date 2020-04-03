use super::{Instruction, InstructionName, Modify};
use crate::address::AddressMap;
use crate::bitops::BitOps;
use crate::cpu::state::CPU;
use crate::cpu::variables::Flag;
use std::{cell::RefCell, rc::Rc};

/// Represents the DEC instruction (http://www.obelisk.me.uk/6502/reference.html#DEC)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct DEC;

impl Instruction for DEC {
    fn name(&self) -> InstructionName {
        InstructionName::DEC
    }
}

impl Modify for DEC {
    fn execute(&self, cpu: &Rc<RefCell<CPU>>, addr: u16, old_val: u8) {
        let result: u8 = old_val.wrapping_sub(1);
        cpu.borrow_mut().memory.set(addr, result);
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

    #[test]
    fn test_dec() {
        let mut cpu = CPU::mock();
        cpu.memory.set(0x3209, 100);
        let cpu = Rc::new(RefCell::new(cpu));
        DEC.execute(&cpu, 0x3209, 100);
        assert_eq!(cpu.borrow().memory.get(0x3209), 99);
    }

    #[test]
    fn test_dec_z() {
        let mut cpu = CPU::mock();
        cpu.registers.clear_flag(Flag::Z);
        cpu.memory.set(0x3209, 100);
        let cpu = Rc::new(RefCell::new(cpu));
        DEC.execute(&cpu, 0x3209, 100);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::Z), false);
        cpu.borrow_mut().memory.set(0x3209, 1);
        DEC.execute(&cpu, 0x3209, 1);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::Z), true);
    }

    #[test]
    fn test_dec_n() {
        let mut cpu = CPU::mock();
        cpu.registers.clear_flag(Flag::N);
        cpu.memory.set(0x3209, 100);
        let cpu = Rc::new(RefCell::new(cpu));
        DEC.execute(&cpu, 0x3209, 100);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::N), false);
        cpu.borrow_mut().memory.set(0x3209, 0);
        DEC.execute(&cpu, 0x3209, 0);
        assert_eq!(cpu.borrow().registers.is_flag_set(Flag::N), true);
    }
}
