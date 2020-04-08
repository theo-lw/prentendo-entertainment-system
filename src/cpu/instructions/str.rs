use super::{Instruction, InstructionName, Write};
use crate::state::CPU;
use crate::cpu::variables::Get;

/// Represents the 'store' instructions
/// (http://www.obelisk.me.uk/6502/reference.html#STA)
/// (http://www.obelisk.me.uk/6502/reference.html#STX)
/// (http://www.obelisk.me.uk/6502/reference.html#STY)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ST<T: Get>(pub T);

impl<T: Get> Instruction for ST<T> {
    fn name(&self) -> InstructionName {
        InstructionName::ST(self.0.name())
    }
}

impl<T: Get, S: CPU> Write<S> for ST<T> {
    fn execute(&self, cpu: &mut S, addr: u16) {
        let register: u8 = self.0.get(cpu);
        cpu.set_mem(addr, register)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::variables::a_register::A;
    use crate::state::NES;
    use crate::state::cpu::{Registers, Memory};

    #[test]
    fn test_st() {
        let mut cpu = NES::mock();
        cpu.set_a(23);
        ST(A).execute(&mut cpu, 0x99);
        assert_eq!(cpu.get_mem(0x99), 23);
    }
}
