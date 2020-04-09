use super::{Implied, Instruction, InstructionName};
use crate::state::CPU;

/// Represents the NOP instruction (http://www.obelisk.me.uk/6502/reference.html#NOP)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct NOP;

impl Instruction for NOP {
    fn name(&self) -> InstructionName {
        InstructionName::NOP
    }
}

impl<S: CPU> Implied<S> for NOP {
    fn execute(&self, _: &mut S) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::cpu::Registers;
    use crate::state::NES;

    #[test]
    fn test_nop() {
        let mut cpu = NES::mock();
        cpu.set_x(100);
        cpu.set_y(100);
        cpu.set_a(100);
        NOP.execute(&mut cpu);
        assert_eq!(cpu.get_x(), 100);
        assert_eq!(cpu.get_y(), 100);
        assert_eq!(cpu.get_a(), 100);
    }
}
