use super::{Implied, Instruction, InstructionName, Modify};
use crate::bitops::BitOps;
use crate::cpu::variables::Flag;
use crate::state::CPU;

/// Represents the LSR instruction (http://www.obelisk.me.uk/6502/reference.html#LSR)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct LSR;

impl LSR {
    fn set_flags_and_return(cpu: &mut dyn CPU, arg: u8) -> u8 {
        let result: u8 = arg >> 1;
        cpu.assign_flag(Flag::C, arg.is_bit_set(0));
        cpu.assign_flag(Flag::Z, result == 0);
        cpu.assign_flag(Flag::N, result.is_bit_set(7));
        result
    }
}

impl Instruction for LSR {
    fn name(&self) -> InstructionName {
        InstructionName::LSR
    }
}

impl<S: CPU> Modify<S> for LSR {
    fn execute(&self, cpu: &mut S, addr: u16, old_val: u8) {
        let new_val: u8 = Self::set_flags_and_return(cpu, old_val);
        cpu.set_mem(addr, new_val);
    }
}

impl<S: CPU> Implied<S> for LSR {
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
    fn test_lsr_c() {
        let mut cpu = NES::mock();
        cpu.clear_flag(Flag::C);
        LSR::set_flags_and_return(&mut cpu, 0b0100_0001);
        assert_eq!(cpu.is_flag_set(Flag::C), true);
        LSR::set_flags_and_return(&mut cpu, 0b1010_1010);
        assert_eq!(cpu.is_flag_set(Flag::C), false);
    }

    #[test]
    fn test_lsr_z() {
        let mut cpu = NES::mock();
        cpu.clear_flag(Flag::Z);
        LSR::set_flags_and_return(&mut cpu, 0b0000_0001);
        assert_eq!(cpu.is_flag_set(Flag::Z), true);
        cpu.clear_flag(Flag::Z);
        LSR::set_flags_and_return(&mut cpu, 0b1000_0000);
        assert_eq!(cpu.is_flag_set(Flag::Z), false);
    }

    #[test]
    fn test_lsr_n() {
        let mut cpu = NES::mock();
        cpu.clear_flag(Flag::N);
        LSR::set_flags_and_return(&mut cpu, 0b0100_0001);
        assert_eq!(cpu.is_flag_set(Flag::N), false);
    }

    #[test]
    fn test_lsr_modify() {
        let mut cpu = NES::mock();
        Modify::execute(&LSR, &mut cpu, 0x2013, 0b1001_1100);
        assert_eq!(cpu.get_mem(0x2013), 0b0100_1110);
    }

    #[test]
    fn test_lsr_implied() {
        let mut cpu = NES::mock();
        cpu.set_a(0b0110_0010);
        Implied::execute(&LSR, &mut cpu);
        assert_eq!(cpu.get_a(), 0b0011_0001);
    }
}
