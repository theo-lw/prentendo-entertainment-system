use super::{Branch, Instruction, InstructionName};
use crate::cpu::variables::Flag;
use crate::state::CPU;

/// Represents the 'branch if set' instructions
/// (http://www.obelisk.me.uk/6502/reference.html#BCS)
/// (http://www.obelisk.me.uk/6502/reference.html#BEQ)
/// (http://www.obelisk.me.uk/6502/reference.html#BMI)
/// (http://www.obelisk.me.uk/6502/reference.html#BVS)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BS(pub Flag);

impl Instruction for BS {
    fn name(&self) -> InstructionName {
        InstructionName::BS(self.0)
    }
}

impl<S: CPU> Branch<S> for BS {
    fn should_branch(&self, cpu: &S) -> bool {
        cpu.is_flag_set(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::cpu::Registers;
    use crate::state::NES;

    #[test]
    fn test_bs() {
        let mut cpu = NES::mock();
        cpu.assign_flag(Flag::C, false);
        assert_eq!(BS(Flag::C).should_branch(&mut cpu), false);
        cpu.assign_flag(Flag::C, true);
        assert_eq!(BS(Flag::C).should_branch(&mut cpu), true);
    }
}
