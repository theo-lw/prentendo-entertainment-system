use super::{Implied, Instruction, InstructionName};
use crate::cpu::variables::Flag;
use crate::state::CPU;

/// Represents the 'clear' instructions
/// (http://www.obelisk.me.uk/6502/reference.html#CLC)
/// (http://www.obelisk.me.uk/6502/reference.html#CLD)
/// (http://www.obelisk.me.uk/6502/reference.html#CLI)
/// (http://www.obelisk.me.uk/6502/reference.html#CLV)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CL(pub Flag);

impl Instruction for CL {
    fn name(&self) -> InstructionName {
        InstructionName::CL(self.0)
    }
}

impl<S: CPU> Implied<S> for CL {
    fn execute(&self, cpu: &mut S) {
        cpu.clear_flag(self.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::cpu::Registers;
    use crate::state::NES;

    #[test]
    fn test_clc() {
        let mut cpu = NES::mock();
        cpu.clear_flag(Flag::C);
        CL(Flag::C).execute(&mut cpu);
        assert_eq!(cpu.is_flag_set(Flag::C), false);
        cpu.set_flag(Flag::Z);
        CL(Flag::Z).execute(&mut cpu);
        assert_eq!(cpu.is_flag_set(Flag::C), false);
    }
}
