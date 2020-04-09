use crate::cpu::{
    instructions::Read,
    opcode_generators::{AddressingMode, CPUCycle},
};
use crate::state::CPU;
use std::{cell::RefCell, ops::Generator, pin::Pin};

/// Creates the opcode for 'Read' instructions with immediate addressing
pub fn read<'a, T: Read<S> + 'a, S: CPU>(
    cpu: &'a RefCell<S>,
    instruction: T,
) -> Pin<Box<dyn Generator<Yield = CPUCycle, Return = CPUCycle> + 'a>> {
    Box::pin(move || {
        let mut cycle = CPUCycle {
            instruction: instruction.name(),
            mode: AddressingMode::Immediate,
            cycle: 0,
        };
        yield cycle;
        cycle.next();
        let addr: u16 = cpu.borrow().get_pc();
        instruction.execute(&mut cpu.borrow_mut(), addr);
        cpu.borrow_mut().increment_pc();
        cycle
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::instructions::{adc::ADC, Instruction};
    use crate::state::cpu::{Memory, Registers};
    use crate::state::NES;
    use std::ops::GeneratorState;

    #[test]
    fn test_read() {
        let mut cpu = NES::mock();
        cpu.set_pc(0);
        cpu.set_mem(cpu.get_pc(), 3);
        cpu.set_a(12);
        let cpu = RefCell::new(cpu);
        let instruction = ADC;
        let mut opcode = read(&cpu, instruction);
        let mut cycle = CPUCycle {
            instruction: instruction.name(),
            mode: AddressingMode::Immediate,
            cycle: 0,
        };
        for _ in 0..1 {
            let state = opcode.as_mut().resume(());
            assert_eq!(state, GeneratorState::Yielded(cycle));
            assert_eq!(cpu.borrow().get_a(), 12);
            cycle.next();
        }
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Complete(cycle));
        assert_eq!(cpu.borrow().get_a(), 15);
        assert_eq!(cpu.borrow().get_pc(), 1);
    }
}
