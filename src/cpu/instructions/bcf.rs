use super::{Branch, Instruction, InstructionName};
use crate::cpu::variables::Flag;
use crate::state::CPU;

/// Represents the 'branch if clear' instructions
/// (http://www.obelisk.me.uk/6502/reference.html#BCC)
/// (http://www.obelisk.me.uk/6502/reference.html#BNE)
/// (http://www.obelisk.me.uk/6502/reference.html#BPL)
/// (http://www.obelisk.me.uk/6502/reference.html#BVC)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BC(pub Flag);

impl Instruction for BC {
    fn name(&self) -> InstructionName {
        InstructionName::BC(self.0)
    }
}

impl<S: CPU> Branch<S> for BC {
    fn should_branch(&self, cpu: &S) -> bool {
        !cpu.is_flag_set(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::cpu::Registers;
    use crate::state::NES;

    #[test]
    fn test_bc() {
        let mut cpu = NES::mock();
        cpu.assign_flag(Flag::Z, false);
        assert_eq!(BC(Flag::Z).should_branch(&mut cpu), true);
        cpu.assign_flag(Flag::Z, true);
        assert_eq!(BC(Flag::Z).should_branch(&mut cpu), false);
    }
}
