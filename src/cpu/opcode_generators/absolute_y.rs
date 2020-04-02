use crate::cpu::{
    instructions::{Read, Write},
    opcode_generators::{AddressingMode, CPUCycle},
    state::CPU,
};
use std::{cell::RefCell, ops::Generator, pin::Pin, rc::Rc};

/// Creates the opcode for 'Read' instructions with absolute Y addressing
pub fn read<'a, T: Read + 'a>(
    cpu: &'a Rc<RefCell<CPU>>,
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
        let (low_byte, overflow): (u8, bool) = low_byte.overflowing_add(cpu.borrow().registers.y);
        yield cycle;
        cycle.next();
        if overflow {
            high_byte = high_byte.wrapping_add(1);
            yield cycle;
            cycle.next();
        }
        instruction.execute(cpu, u16::from_be_bytes([high_byte, low_byte]));
        cycle
    })
}

/// Creates the opcode for 'Write' instructions with absolute Y addressing
pub fn write<'a, T: Write + 'a>(
    cpu: &'a Rc<RefCell<CPU>>,
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
        let (low_byte, overflow): (u8, bool) = low_byte.overflowing_add(cpu.borrow().registers.y);
        yield cycle;
        cycle.next();
        if overflow {
            high_byte = high_byte.wrapping_add(1);
        }
        yield cycle;
        cycle.next();
        instruction.execute(cpu, u16::from_be_bytes([high_byte, low_byte]));
        cycle
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::address::AddressMap;
    use crate::cpu::instructions::{adc::ADC, str::ST, Instruction};
    use crate::cpu::variables::a_register::A;
    use std::ops::GeneratorState;

    #[test]
    fn test_read() {
        let mut cpu = CPU::mock();
        cpu.registers.y = 5;
        cpu.memory.set(cpu.registers.pc, 0x20);
        cpu.memory.set(cpu.registers.pc + 1, 0x31);
        cpu.memory.set(0x3125, 61);
        cpu.registers.a = 38;
        cpu.registers.pc = 0;
        let cpu = Rc::new(RefCell::new(cpu));
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
            assert_eq!(cpu.borrow().registers.a, 38);
            cycle.next();
        }
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Complete(cycle));
        assert_eq!(cpu.borrow().registers.a, 99);
        assert_eq!(cpu.borrow().registers.pc, 2);
    }

    #[test]
    fn test_write() {
        let mut cpu = CPU::mock();
        cpu.registers.a = 43;
        cpu.registers.y = 4;
        cpu.registers.pc = 0;
        cpu.memory.set(cpu.registers.pc, 0x23);
        cpu.memory.set(cpu.registers.pc + 1, 0x44);
        cpu.memory.set(0x4427, 0);
        let cpu = Rc::new(RefCell::new(cpu));
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
            assert_eq!(cpu.borrow().memory.get(0x4427), 0);
            cycle.next();
        }
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Complete(cycle));
        assert_eq!(cpu.borrow().memory.get(0x4427), 43);
        assert_eq!(cpu.borrow().registers.pc, 2);
    }
}
