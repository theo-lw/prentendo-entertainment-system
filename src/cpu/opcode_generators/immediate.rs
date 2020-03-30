use crate::{
    cpu::{
        instructions::{Read},
        opcode_generators::{AddressingMode, CPUCycle},
        state::CPU,
    },
};
use std::{cell::RefCell, ops::Generator, pin::Pin, rc::Rc};

/// Creates the opcode for 'Read' instructions with immediate addressing
pub fn read<'a, T: Read + 'a>(
    cpu: &'a Rc<RefCell<CPU>>,
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
        let addr: u16 = cpu.borrow().registers.pc;
        instruction.execute(cpu, addr);
        cpu.borrow_mut().registers.increment_pc();
        return cycle;
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{address::AddressMap, cpu::instructions::{Instruction, adc::ADC}};
    use std::ops::GeneratorState;

    #[test]
    fn test_read() {
        let mut cpu = CPU::mock();
        cpu.registers.pc = 0;
        cpu.memory.set(cpu.registers.pc, 3);
        cpu.registers.a = 12;
        let cpu = Rc::new(RefCell::new(cpu));
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
            assert_eq!(cpu.borrow().registers.a, 12);
            cycle.next();
        }
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Complete(cycle));
        assert_eq!(cpu.borrow().registers.a, 15);
        assert_eq!(cpu.borrow().registers.pc, 1);
    }
}
