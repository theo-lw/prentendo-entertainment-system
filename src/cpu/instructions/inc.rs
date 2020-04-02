use super::{Instruction, InstructionName, Modify};
use crate::bitops::BitOps;
use crate::address::AddressMap;
use crate::cpu::state::CPU;
use crate::cpu::variables::Flag;
use std::{cell::RefCell, rc::Rc};

/// Represents the INC instruction (http://www.obelisk.me.uk/6502/reference.html#INC)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct INC;

impl Instruction for INC {
    fn name(&self) -> InstructionName {
        InstructionName::INC
    }
}

impl Modify for INC {
    fn execute(&self, cpu: &Rc<RefCell<CPU>>, addr: u16, old_val: u8) {
        let result: u8 = old_val.wrapping_add(1);
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
    fn test_inc() {
        let mut cpu = CPU::mock();
        cpu.memory.set(0x3209, 100);
        let cpu = Rc::new(RefCell::new(cpu));
        INC.execute(&cpu, 0x3209, 100);
        assert_eq!(cpu.borrow().memory.get(0x3209), 101);
    }

    #[test]
    fn test_inc_z() {
        let mut cpu = CPU::mock();
        cpu.registers.clear_flag(Flag::Z);
        cpu.memory.set(0x3209, 100);
        let cpu = Rc::new(RefCell::new(cpu));
        INC.execute(&cpu, 0x3209, 100);
        assert_eq!(cpu.borrow().registers.get_flag(Flag::Z), 0);
        cpu.borrow_mut().memory.set(0x3209, 255);
        INC.execute(&cpu, 0x3209, 255);
        assert_eq!(cpu.borrow().registers.get_flag(Flag::Z), 1);
    }

    #[test]
    fn test_inc_n() {
        let mut cpu = CPU::mock();
        cpu.registers.clear_flag(Flag::N);
        cpu.memory.set(0x3209, 100);
        let cpu = Rc::new(RefCell::new(cpu));
        INC.execute(&cpu, 0x3209, 100);
        assert_eq!(cpu.borrow().registers.get_flag(Flag::N), 0);
        cpu.borrow_mut().memory.set(0x3209, 204);
        INC.execute(&cpu, 0x3209, 204);
        assert_eq!(cpu.borrow().registers.get_flag(Flag::N), 1);
    }
}
