use super::{Implied, Instruction, InstructionName};
use crate::bitops::BitOps;
use crate::state::CPU;
use crate::cpu::variables::{Flag, Get, Set};

/// Represents the 'decrement register' instructions
/// (http://www.obelisk.me.uk/6502/reference.html#DEX)
/// (http://www.obelisk.me.uk/6502/reference.html#DEY)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct DE<T: Get + Set>(pub T);

impl<T: Get + Set> Instruction for DE<T> {
    fn name(&self) -> InstructionName {
        InstructionName::DE(self.0.name())
    }
}

impl<T: Get + Set, S: CPU> Implied<S> for DE<T> {
    fn execute(&self, cpu: &mut S) {
        let result: u8 = self.0.get(cpu).wrapping_sub(1);
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
    fn test_dex() {
        let mut cpu = NES::mock();
        cpu.set_x(100);
        DE(X).execute(&mut cpu);
        assert_eq!(cpu.get_x(), 99);
    }

    #[test]
    fn test_dex_z() {
        let mut cpu = NES::mock();
        cpu.clear_flag(Flag::Z);
        cpu.set_x(100);
        DE(X).execute(&mut cpu);
        assert_eq!(cpu.is_flag_set(Flag::Z), false);
        cpu.set_x(1);
        DE(X).execute(&mut cpu);
        assert_eq!(cpu.is_flag_set(Flag::Z), true);
    }

    #[test]
    fn test_dex_n() {
        let mut cpu = NES::mock();
        cpu.clear_flag(Flag::N);
        cpu.set_y(100);
        DE(Y).execute(&mut cpu);
        assert_eq!(cpu.is_flag_set(Flag::N), false);
        cpu.set_y(0);
        DE(Y).execute(&mut cpu);
        assert_eq!(cpu.is_flag_set(Flag::N), true);
    }
}
