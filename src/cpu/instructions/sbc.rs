use super::{Instruction, InstructionName, Read};
use crate::bitops::BitOps;
use crate::cpu::variables::Flag;
use crate::state::CPU;

/// Represents the SBC instruction (http://www.obelisk.me.uk/6502/reference.html#SBC)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SBC;

impl Instruction for SBC {
    fn name(&self) -> InstructionName {
        InstructionName::SBC
    }
}

impl<S: CPU> Read<S> for SBC {
    fn execute(&self, cpu: &mut S, addr: u16) {
        let byte: u8 = cpu.get_mem(addr);
        let carry: u8 = if cpu.is_flag_set(Flag::C) { 0 } else { 1 };
        let a_register: u8 = cpu.get_a();
        let (result, overflow1): (u8, bool) = a_register.overflowing_sub(byte);
        let (result, overflow2): (u8, bool) = result.overflowing_sub(carry);
        cpu.assign_flag(Flag::N, result.is_bit_set(7));
        cpu.assign_flag(Flag::Z, result == 0);
        cpu.assign_flag(Flag::C, !(overflow1 || overflow2));
        cpu.assign_flag(
            Flag::V,
            ((result ^ a_register) & (byte ^ a_register)).is_bit_set(7),
        );
        cpu.set_a(result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::cpu::{Memory, Registers};
    use crate::state::NES;

    #[test]
    fn test_sbc() {
        let mut cpu = NES::mock();
        cpu.assign_flag(Flag::C, false);
        cpu.set_a(132);
        cpu.set_mem(cpu.get_pc(), 40);
        let addr: u16 = cpu.get_pc();
        SBC.execute(&mut cpu, addr);
        assert_eq!(cpu.get_a(), 91);
    }

    #[test]
    fn test_sbc_n() {
        let mut cpu = NES::mock();
        cpu.assign_flag(Flag::N, false);
        cpu.set_a(0b0100_0000);
        cpu.set_mem(cpu.get_pc(), 0b1000_0000);
        let addr: u16 = cpu.get_pc();
        SBC.execute(&mut cpu, addr);
        assert_eq!(cpu.is_flag_set(Flag::N), true);
        cpu.assign_flag(Flag::N, false);
        cpu.set_mem(addr, 0b0100_0000);
        cpu.set_a(0b1010_0000);
        SBC.execute(&mut cpu, addr);
        assert_eq!(cpu.is_flag_set(Flag::N), false);
    }

    #[test]
    fn test_sbc_z() {
        let mut cpu = NES::mock();
        cpu.assign_flag(Flag::Z, false);
        cpu.set_a(0b0100_0000);
        cpu.set_mem(cpu.get_pc(), 0b1000_0000);
        let addr: u16 = cpu.get_pc();
        SBC.execute(&mut cpu, addr);
        assert_eq!(cpu.is_flag_set(Flag::Z), false);
        cpu.assign_flag(Flag::Z, false);
        cpu.assign_flag(Flag::C, true);
        cpu.set_mem(addr, 0b1001_0010);
        cpu.set_a(0b1001_0010);
        SBC.execute(&mut cpu, addr);
        assert_eq!(cpu.is_flag_set(Flag::Z), true);
    }

    #[test]
    fn test_sbc_c() {
        let mut cpu = NES::mock();
        cpu.assign_flag(Flag::C, true);
        cpu.set_a(0b1000_0000);
        cpu.set_mem(cpu.get_pc(), 0b0010_0000);
        let addr: u16 = cpu.get_pc();
        SBC.execute(&mut cpu, addr);
        assert_eq!(cpu.is_flag_set(Flag::C), true);
        cpu.set_mem(addr, 0b1100_0000);
        cpu.set_a(0b1010_0000);
        SBC.execute(&mut cpu, addr);
        assert_eq!(cpu.is_flag_set(Flag::C), false);
    }

    #[test]
    fn test_sbc_v() {
        let mut cpu = NES::mock();
        cpu.assign_flag(Flag::V, false);
        cpu.set_a(64i8 as u8);
        cpu.set_mem(cpu.get_pc(), -72i8 as u8);
        let addr: u16 = cpu.get_pc();
        SBC.execute(&mut cpu, addr);
        assert_eq!(cpu.is_flag_set(Flag::V), true);
        cpu.assign_flag(Flag::V, false);
        cpu.set_mem(addr, 4i8 as u8);
        cpu.set_a(43i8 as u8);
        SBC.execute(&mut cpu, addr);
        assert_eq!(cpu.is_flag_set(Flag::V), false);
    }
}
