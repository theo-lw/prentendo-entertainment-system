use super::{Implied, Instruction, InstructionName};
use crate::bitops::BitOps;
use crate::cpu::variables::{Flag, Get, Set};
use crate::state::CPU;

/// Represents the 'transfer' instructions.
/// (http://www.obelisk.me.uk/6502/reference.html#TAX)
/// (http://www.obelisk.me.uk/6502/reference.html#TAY)
/// (http://www.obelisk.me.uk/6502/reference.html#TSX)
/// (http://www.obelisk.me.uk/6502/reference.html#TXA)
/// (http://www.obelisk.me.uk/6502/reference.html#TXS)
/// (http://www.obelisk.me.uk/6502/reference.html#TYA)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct T<U: Get, V: Set>(pub U, pub V);

impl<U: Get, V: Set> Instruction for T<U, V> {
    fn name(&self) -> InstructionName {
        InstructionName::T(self.0.name(), self.1.name())
    }
}

impl<U: Get, V: Set, S: CPU> Implied<S> for T<U, V> {
    fn execute(&self, cpu: &mut S) {
        let result: u8 = self.0.get(cpu);
        self.1.set(cpu, result);
        if self.1.flags_set_on_change() {
            cpu.assign_flag(Flag::Z, result == 0);
            cpu.assign_flag(Flag::N, result.is_bit_set(7));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::variables::{a_register::A, stack_pointer::S, x_register::X, y_register::Y};
    use crate::state::cpu::Registers;
    use crate::state::NES;

    #[test]
    fn test_txs() {
        let mut cpu = NES::mock();
        cpu.set_x(42);
        cpu.set_s(30);
        T(X, S).execute(&mut cpu);
        assert_eq!(cpu.get_s(), 42);
    }

    #[test]
    fn test_tay_z() {
        let mut cpu = NES::mock();
        cpu.assign_flag(Flag::Z, false);
        cpu.set_a(198);
        cpu.set_x(12);
        T(A, Y).execute(&mut cpu);
        assert_eq!(cpu.is_flag_set(Flag::Z), false);
        cpu.set_a(0);
        T(A, Y).execute(&mut cpu);
        assert_eq!(cpu.is_flag_set(Flag::Z), true);
    }

    #[test]
    fn test_txa_n() {
        let mut cpu = NES::mock();
        cpu.assign_flag(Flag::N, false);
        cpu.set_a(198);
        cpu.set_x(12);
        T(X, A).execute(&mut cpu);
        assert_eq!(cpu.is_flag_set(Flag::N), false);
        cpu.set_x(-42i8 as u8);
        T(X, A).execute(&mut cpu);
        assert_eq!(cpu.is_flag_set(Flag::N), true);
    }
}
