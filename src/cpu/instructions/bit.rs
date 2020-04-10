use super::{Instruction, InstructionName, Read};
use crate::bitops::BitOps;
use crate::cpu::variables::Flag;
use crate::state::CPU;

/// Represents the BIT instruction (http://www.obelisk.me.uk/6502/reference.html#BIT)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BIT;

impl Instruction for BIT {
    fn name(&self) -> InstructionName {
        InstructionName::BIT
    }
}

impl<S: CPU> Read<S> for BIT {
    fn execute(&self, cpu: &mut S, addr: u16) {
        let byte: u8 = cpu.get_mem(addr);
        cpu.assign_flag(Flag::Z, cpu.get_a() & byte == 0);
        cpu.assign_flag(Flag::N, byte.is_bit_set(7));
        cpu.assign_flag(Flag::V, byte.is_bit_set(6));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::cpu::{Memory, Registers};
    use crate::state::NES;

    #[test]
    fn test_bit_z() {
        let mut cpu = NES::mock();
        cpu.set_a(0b1001_1110);
        cpu.set_mem(0x0, 0b0110_0001);
        cpu.set_mem(0x1, 0b1000_0000);
        cpu.assign_flag(Flag::Z, false);
        BIT.execute(&mut cpu, 0x1);
        assert_eq!(cpu.is_flag_set(Flag::Z), false);
        BIT.execute(&mut cpu, 0x0);
        assert_eq!(cpu.is_flag_set(Flag::Z), true);
    }

    #[test]
    fn test_bit_n() {
        let mut cpu = NES::mock();
        cpu.set_a(0b1001_1110);
        cpu.set_mem(0x0, 0b0110_0001);
        cpu.set_mem(0x1, 0b1000_0000);
        cpu.assign_flag(Flag::N, false);
        BIT.execute(&mut cpu, 0x1);
        assert_eq!(cpu.is_flag_set(Flag::N), true);
        BIT.execute(&mut cpu, 0x0);
        assert_eq!(cpu.is_flag_set(Flag::N), false);
    }

    #[test]
    fn test_bit_v() {
        let mut cpu = NES::mock();
        cpu.set_a(0b1001_1110);
        cpu.set_mem(0x0, 0b0110_0001);
        cpu.set_mem(0x1, 0b1000_0000);
        cpu.assign_flag(Flag::V, false);
        BIT.execute(&mut cpu, 0x0);
        assert_eq!(cpu.is_flag_set(Flag::V), true);
        BIT.execute(&mut cpu, 0x1);
        assert_eq!(cpu.is_flag_set(Flag::V), false);
    }
}
