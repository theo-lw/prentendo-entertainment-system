use super::{Instruction, InstructionName, Read};
use crate::bitops::BitOps;
use crate::cpu::variables::Flag;
use crate::state::CPU;

/// Represents the ORA instruction (http://www.obelisk.me.uk/6502/reference.html#ORA)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ORA;

impl Instruction for ORA {
    fn name(&self) -> InstructionName {
        InstructionName::ORA
    }
}

impl<S: CPU> Read<S> for ORA {
    fn execute(&self, cpu: &mut S, addr: u16) {
        cpu.set_a(cpu.get_a() | cpu.get_mem(addr));
        cpu.assign_flag(Flag::Z, cpu.get_a() == 0);
        cpu.assign_flag(Flag::N, cpu.get_a().is_bit_set(7));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::cpu::{Memory, Registers};
    use crate::state::NES;

    #[test]
    fn test_ora() {
        let mut cpu = NES::mock();
        cpu.set_mem(0x19, 0b1001_0110);
        cpu.set_a(0b1000_0101);
        ORA.execute(&mut cpu, 0x19);
        assert_eq!(cpu.get_a(), 0b1001_0111);
    }

    #[test]
    fn test_ora_z() {
        let mut cpu = NES::mock();
        cpu.set_mem(0x19, 0b1001_0110);
        cpu.set_a(0b1000_0101);
        cpu.clear_flag(Flag::Z);
        ORA.execute(&mut cpu, 0x19);
        assert_eq!(cpu.is_flag_set(Flag::Z), false);
        cpu.set_mem(0x19, 0);
        cpu.set_a(0);
        ORA.execute(&mut cpu, 0x19);
        assert_eq!(cpu.is_flag_set(Flag::Z), true);
    }

    #[test]
    fn test_ora_n() {
        let mut cpu = NES::mock();
        cpu.set_mem(0x19, 0b0101_0110);
        cpu.set_a(0b0000_0101);
        cpu.clear_flag(Flag::N);
        ORA.execute(&mut cpu, 0x19);
        assert_eq!(cpu.is_flag_set(Flag::N), false);
        cpu.set_mem(0x19, 0b1011_0001);
        ORA.execute(&mut cpu, 0x19);
        assert_eq!(cpu.is_flag_set(Flag::N), true);
    }
}
