use super::{Implied, Instruction, InstructionName};
use crate::cpu::variables::Flag;
use crate::state::CPU;

/// Represents the 'set flag' instructions
/// (http://www.obelisk.me.uk/6502/reference.html#SEC)
/// (http://www.obelisk.me.uk/6502/reference.html#SED)
/// (http://www.obelisk.me.uk/6502/reference.html#SEI)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SE(pub Flag);

impl Instruction for SE {
    fn name(&self) -> InstructionName {
        InstructionName::SE(self.0)
    }
}

impl<S: CPU> Implied<S> for SE {
    fn execute(&self, cpu: &mut S) {
        cpu.assign_flag(self.0, true);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::cpu::Registers;
    use crate::state::NES;

    #[test]
    fn test_sec() {
        let mut cpu = NES::mock();
        cpu.assign_flag(Flag::C, false);
        SE(Flag::C).execute(&mut cpu);
        assert_eq!(cpu.is_flag_set(Flag::C), true);
        SE(Flag::Z).execute(&mut cpu);
        assert_eq!(cpu.is_flag_set(Flag::C), true);
    }
}
