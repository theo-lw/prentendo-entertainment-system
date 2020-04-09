use super::{Instruction, InstructionName, Read};
use crate::bitops::BitOps;
use crate::cpu::variables::Flag;
use crate::state::CPU;

/// Represents the EOR instruction (http://www.obelisk.me.uk/6502/reference.html#EOR)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct EOR;

impl Instruction for EOR {
    fn name(&self) -> InstructionName {
        InstructionName::EOR
    }
}

impl<S: CPU> Read<S> for EOR {
    fn execute(&self, cpu: &mut S, addr: u16) {
        cpu.set_a(cpu.get_a() ^ cpu.get_mem(addr));
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
    fn test_eor() {
        let mut cpu = NES::mock();
        cpu.set_mem(0x23, 0b1001_0110);
        cpu.set_a(0b1000_0101);
        EOR.execute(&mut cpu, 0x23);
        assert_eq!(cpu.get_a(), 0b0001_0011);
    }

    #[test]
    fn test_eor_z() {
        let mut cpu = NES::mock();
        cpu.set_mem(0x23, 0b1001_0110);
        cpu.set_a(0b1000_0101);
        cpu.clear_flag(Flag::Z);
        EOR.execute(&mut cpu, 0x23);
        assert_eq!(cpu.is_flag_set(Flag::Z), false);
        cpu.set_a(0b1011_0101);
        cpu.set_mem(0x23, 0b1011_0101);
        EOR.execute(&mut cpu, 0x23);
        assert_eq!(cpu.is_flag_set(Flag::Z), true);
    }

    #[test]
    fn test_eor_n() {
        let mut cpu = NES::mock();
        cpu.set_mem(0x23, 0b1101_0110);
        cpu.set_a(0b1000_0101);
        cpu.clear_flag(Flag::N);
        EOR.execute(&mut cpu, 0x23);
        assert_eq!(cpu.is_flag_set(Flag::N), false);
        cpu.set_a(0b0001_0010);
        cpu.set_mem(0x23, 0b1011_0001);
        EOR.execute(&mut cpu, 0x23);
        assert_eq!(cpu.is_flag_set(Flag::N), true);
    }
}
