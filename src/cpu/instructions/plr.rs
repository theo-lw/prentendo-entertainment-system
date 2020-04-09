use super::{Instruction, InstructionName, PullStack};
use crate::bitops::BitOps;
use crate::cpu::variables::{Flag, Set};
use crate::state::CPU;

/// Represents the 'pull stack' instructions
/// (http://www.obelisk.me.uk/6502/reference.html#PLA)
/// (http://www.obelisk.me.uk/6502/reference.html#PLP)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PL<T: Set>(pub T);

impl<T: Set> Instruction for PL<T> {
    fn name(&self) -> InstructionName {
        InstructionName::PL(self.0.name())
    }
}

impl<T: Set, S: CPU> PullStack<S> for PL<T> {
    fn set(&self, cpu: &mut S, val: u8) {
        cpu.assign_flag(Flag::Z, val == 0);
        cpu.assign_flag(Flag::N, val.is_bit_set(7));
        self.0.set(cpu, val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::variables::{a_register::A, p_register::P};
    use crate::state::cpu::Registers;
    use crate::state::NES;

    #[test]
    fn test_plp() {
        let mut cpu = NES::mock();
        cpu.set_p(0);
        PL(P).set(&mut cpu, 0b0101_1110);
        assert_eq!(cpu.get_p(), 0b0110_1110);
    }

    #[test]
    fn test_pla_z() {
        let mut cpu = NES::mock();
        cpu.clear_flag(Flag::Z);
        PL(A).set(&mut cpu, 12);
        assert_eq!(cpu.is_flag_set(Flag::Z), false);
        PL(A).set(&mut cpu, 0);
        assert_eq!(cpu.is_flag_set(Flag::Z), true);
    }

    #[test]
    fn test_pl_n() {
        let mut cpu = NES::mock();
        cpu.clear_flag(Flag::N);
        PL(A).set(&mut cpu, 0);
        assert_eq!(cpu.is_flag_set(Flag::N), false);
        PL(A).set(&mut cpu, 0b1100_0010);
        assert_eq!(cpu.is_flag_set(Flag::N), true);
    }
}
