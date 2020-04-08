use super::{Implied, Instruction, InstructionName};
use crate::bitops::BitOps;
use crate::state::CPU;
use crate::cpu::variables::{Flag, Get, Set};

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
    use crate::cpu::variables::{x_register::X, y_register::Y};
    use crate::state::NES;
    use crate::state::cpu::Registers;

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
        cpu.clear_flag(Flag::Z);
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
        cpu.clear_flag(Flag::N);
        cpu.set_x(100);
        IN(X).execute(&mut cpu);
        assert_eq!(cpu.is_flag_set(Flag::N), false);
        cpu.set_x(235);
        IN(X).execute(&mut cpu);
        assert_eq!(cpu.is_flag_set(Flag::N), true);
    }
}
