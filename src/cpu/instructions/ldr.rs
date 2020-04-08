use super::{Instruction, InstructionName, Read};
use crate::bitops::BitOps;
use crate::state::CPU;
use crate::cpu::variables::{Flag, Set};

/// Represents the LD instructions
/// (http://www.obelisk.me.uk/6502/reference.html#LDA)
/// (http://www.obelisk.me.uk/6502/reference.html#LDX)
/// (http://www.obelisk.me.uk/6502/reference.html#LDY)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct LD<T: Set>(pub T);

impl<T: Set> Instruction for LD<T> {
    fn name(&self) -> InstructionName {
        InstructionName::LD(self.0.name())
    }
}

impl<T: Set, S: CPU> Read<S> for LD<T> {
    fn execute(&self, cpu: &mut S, addr: u16) {
        let byte: u8 = cpu.get_mem(addr);
        self.0.set(cpu, byte);
        if byte == 0 {
            cpu.set_flag(Flag::Z);
        }
        if byte.is_bit_set(7) {
            cpu.set_flag(Flag::N);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::variables::{a_register::A, x_register::X, y_register::Y};
    use crate::state::NES;
    use crate::state::cpu::{Registers, Memory};

    #[test]
    fn test_ld() {
        let mut cpu = NES::mock();
        cpu.set_mem(0x31, 0b1001_0110);
        cpu.set_a(0b1000_0101);
        LD(A).execute(&mut cpu, 0x31);
        assert_eq!(cpu.get_a(), 0b1001_0110);
    }

    #[test]
    fn test_ld_z() {
        let mut cpu = NES::mock();
        cpu.set_mem(0x31, 0b1001_0110);
        cpu.set_x(0b1000_0101);
        cpu.clear_flag(Flag::Z);
        LD(X).execute(&mut cpu, 0x31);
        assert_eq!(cpu.is_flag_set(Flag::Z), false);
        cpu.set_mem(0x31, 0);
        LD(X).execute(&mut cpu, 0x31);
        assert_eq!(cpu.is_flag_set(Flag::Z), true);
    }

    #[test]
    fn test_ld_n() {
        let mut cpu = NES::mock();
        cpu.set_mem(0x31, 0b0101_0110);
        cpu.set_y(0b1000_0101);
        cpu.clear_flag(Flag::N);
        LD(Y).execute(&mut cpu, 0x31);
        assert_eq!(cpu.is_flag_set(Flag::N), false);
        cpu.set_a(0b1001_0010);
        cpu.set_mem(0x31, 0b1011_0001);
        LD(Y).execute(&mut cpu, 0x31);
        assert_eq!(cpu.is_flag_set(Flag::N), true);
    }
}
