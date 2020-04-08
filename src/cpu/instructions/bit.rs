use super::{Instruction, InstructionName, Read};
use crate::bitops::BitOps;
use crate::state::CPU;
use crate::cpu::variables::Flag;

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
        if cpu.get_a() & byte == 0 {
            cpu.set_flag(Flag::Z);
        }
        if byte.is_bit_set(7) {
            cpu.set_flag(Flag::N);
        } else {
            cpu.clear_flag(Flag::N);
        }
        if byte.is_bit_set(6) {
            cpu.set_flag(Flag::V);
        } else {
            cpu.clear_flag(Flag::V);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::NES;
    use crate::state::cpu::{Registers, Memory};

    #[test]
    fn test_bit_z() {
        let mut cpu = NES::mock();
        cpu.set_a(0b1001_1110);
        cpu.set_mem(0x0, 0b0110_0001);
        cpu.set_mem(0x1, 0b1000_0000);
        cpu.clear_flag(Flag::Z);
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
        cpu.clear_flag(Flag::N);
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
        cpu.clear_flag(Flag::V);
        BIT.execute(&mut cpu, 0x0);
        assert_eq!(cpu.is_flag_set(Flag::V), true);
        BIT.execute(&mut cpu, 0x1);
        assert_eq!(cpu.is_flag_set(Flag::V), false);
    }
}
