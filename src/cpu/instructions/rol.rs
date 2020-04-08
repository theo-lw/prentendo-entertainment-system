use super::{Implied, Instruction, InstructionName, Modify};
use crate::bitops::BitOps;
use crate::state::CPU;
use crate::cpu::variables::Flag;

/// Represents the ROL instruction (http://www.obelisk.me.uk/6502/reference.html#ROL)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ROL;

impl ROL {
    fn set_flags_and_return(cpu: &mut dyn CPU, arg: u8) -> u8 {
        let mut result: u8 = arg << 1;
        if cpu.is_flag_set(Flag::C) {
            result.set_bit(0);
        } else {
            result.clear_bit(0);
        }
        if arg.is_bit_set(7) {
            cpu.set_flag(Flag::C);
        } else {
            cpu.clear_flag(Flag::C);
        }
        if result == 0 {
            cpu.set_flag(Flag::Z);
        }
        if result.is_bit_set(7) {
            cpu.set_flag(Flag::N);
        }
        result
    }
}

impl Instruction for ROL {
    fn name(&self) -> InstructionName {
        InstructionName::ROL
    }
}

impl<S: CPU> Modify<S> for ROL {
    fn execute(&self, cpu: &mut S, addr: u16, old_val: u8) {
        let new_val: u8 = Self::set_flags_and_return(cpu, old_val);
        cpu.set_mem(addr, new_val);
    }
}

impl<S: CPU> Implied<S> for ROL {
    fn execute(&self, cpu: &mut S) {
        let a_register: u8 = cpu.get_a();
        let new_val: u8 = Self::set_flags_and_return(cpu, a_register);
        cpu.set_a(new_val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::NES;
    use crate::state::cpu::{Registers, Memory};

    #[test]
    fn test_rol_c() {
        let mut cpu = NES::mock();
        cpu.clear_flag(Flag::C);
        ROL::set_flags_and_return(&mut cpu, 0b1100_0000);
        assert_eq!(cpu.is_flag_set(Flag::C), true);
        ROL::set_flags_and_return(&mut cpu, 0b0010_1011);
        assert_eq!(cpu.is_flag_set(Flag::C), false);
    }

    #[test]
    fn test_rol_z() {
        let mut cpu = NES::mock();
        cpu.clear_flag(Flag::Z);
        ROL::set_flags_and_return(&mut cpu, 0b1000_0000);
        assert_eq!(cpu.is_flag_set(Flag::Z), true);
        cpu.clear_flag(Flag::Z);
        ROL::set_flags_and_return(&mut cpu, 0b0010_1011);
        assert_eq!(cpu.is_flag_set(Flag::Z), false);
    }

    #[test]
    fn test_rol_n() {
        let mut cpu = NES::mock();
        cpu.clear_flag(Flag::N);
        ROL::set_flags_and_return(&mut cpu, 0b0100_0000);
        assert_eq!(cpu.is_flag_set(Flag::N), true);
        cpu.clear_flag(Flag::N);
        ROL::set_flags_and_return(&mut cpu, 0b1010_1011);
        assert_eq!(cpu.is_flag_set(Flag::N), false);
    }

    #[test]
    fn test_rol_modify() {
        let mut cpu = NES::mock();
        cpu.set_flag(Flag::C);
        Modify::execute(&ROL, &mut cpu, 0x2013, 0b1001_1100);
        assert_eq!(cpu.get_mem(0x2013), 0b0011_1001);
    }

    #[test]
    fn test_rol_implied() {
        let mut cpu = NES::mock();
        cpu.clear_flag(Flag::C);
        cpu.set_a(0b0110_0010);
        Implied::execute(&ROL, &mut cpu);
        assert_eq!(cpu.get_a(), 0b1100_0100);
    }
}
