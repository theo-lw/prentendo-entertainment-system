use super::{Instruction, InstructionName, Modify};
use crate::bitops::BitOps;
use crate::state::CPU;
use crate::cpu::variables::Flag;

/// Represents the DEC instruction (http://www.obelisk.me.uk/6502/reference.html#DEC)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct DEC;

impl Instruction for DEC {
    fn name(&self) -> InstructionName {
        InstructionName::DEC
    }
}

impl<S: CPU> Modify<S> for DEC {
    fn execute(&self, cpu: &mut S, addr: u16, old_val: u8) {
        let result: u8 = old_val.wrapping_sub(1);
        cpu.set_mem(addr, result);
        if result == 0 {
            cpu.set_flag(Flag::Z);
        }
        if result.is_bit_set(7) {
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
    fn test_dec() {
        let mut cpu = NES::mock();
        cpu.set_mem(0x3209, 100);
        DEC.execute(&mut cpu, 0x3209, 100);
        assert_eq!(cpu.get_mem(0x3209), 99);
    }

    #[test]
    fn test_dec_z() {
        let mut cpu = NES::mock();
        cpu.clear_flag(Flag::Z);
        cpu.set_mem(0x3209, 100);
        DEC.execute(&mut cpu, 0x3209, 100);
        assert_eq!(cpu.is_flag_set(Flag::Z), false);
        cpu.set_mem(0x3209, 1);
        DEC.execute(&mut cpu, 0x3209, 1);
        assert_eq!(cpu.is_flag_set(Flag::Z), true);
    }

    #[test]
    fn test_dec_n() {
        let mut cpu = NES::mock();
        cpu.clear_flag(Flag::N);
        cpu.set_mem(0x3209, 100);
        DEC.execute(&mut cpu, 0x3209, 100);
        assert_eq!(cpu.is_flag_set(Flag::N), false);
        cpu.set_mem(0x3209, 0);
        DEC.execute(&mut cpu, 0x3209, 0);
        assert_eq!(cpu.is_flag_set(Flag::N), true);
    }
}
