use super::{Instruction, InstructionName, Modify};
use crate::bitops::BitOps;
use crate::cpu::variables::Flag;
use crate::state::CPU;

/// Represents the INC instruction (http://www.obelisk.me.uk/6502/reference.html#INC)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct INC;

impl Instruction for INC {
    fn name(&self) -> InstructionName {
        InstructionName::INC
    }
}

impl<S: CPU> Modify<S> for INC {
    fn execute(&self, cpu: &mut S, addr: u16, old_val: u8) {
        let result: u8 = old_val.wrapping_add(1);
        cpu.set_mem(addr, result);
        cpu.assign_flag(Flag::Z, result == 0);
        cpu.assign_flag(Flag::N, result.is_bit_set(7));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::cpu::{Memory, Registers};
    use crate::state::NES;

    #[test]
    fn test_inc() {
        let mut cpu = NES::mock();
        cpu.set_mem(0x3209, 100);
        INC.execute(&mut cpu, 0x3209, 100);
        assert_eq!(cpu.get_mem(0x3209), 101);
    }

    #[test]
    fn test_inc_z() {
        let mut cpu = NES::mock();
        cpu.assign_flag(Flag::Z, false);
        cpu.set_mem(0x3209, 100);
        INC.execute(&mut cpu, 0x3209, 100);
        assert_eq!(cpu.is_flag_set(Flag::Z), false);
        cpu.set_mem(0x3209, 255);
        INC.execute(&mut cpu, 0x3209, 255);
        assert_eq!(cpu.is_flag_set(Flag::Z), true);
    }

    #[test]
    fn test_inc_n() {
        let mut cpu = NES::mock();
        cpu.assign_flag(Flag::N, false);
        cpu.set_mem(0x3209, 100);
        INC.execute(&mut cpu, 0x3209, 100);
        assert_eq!(cpu.is_flag_set(Flag::N), false);
        cpu.set_mem(0x3209, 204);
        INC.execute(&mut cpu, 0x3209, 204);
        assert_eq!(cpu.is_flag_set(Flag::N), true);
    }
}
