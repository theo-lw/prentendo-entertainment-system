use crate::cpu::{
    instructions::{Read, Write},
    opcode_generators::{AddressingMode, CPUCycle},
};
use crate::state::CPU;
use std::{cell::RefCell, ops::Generator, pin::Pin};

/// Creates the opcode for 'Read' instructions with absolute Y addressing
pub fn read<'a, T: Read<S> + 'a, S: CPU>(
    cpu: &'a RefCell<S>,
    instruction: T,
) -> Pin<Box<dyn Generator<Yield = CPUCycle, Return = CPUCycle> + 'a>> {
    Box::pin(move || {
        let mut cycle = CPUCycle {
            instruction: instruction.name(),
            mode: AddressingMode::AbsoluteY,
            cycle: 0,
        };
        yield cycle;
        cycle.next();
        let low_byte: u8 = cpu.borrow_mut().get_and_increment_pc();
        yield cycle;
        cycle.next();
        let mut high_byte: u8 = cpu.borrow_mut().get_and_increment_pc();
        let (low_byte, overflow): (u8, bool) = low_byte.overflowing_add(cpu.borrow().get_y());
        yield cycle;
        cycle.next();
        if overflow {
            high_byte = high_byte.wrapping_add(1);
            yield cycle;
            cycle.next();
        }
        instruction.execute(
            &mut cpu.borrow_mut(),
            u16::from_be_bytes([high_byte, low_byte]),
        );
        cycle
    })
}

/// Creates the opcode for 'Write' instructions with absolute Y addressing
pub fn write<'a, T: Write<S> + 'a, S: CPU>(
    cpu: &'a RefCell<S>,
    instruction: T,
) -> Pin<Box<dyn Generator<Yield = CPUCycle, Return = CPUCycle> + 'a>> {
    Box::pin(move || {
        let mut cycle = CPUCycle {
            instruction: instruction.name(),
            mode: AddressingMode::AbsoluteY,
            cycle: 0,
        };
        yield cycle;
        cycle.next();
        let low_byte: u8 = cpu.borrow_mut().get_and_increment_pc();
        yield cycle;
        cycle.next();
        let mut high_byte: u8 = cpu.borrow_mut().get_and_increment_pc();
        let (low_byte, overflow): (u8, bool) = low_byte.overflowing_add(cpu.borrow().get_y());
        yield cycle;
        cycle.next();
        if overflow {
            high_byte = high_byte.wrapping_add(1);
        }
        yield cycle;
        cycle.next();
        instruction.execute(
            &mut cpu.borrow_mut(),
            u16::from_be_bytes([high_byte, low_byte]),
        );
        cycle
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::instructions::{adc::ADC, str::ST, Instruction};
    use crate::cpu::variables::a_register::A;
    use std::ops::GeneratorState;
    use crate::state::NES;
    use crate::state::cpu::{Registers, Memory};

    #[test]
    fn test_read() {
        let mut cpu = NES::mock();
        cpu.set_y(5);
        cpu.set_mem(cpu.get_pc(), 0x20);
        cpu.set_mem(cpu.get_pc() + 1, 0x31);
        cpu.set_mem(0x3125, 61);
        cpu.set_a(38);
        cpu.set_pc(0);
        let cpu = RefCell::new(cpu);
        let instruction = ADC;
        let mut opcode = read(&cpu, instruction);
        let mut cycle = CPUCycle {
            instruction: instruction.name(),
            mode: AddressingMode::AbsoluteY,
            cycle: 0,
        };
        for _ in 0..3 {
            let state = opcode.as_mut().resume(());
            assert_eq!(state, GeneratorState::Yielded(cycle));
            assert_eq!(cpu.borrow().get_a(), 38);
            cycle.next();
        }
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Complete(cycle));
        assert_eq!(cpu.borrow().get_a(), 99);
        assert_eq!(cpu.borrow().get_pc(), 2);
    }

    #[test]
    fn test_write() {
        let mut cpu = NES::mock();
        cpu.set_a(43);
        cpu.set_y(4);
        cpu.set_pc(0);
        cpu.set_mem(cpu.get_pc(), 0x23);
        cpu.set_mem(cpu.get_pc() + 1, 0x05);
        cpu.set_mem(0x0527, 0);
        let cpu = RefCell::new(cpu);
        let instruction = ST(A);
        let mut opcode = write(&cpu, instruction);
        let mut cycle = CPUCycle {
            instruction: instruction.name(),
            mode: AddressingMode::AbsoluteY,
            cycle: 0,
        };
        for _ in 0..4 {
            let state = opcode.as_mut().resume(());
            assert_eq!(state, GeneratorState::Yielded(cycle));
            assert_eq!(cpu.borrow().get_mem(0x0527), 0);
            cycle.next();
        }
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Complete(cycle));
        assert_eq!(cpu.borrow().get_mem(0x0527), 43);
        assert_eq!(cpu.borrow().get_pc(), 2);
    }
}
