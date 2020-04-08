use super::{Implied, Instruction, InstructionName, Modify};
use crate::bitops::BitOps;
use crate::state::CPU;
use crate::cpu::variables::Flag;

/// Represents the ROR instruction (http://www.obelisk.me.uk/6502/reference.html#ROR)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ROR;

impl ROR {
    fn set_flags_and_return(cpu: &mut dyn CPU, arg: u8) -> u8 {
        let mut result: u8 = arg >> 1;
        if cpu.is_flag_set(Flag::C) {
            result.set_bit(7);
        } else {
            result.clear_bit(7);
        }
        if arg.is_bit_set(0) {
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

impl Instruction for ROR {
    fn name(&self) -> InstructionName {
        InstructionName::ROR
    }
}

impl<S: CPU> Modify<S> for ROR {
    fn execute(&self, cpu: &mut S, addr: u16, old_val: u8) {
        let new_val: u8 = Self::set_flags_and_return(cpu, old_val);
        cpu.set_mem(addr, new_val);
    }
}

impl<S: CPU> Implied<S> for ROR {
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
        ROR::set_flags_and_return(&mut cpu, 0b0100_0001);
        assert_eq!(cpu.is_flag_set(Flag::C), true);
        ROR::set_flags_and_return(&mut cpu, 0b1010_1010);
        assert_eq!(cpu.is_flag_set(Flag::C), false);
    }

    #[test]
    fn test_rol_z() {
        let mut cpu = NES::mock();
        cpu.clear_flag(Flag::Z);
        ROR::set_flags_and_return(&mut cpu, 0b0000_0001);
        assert_eq!(cpu.is_flag_set(Flag::Z), true);
        cpu.clear_flag(Flag::Z);
        ROR::set_flags_and_return(&mut cpu, 0b0010_1011);
        assert_eq!(cpu.is_flag_set(Flag::Z), false);
    }

    #[test]
    fn test_rol_n() {
        let mut cpu = NES::mock();
        cpu.clear_flag(Flag::N);
        cpu.set_flag(Flag::C);
        ROR::set_flags_and_return(&mut cpu, 0b0100_0000);
        assert_eq!(cpu.is_flag_set(Flag::N), true);
        cpu.clear_flag(Flag::N);
        ROR::set_flags_and_return(&mut cpu, 0b1010_1011);
        assert_eq!(cpu.is_flag_set(Flag::N), false);
    }

    #[test]
    fn test_rol_modify() {
        let mut cpu = NES::mock();
        cpu.set_flag(Flag::C);
        Modify::execute(&ROR, &mut cpu, 0x2013, 0b1001_1100);
        assert_eq!(cpu.get_mem(0x2013), 0b1100_1110);
    }

    #[test]
    fn test_rol_implied() {
        let mut cpu = NES::mock();
        cpu.clear_flag(Flag::C);
        cpu.set_a(0b0110_0010);
        Implied::execute(&ROR, &mut cpu);
        assert_eq!(cpu.get_a(), 0b0011_0001);
    }
}
