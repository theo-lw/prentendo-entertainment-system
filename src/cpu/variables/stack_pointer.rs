use super::{Get, Register, RegisterName, Set};
use crate::state::CPU;

/// Represents the Stack Pointer
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct S;

impl Register for S {
    fn name(&self) -> RegisterName {
        RegisterName::S
    }
}

impl Get for S {
    fn get(&self, cpu: &dyn CPU) -> u8 {
        cpu.get_s()
    }
}

impl Set for S {
    fn set(&self, cpu: &mut dyn CPU, val: u8) {
        cpu.set_s(val);
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
        cpu.set_s(23);
        assert_eq!(S.get(&cpu), 23);
    }

    #[test]
    fn test_set() {
        let mut cpu = NES::mock();
        cpu.set_s(40);
        S.set(&mut cpu, 94);
        assert_eq!(cpu.get_s(), 94);
    }
}
