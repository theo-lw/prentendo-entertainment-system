use super::{Get, Register, RegisterName, Set};
use crate::state::CPU;

/// Represents the X register
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct X;

impl Register for X {
    fn name(&self) -> RegisterName {
        RegisterName::X
    }

    fn flags_set_on_change(&self) -> bool {
        true
    }
}

impl Get for X {
    fn get(&self, cpu: &dyn CPU) -> u8 {
        cpu.get_x()
    }
}

impl Set for X {
    fn set(&self, cpu: &mut dyn CPU, val: u8) {
        cpu.set_x(val);
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
        cpu.set_x(23);
        assert_eq!(X.get(&cpu), 23);
    }

    #[test]
    fn test_set() {
        let mut cpu = NES::mock();
        cpu.set_x(40);
        X.set(&mut cpu, 94);
        assert_eq!(cpu.get_x(), 94);
    }
}
