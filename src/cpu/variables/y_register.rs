use super::{Get, Register, RegisterName, Set};
use crate::state::CPU;

/// Represents the Y register
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Y;

impl Register for Y {
    fn name(&self) -> RegisterName {
        RegisterName::Y
    }

    fn flags_set_on_change(&self) -> bool {
        true
    }
}

impl Get for Y {
    fn get(&self, cpu: &dyn CPU) -> u8 {
        cpu.get_y()
    }
}

impl Set for Y {
    fn set(&self, cpu: &mut dyn CPU, val: u8) {
        cpu.set_y(val);
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
        cpu.set_y(23);
        assert_eq!(Y.get(&cpu), 23);
    }

    #[test]
    fn test_set() {
        let mut cpu = NES::mock();
        cpu.set_y(40);
        Y.set(&mut cpu, 94);
        assert_eq!(cpu.get_y(), 94);
    }
}
