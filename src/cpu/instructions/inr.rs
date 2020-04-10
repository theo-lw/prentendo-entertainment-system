use super::{Implied, Instruction, InstructionName};
use crate::bitops::BitOps;
use crate::cpu::variables::{Flag, Get, Set};
use crate::state::CPU;

/// Represents the 'increment register' instructions
/// (http://www.obelisk.me.uk/6502/reference.html#INX)
/// (http://www.obelisk.me.uk/6502/reference.html#INY)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct IN<T: Get + Set>(pub T);

impl<T: Get + Set> Instruction for IN<T> {
    fn name(&self) -> InstructionName {
        InstructionName::IN(self.0.name())
    }
}

impl<T: Get + Set, S: CPU> Implied<S> for IN<T> {
    fn execute(&self, cpu: &mut S) {
        let result: u8 = self.0.get(cpu).wrapping_add(1);
        self.0.set(cpu, result);
        cpu.assign_flag(Flag::Z, result == 0);
        cpu.assign_flag(Flag::N, result.is_bit_set(7));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::variables::{x_register::X, y_register::Y};
    use crate::state::cpu::Registers;
    use crate::state::NES;

    #[test]
    fn test_in() {
        let mut cpu = NES::mock();
        cpu.set_x(100);
        IN(X).execute(&mut cpu);
        assert_eq!(cpu.get_x(), 101);
    }

    #[test]
    fn test_in_z() {
        let mut cpu = NES::mock();
        cpu.assign_flag(Flag::Z, false);
        cpu.set_y(100);
        IN(Y).execute(&mut cpu);
        assert_eq!(cpu.is_flag_set(Flag::Z), false);
        cpu.set_y(255);
        IN(Y).execute(&mut cpu);
        assert_eq!(cpu.is_flag_set(Flag::Z), true);
    }

    #[test]
    fn test_in_n() {
        let mut cpu = NES::mock();
        cpu.assign_flag(Flag::N, false);
        cpu.set_x(100);
        IN(X).execute(&mut cpu);
        assert_eq!(cpu.is_flag_set(Flag::N), false);
        cpu.set_x(235);
        IN(X).execute(&mut cpu);
        assert_eq!(cpu.is_flag_set(Flag::N), true);
    }
}
