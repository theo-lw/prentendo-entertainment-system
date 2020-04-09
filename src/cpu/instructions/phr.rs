use super::{Instruction, InstructionName, PushStack};
use crate::cpu::variables::Get;
use crate::state::CPU;

/// Represents the 'push register' instructions
/// (http://www.obelisk.me.uk/6502/reference.html#PHA)
/// (http://www.obelisk.me.uk/6502/reference.html#PHP)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PH<T: Get>(pub T);

impl<T: Get> Instruction for PH<T> {
    fn name(&self) -> InstructionName {
        InstructionName::PH(self.0.name())
    }
}

impl<T: Get, S: CPU> PushStack<S> for PH<T> {
    fn get(&self, cpu: &S) -> u8 {
        self.0.get(cpu)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::variables::{a_register::A, p_register::P};
    use crate::state::cpu::Registers;
    use crate::state::NES;

    #[test]
    fn test_pha() {
        let mut cpu = NES::mock();
        cpu.set_a(30);
        assert_eq!(PH(A).get(&cpu), 30);
    }

    #[test]
    fn test_php() {
        let mut cpu = NES::mock();
        cpu.set_p(0b0110_0000);
        assert_eq!(PH(P).get(&cpu), 0b0111_0000);
    }
}
