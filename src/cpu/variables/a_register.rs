use super::{Get, Register, RegisterName, Set};
use crate::state::CPU;

/// Represents the A register
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct A;

impl Register for A {
    fn name(&self) -> RegisterName {
        RegisterName::A
    }

    fn flags_set_on_change(&self) -> bool {
        true
    }
}

impl Get for A {
    fn get(&self, cpu: &dyn CPU) -> u8 {
        cpu.get_a()
    }
}

impl Set for A {
    fn set(&self, cpu: &mut dyn CPU, val: u8) {
        cpu.set_a(val);
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
        cpu.set_a(23);
        assert_eq!(A.get(&cpu), 23);
    }

    #[test]
    fn test_set() {
        let mut cpu = NES::mock();
        cpu.set_a(40);
        A.set(&mut cpu, 94);
        assert_eq!(cpu.get_a(), 94);
    }
}
