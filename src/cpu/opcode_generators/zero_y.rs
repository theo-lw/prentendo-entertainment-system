use crate::cpu::{
    instructions::{Read, Write},
    opcode_generators::{AddressingMode, CPUCycle},
    state::CPU,
};
use std::{cell::RefCell, ops::Generator, pin::Pin, rc::Rc};

/// Creates the opcode for 'Read' instructions with zero Y addressing
pub fn read<'a, T: Read + 'a>(
    cpu: &'a Rc<RefCell<CPU>>,
    instruction: T,
) -> Pin<Box<dyn Generator<Yield = CPUCycle, Return = CPUCycle> + 'a>> {
    Box::pin(move || {
        let mut cycle = CPUCycle {
            instruction: instruction.name(),
            mode: AddressingMode::ZeroY,
            cycle: 0,
        };
        yield cycle;
        cycle.next();
        let address: u8 = cpu.borrow_mut().get_and_increment_pc();
        yield cycle;
        cycle.next();
        let address: u8 = address.wrapping_add(cpu.borrow().registers.y);
        yield cycle;
        cycle.next();
        instruction.execute(cpu, u16::from_be_bytes([0, address]));
        cycle
    })
}

/// Creates the opcode for 'Write' instructions with zero Y addressing
pub fn write<'a, T: Write + 'a>(
    cpu: &'a Rc<RefCell<CPU>>,
    instruction: T,
) -> Pin<Box<dyn Generator<Yield = CPUCycle, Return = CPUCycle> + 'a>> {
    Box::pin(move || {
        let mut cycle = CPUCycle {
            instruction: instruction.name(),
            mode: AddressingMode::ZeroY,
            cycle: 0,
        };
        yield cycle;
        cycle.next();
        let address: u8 = cpu.borrow_mut().get_and_increment_pc();
        yield cycle;
        cycle.next();
        let address: u8 = address.wrapping_add(cpu.borrow().registers.y);
        yield cycle;
        cycle.next();
        instruction.execute(cpu, u16::from_be_bytes([0, address]));
        cycle
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::variables::a_register::A;
    use crate::{
        address::AddressMap,
        cpu::instructions::{adc::ADC, str::ST, Instruction},
    };
    use std::ops::GeneratorState;

    #[test]
    fn test_read() {
        let mut cpu = CPU::mock();
        cpu.registers.y = 34;
        cpu.registers.pc = 0;
        cpu.memory.set(cpu.registers.pc, 0x23);
        cpu.memory.set(0x23 + 34, 19);
        cpu.registers.a = 133;
        let cpu = Rc::new(RefCell::new(cpu));
        let instruction = ADC;
        let mut opcode = read(&cpu, instruction);
        let mut cycle = CPUCycle {
            instruction: instruction.name(),
            mode: AddressingMode::ZeroY,
            cycle: 0,
        };
        for _ in 0..3 {
            let state = opcode.as_mut().resume(());
            assert_eq!(state, GeneratorState::Yielded(cycle));
            assert_eq!(cpu.borrow().registers.a, 133);
            cycle.next();
        }
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Complete(cycle));
        assert_eq!(cpu.borrow().registers.a, 152);
        assert_eq!(cpu.borrow().registers.pc, 1);
    }

    #[test]
    fn test_write() {
        let mut cpu = CPU::mock();
        cpu.registers.pc = 0;
        cpu.registers.a = 43;
        cpu.registers.y = 5;
        cpu.memory.set(cpu.registers.pc, 0x10);
        cpu.memory.set(0x15, 0);
        let cpu = Rc::new(RefCell::new(cpu));
        let instruction = ST(A);
        let mut opcode = write(&cpu, instruction);
        let mut cycle = CPUCycle {
            instruction: instruction.name(),
            mode: AddressingMode::ZeroY,
            cycle: 0,
        };
        for _ in 0..3 {
            let state = opcode.as_mut().resume(());
            assert_eq!(state, GeneratorState::Yielded(cycle));
            assert_eq!(cpu.borrow().memory.get(0x15), 0);
            cycle.next();
        }
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Complete(cycle));
        assert_eq!(cpu.borrow().memory.get(0x15), 43);
        assert_eq!(cpu.borrow().registers.pc, 1);
    }
}
