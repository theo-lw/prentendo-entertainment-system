use super::{Get, Register, RegisterName, Set};
use crate::state::CPU;

/// Represents the P register
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct P;

impl Register for P {
    fn name(&self) -> RegisterName {
        RegisterName::P
    }
}

impl Get for P {
    fn get(&self, cpu: &dyn CPU) -> u8 {
        cpu.get_p() | 0b0011_0000
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
    use crate::state::NES;
    use crate::state::cpu::Registers;

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
        P.set(&mut cpu, 94);
        assert_eq!(cpu.get_p(), 94);
    }
}
