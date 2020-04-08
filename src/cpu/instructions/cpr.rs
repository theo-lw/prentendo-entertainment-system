use super::{Instruction, InstructionName, Read};
use crate::bitops::BitOps;
use crate::state::CPU;
use crate::cpu::variables::{Flag, Get};

/// Represents the 'compare' instructions (CMP, CPX, CPY)
/// (http://www.obelisk.me.uk/6502/reference.html#CMP)
/// (http://www.obelisk.me.uk/6502/reference.html#CPX)
/// (http://www.obelisk.me.uk/6502/reference.html#CPY)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CP<T: Get>(pub T);

impl<T: Get> Instruction for CP<T> {
    fn name(&self) -> InstructionName {
        InstructionName::CP(self.0.name())
    }
}

impl<T: Get, S: CPU> Read<S> for CP<T> {
    fn execute(&self, cpu: &mut S, addr: u16) {
        let byte: u8 = cpu.get_mem(addr);
        let register: u8 = self.0.get(cpu);
        let (result, overflow): (u8, bool) = register.overflowing_sub(byte);
        if !overflow {
            cpu.set_flag(Flag::C);
        }
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
    use crate::cpu::variables::{a_register::A, x_register::X, y_register::Y};
    use crate::state::NES;
    use crate::state::cpu::{Registers, Memory};

    #[test]
    fn test_cp_c() {
        let mut cpu = NES::mock();
        cpu.set_mem(0x30, 13);
        cpu.set_x(2);
        cpu.clear_flag(Flag::C);
        CP(X).execute(&mut cpu, 0x30);
        assert_eq!(cpu.is_flag_set(Flag::C), false);
        cpu.set_x(30);
        CP(X).execute(&mut cpu, 0x30);
        assert_eq!(cpu.is_flag_set(Flag::C), true);
    }

    #[test]
    fn test_cp_z() {
        let mut cpu = NES::mock();
        cpu.set_mem(0x30, 30);
        cpu.set_y(50);
        cpu.clear_flag(Flag::Z);
        CP(Y).execute(&mut cpu, 0x30);
        assert_eq!(cpu.is_flag_set(Flag::Z), false);
        cpu.set_mem(0x30, 50);
        CP(Y).execute(&mut cpu, 0x30);
        assert_eq!(cpu.is_flag_set(Flag::Z), true);
    }

    #[test]
    fn test_cp_n() {
        let mut cpu = NES::mock();
        cpu.set_mem(0x30, 40);
        cpu.set_a(45);
        cpu.clear_flag(Flag::N);
        CP(A).execute(&mut cpu, 0x30);
        assert_eq!(cpu.is_flag_set(Flag::N), false);
        cpu.set_a(12);
        CP(A).execute(&mut cpu, 0x30);
        assert_eq!(cpu.is_flag_set(Flag::N), true);
    }
}
