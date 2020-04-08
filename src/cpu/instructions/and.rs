use super::{Instruction, InstructionName, Read};
use crate::bitops::BitOps;
use crate::state::CPU;
use crate::cpu::variables::Flag;

/// Represents the AND instruction (http://www.obelisk.me.uk/6502/reference.html#AND)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AND;

impl Instruction for AND {
    fn name(&self) -> InstructionName {
        InstructionName::AND
    }
}

impl<S: CPU> Read<S> for AND {
    fn execute(&self, cpu: &mut S, addr: u16) {
        cpu.set_a(cpu.get_a() & cpu.get_mem(addr));
        if cpu.get_a() == 0 {
            cpu.set_flag(Flag::Z);
        }
        if cpu.get_a().is_bit_set(7) {
            cpu.set_flag(Flag::N);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::NES;
    use crate::state::cpu::{Registers, Memory};

    #[test]
    fn test_and() {
        let mut cpu = NES::mock();
        cpu.set_mem(0x07FF, 0b1001_0110);
        cpu.set_a(0b1000_0101);
        AND.execute(&mut cpu, 0x07FF);
        assert_eq!(cpu.get_a(), 0b1000_0100);
    }

    #[test]
    fn test_and_z() {
        let mut cpu = NES::mock();
        cpu.set_mem(0x07FF, 0b1001_0110);
        cpu.set_a(0b1000_0101);
        cpu.clear_flag(Flag::Z);
        AND.execute(&mut cpu, 0x07FF);
        assert_eq!(cpu.is_flag_set(Flag::Z), false);
        cpu.set_mem(0x07FF, 0);
        AND.execute(&mut cpu, 0x07FF);
        assert_eq!(cpu.is_flag_set(Flag::Z), true);
    }

    #[test]
    fn test_and_n() {
        let mut cpu = NES::mock();
        cpu.set_mem(0x07FF, 0b0101_0110);
        cpu.set_a(0b1000_0101);
        cpu.clear_flag(Flag::N);
        AND.execute(&mut cpu, 0x07FF);
        assert_eq!(cpu.is_flag_set(Flag::N), false);
        cpu.set_a(0b1001_0010);
        cpu.set_mem(0x07FF, 0b1011_0001);
        AND.execute(&mut cpu, 0x07FF);
        assert_eq!(cpu.is_flag_set(Flag::N), true);
    }
}
