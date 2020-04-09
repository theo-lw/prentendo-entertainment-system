use super::{Implied, Instruction, InstructionName, Modify};
use crate::bitops::BitOps;
use crate::cpu::variables::Flag;
use crate::state::CPU;

/// Represents the ASL instruction (http://www.obelisk.me.uk/6502/reference.html#ASL)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ASL;

impl ASL {
    fn set_flags_and_return(cpu: &mut dyn CPU, arg: u8) -> u8 {
        let result: u8 = arg << 1;
        cpu.assign_flag(Flag::C, arg.is_bit_set(7));
        cpu.assign_flag(Flag::Z, result == 0);
        cpu.assign_flag(Flag::N, result.is_bit_set(7));
        result
    }
}

impl Instruction for ASL {
    fn name(&self) -> InstructionName {
        InstructionName::ASL
    }
}

impl<S: CPU> Modify<S> for ASL {
    fn execute(&self, cpu: &mut S, addr: u16, old_val: u8) {
        let new_val: u8 = Self::set_flags_and_return(cpu, old_val);
        cpu.set_mem(addr, new_val);
    }
}

impl<S: CPU> Implied<S> for ASL {
    fn execute(&self, cpu: &mut S) {
        let a_register = cpu.get_a();
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
    fn test_asl_c() {
        let mut cpu = NES::mock();
        cpu.clear_flag(Flag::C);
        ASL::set_flags_and_return(&mut cpu, 0b1100_0001);
        assert_eq!(cpu.is_flag_set(Flag::C), true);
        ASL::set_flags_and_return(&mut cpu, 0b0010_1011);
        assert_eq!(cpu.is_flag_set(Flag::C), false);
    }

    #[test]
    fn test_asl_z() {
        let mut cpu = NES::mock();
        cpu.clear_flag(Flag::Z);
        ASL::set_flags_and_return(&mut cpu, 0b1000_0000);
        assert_eq!(cpu.is_flag_set(Flag::Z), true);
        cpu.clear_flag(Flag::Z);
        ASL::set_flags_and_return(&mut cpu, 0b0010_1011);
        assert_eq!(cpu.is_flag_set(Flag::Z), false);
    }

    #[test]
    fn test_asl_n() {
        let mut cpu = NES::mock();
        cpu.clear_flag(Flag::N);
        ASL::set_flags_and_return(&mut cpu, 0b0100_0000);
        assert_eq!(cpu.is_flag_set(Flag::N), true);
        cpu.clear_flag(Flag::N);
        ASL::set_flags_and_return(&mut cpu, 0b1010_1011);
        assert_eq!(cpu.is_flag_set(Flag::N), false);
    }

    #[test]
    fn test_asl_modify() {
        let mut cpu = NES::mock();
        Modify::execute(&ASL, &mut cpu, 0x2013, 0b1001_1100);
        assert_eq!(cpu.get_mem(0x2013), 0b0011_1000);
    }

    #[test]
    fn test_asl_implied() {
        let mut cpu = NES::mock();
        cpu.set_a(0b0110_0010);
        Implied::execute(&ASL, &mut cpu);
        assert_eq!(cpu.get_a(), 0b1100_0100);
    }
}
