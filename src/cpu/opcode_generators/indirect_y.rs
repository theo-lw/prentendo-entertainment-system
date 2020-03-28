use crate::{
    address::AddressMap,
    cpu::{
        instructions::{Modify, Read, Write},
        opcode_generators::{AddressingMode, CPUCycle},
        state::CPU,
    },
};
use std::{cell::RefCell, ops::Generator, pin::Pin, rc::Rc};

pub fn read<'a, T: Read + 'a>(
    cpu: &'a Rc<RefCell<CPU>>,
    instruction: T,
) -> Pin<Box<dyn Generator<Yield = CPUCycle<T>, Return = CPUCycle<T>> + 'a>> {
    Box::pin(move || {
        let mut cycle = CPUCycle {
            instruction,
            mode: AddressingMode::IndirectY,
            cycle: 0,
        };
        yield cycle;
        cycle.next();
        let pointer: u16 = u16::from_be_bytes([0, cpu.borrow_mut().get_and_increment_pc()]);
        yield cycle;
        cycle.next();
        let low_byte: u8 = cpu.borrow().memory.get(pointer);
        yield cycle;
        cycle.next();
        let mut high_byte: u8 = cpu.borrow().memory.get(pointer.wrapping_add(1));
        let (low_byte, overflow): (u8, bool) = low_byte.overflowing_add(cpu.borrow().registers.y);
        yield cycle;
        cycle.next();
        if overflow {
            high_byte = high_byte.wrapping_add(1);
            yield cycle;
            cycle.next();
        }
        instruction.execute(cpu, u16::from_be_bytes([high_byte, low_byte]));
        return cycle;
    })
}

pub fn write<'a, T: Write + 'a>(
    cpu: &'a Rc<RefCell<CPU>>,
    instruction: T,
) -> Pin<Box<dyn Generator<Yield = CPUCycle<T>, Return = CPUCycle<T>> + 'a>> {
    Box::pin(move || {
        let mut cycle = CPUCycle {
            instruction,
            mode: AddressingMode::IndirectY,
            cycle: 0,
        };
        yield cycle;
        cycle.next();
        let pointer: u16 = u16::from_be_bytes([0, cpu.borrow_mut().get_and_increment_pc()]);
        yield cycle;
        cycle.next();
        let low_byte: u8 = cpu.borrow().memory.get(pointer);
        yield cycle;
        cycle.next();
        let mut high_byte: u8 = cpu.borrow().memory.get(pointer.wrapping_add(1));
        let (low_byte, overflow) = low_byte.overflowing_add(cpu.borrow().registers.y);
        yield cycle;
        cycle.next();
        if overflow {
            high_byte = high_byte.wrapping_add(1);
        }
        yield cycle;
        cycle.next();
        instruction.execute(cpu, u16::from_be_bytes([high_byte, low_byte]));
        return cycle;
    })
}

pub fn modify<'a, T: Modify + 'a>(
    cpu: &'a Rc<RefCell<CPU>>,
    instruction: T,
) -> Pin<Box<dyn Generator<Yield = CPUCycle<T>, Return = CPUCycle<T>> + 'a>> {
    Box::pin(move || {
        let mut cycle = CPUCycle {
            instruction,
            mode: AddressingMode::IndirectY,
            cycle: 0,
        };
        yield cycle;
        cycle.next();
        let pointer: u16 = u16::from_be_bytes([0, cpu.borrow_mut().get_and_increment_pc()]);
        yield cycle;
        cycle.next();
        let low_byte: u8 = cpu.borrow().memory.get(pointer);
        yield cycle;
        cycle.next();
        let mut high_byte: u8 = cpu.borrow().memory.get(pointer.wrapping_add(1));
        let (low_byte, overflow): (u8, bool) = low_byte.overflowing_add(cpu.borrow().registers.y);
        yield cycle;
        cycle.next();
        if overflow {
            high_byte = high_byte.wrapping_add(1);
        }
        yield cycle;
        cycle.next();
        let addr = u16::from_be_bytes([high_byte, low_byte]);
        let val: u8 = cpu.borrow().memory.get(addr);
        yield cycle;
        cycle.next();
        cpu.borrow_mut().memory.set(addr, val);
        yield cycle;
        cycle.next();
        instruction.execute(cpu, addr, val);
        return cycle;
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::instructions::adc::ADC;
    use std::ops::GeneratorState;

    #[test]
    fn test_read() {
        let mut cpu = CPU::mock();
        cpu.registers.y = 2;
        cpu.memory.set(cpu.registers.pc, 0x20);
        cpu.memory.set(0x20, 0x44);
        cpu.memory.set(0x21, 0x11);
        cpu.memory.set(0x1146, 43);
        cpu.registers.a = 120;
        let cpu = Rc::new(RefCell::new(cpu));
        let instruction = ADC;
        let mut opcode = read(&cpu, instruction);
        let mut cycle = CPUCycle {
            instruction,
            mode: AddressingMode::IndirectY,
            cycle: 0,
        };
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().registers.a, 120);
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().registers.a, 120);
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().registers.a, 120);
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().registers.a, 120);
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Complete(cycle));
        assert_eq!(cpu.borrow().registers.a, 163);
    }
}
