use super::{Flag, Get, Register, RegisterName, Set};
use crate::bitops::BitOps;
use crate::state::CPU;

/// Represents the P register
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct P;

impl Register for P {
    fn name(&self) -> RegisterName {
        RegisterName::P
    }

    fn flags_set_on_change(&self) -> bool {
        true
    }
}

impl Get for P {
    fn get(&self, cpu: &dyn CPU) -> u8 {
        let mut result = cpu.get_p();
        // this bit is set because of how the PHP instruction works
        result.set_bit(Flag::B as usize);
        result
    }
}

impl Set for P {
    fn set(&self, cpu: &mut dyn CPU, val: u8) {
        cpu.set_p(val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::cpu::Registers;
    use crate::state::NES;

    #[test]
    fn test_get() {
        let mut cpu = NES::mock();
        cpu.set_p(0b0110_1001);
        assert_eq!(P.get(&cpu), 0b0111_1001);
    }

    #[test]
    fn test_set() {
        let mut cpu = NES::mock();
        cpu.set_p(40);
        P.set(&mut cpu, 0b0101_1110);
        assert_eq!(cpu.get_p(), 0b0110_1110);
    }
}
