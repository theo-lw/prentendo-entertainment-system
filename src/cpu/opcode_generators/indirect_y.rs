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
) -> Pin<Box<dyn Generator<Yield = CPUCycle, Return = CPUCycle> + 'a>> {
    Box::pin(move || {
        let mut cycle = CPUCycle {
            instruction: instruction.name(),
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
) -> Pin<Box<dyn Generator<Yield = CPUCycle, Return = CPUCycle> + 'a>> {
    Box::pin(move || {
        let mut cycle = CPUCycle {
            instruction: instruction.name(),
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
) -> Pin<Box<dyn Generator<Yield = CPUCycle, Return = CPUCycle> + 'a>> {
    Box::pin(move || {
        let mut cycle = CPUCycle {
            instruction: instruction.name(),
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
    use crate::cpu::instructions::{Instruction, adc::ADC, asl::ASL, sta::STA};
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
            instruction: instruction.name(),
            mode: AddressingMode::IndirectY,
            cycle: 0,
        };
        for _ in 0..4 {
            let state = opcode.as_mut().resume(());
            assert_eq!(state, GeneratorState::Yielded(cycle));
            assert_eq!(cpu.borrow().registers.a, 120);
            cycle.next();
        }
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Complete(cycle));
        assert_eq!(cpu.borrow().registers.a, 163);
    }

    #[test]
    fn test_modify() {
        let mut cpu = CPU::mock();
        cpu.registers.y = 3;
        cpu.memory.set(cpu.registers.pc, 0x23);
        cpu.memory.set(0x23, 0x26);
        cpu.memory.set(0x24, 0x44);
        cpu.memory.set(0x4429, 0b0100_0101);
        let cpu = Rc::new(RefCell::new(cpu));
        let instruction = ASL;
        let mut opcode = modify(&cpu, instruction);
        let mut cycle = CPUCycle {
            instruction: instruction.name(),
            mode: AddressingMode::IndirectY,
            cycle: 0,
        };
        for _ in 0..7 {
            let state = opcode.as_mut().resume(());
            assert_eq!(state, GeneratorState::Yielded(cycle));
            assert_eq!(cpu.borrow().memory.get(0x4429), 0b0100_0101);
            cycle.next();
        }
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Complete(cycle));
        assert_eq!(cpu.borrow().memory.get(0x4429), 0b1000_1010);
    }

    #[test]
    fn test_write() {
        let mut cpu = CPU::mock();
        cpu.registers.a = 43;
        cpu.registers.y = 4;
        cpu.memory.set(cpu.registers.pc, 0x10);
        cpu.memory.set(0x10, 0x20);
        cpu.memory.set(0x11, 0x40);
        cpu.memory.set(0x4024, 0);
        let cpu = Rc::new(RefCell::new(cpu));
        let instruction = STA;
        let mut opcode = write(&cpu, instruction);
        let mut cycle = CPUCycle {
            instruction: instruction.name(),
            mode: AddressingMode::IndirectY,
            cycle: 0,
        };
        for _ in 0..5 {
            let state = opcode.as_mut().resume(());
            assert_eq!(state, GeneratorState::Yielded(cycle));
            assert_eq!(cpu.borrow().memory.get(0x4024), 0);
            cycle.next();
        }
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Complete(cycle));
        assert_eq!(cpu.borrow().memory.get(0x4024), 43);
    }
}
