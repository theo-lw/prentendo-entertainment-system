use super::{Implied, Instruction, InstructionName, Modify};
use crate::bitops::BitOps;
use crate::cpu::variables::Flag;
use crate::state::CPU;

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
        cpu.assign_flag(Flag::C, arg.is_bit_set(7));
        cpu.assign_flag(Flag::Z, result == 0);
        cpu.assign_flag(Flag::N, result.is_bit_set(7));
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
    use crate::state::cpu::{Memory, Registers};
    use crate::state::NES;

    #[test]
    fn test_rol_c() {
        let mut cpu = NES::mock();
        cpu.assign_flag(Flag::C, false);
        ROL::set_flags_and_return(&mut cpu, 0b1100_0000);
        assert_eq!(cpu.is_flag_set(Flag::C), true);
        ROL::set_flags_and_return(&mut cpu, 0b0010_1011);
        assert_eq!(cpu.is_flag_set(Flag::C), false);
    }

    #[test]
    fn test_rol_z() {
        let mut cpu = NES::mock();
        cpu.assign_flag(Flag::Z, false);
        ROL::set_flags_and_return(&mut cpu, 0b1000_0000);
        assert_eq!(cpu.is_flag_set(Flag::Z), true);
        cpu.assign_flag(Flag::Z, false);
        ROL::set_flags_and_return(&mut cpu, 0b0010_1011);
        assert_eq!(cpu.is_flag_set(Flag::Z), false);
    }

    #[test]
    fn test_rol_n() {
        let mut cpu = NES::mock();
        cpu.assign_flag(Flag::N, false);
        ROL::set_flags_and_return(&mut cpu, 0b0100_0000);
        assert_eq!(cpu.is_flag_set(Flag::N), true);
        cpu.assign_flag(Flag::N, false);
        ROL::set_flags_and_return(&mut cpu, 0b1010_1011);
        assert_eq!(cpu.is_flag_set(Flag::N), false);
    }

    #[test]
    fn test_rol_modify() {
        let mut cpu = NES::mock();
        cpu.assign_flag(Flag::C, true);
        Modify::execute(&ROL, &mut cpu, 0x2013, 0b1001_1100);
        assert_eq!(cpu.get_mem(0x2013), 0b0011_1001);
    }

    #[test]
    fn test_rol_implied() {
        let mut cpu = NES::mock();
        cpu.assign_flag(Flag::C, false);
        cpu.set_a(0b0110_0010);
        Implied::execute(&ROL, &mut cpu);
        assert_eq!(cpu.get_a(), 0b1100_0100);
    }
}
