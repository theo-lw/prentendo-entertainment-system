use crate::{
    address::AddressMap,
    cpu::{
        instructions::{Instruction, Modify, Read, Write},
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
            mode: AddressingMode::Absolute,
            cycle: 0,
        };
        yield cycle;
        cycle.next();
        let low_byte: u8 = cpu.borrow_mut().get_and_increment_pc();
        yield cycle;
        cycle.next();
        let high_byte: u8 = cpu.borrow_mut().get_and_increment_pc();
        yield cycle;
        cycle.next();
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
            mode: AddressingMode::Absolute,
            cycle: 0,
        };
        yield cycle;
        cycle.next();
        let low_byte: u8 = cpu.borrow_mut().get_and_increment_pc();
        yield cycle;
        cycle.next();
        let high_byte: u8 = cpu.borrow_mut().get_and_increment_pc();
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
            mode: AddressingMode::Absolute,
            cycle: 0,
        };
        yield cycle;
        cycle.next();
        let low_byte: u8 = cpu.borrow_mut().get_and_increment_pc();
        yield cycle;
        cycle.next();
        let high_byte: u8 = cpu.borrow_mut().get_and_increment_pc();
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
    use crate::cpu::instructions::{adc::ADC, asl::ASL};
    use std::ops::GeneratorState;

    #[test]
    fn test_read() {
        let mut cpu = CPU::mock();
        cpu.registers.x = 3;
        cpu.memory.set(cpu.registers.pc, 0x23);
        cpu.memory.set(cpu.registers.pc + 1, 0x44);
        cpu.memory.set(0x4423, 3);
        cpu.registers.a = 12;
        let cpu = Rc::new(RefCell::new(cpu));
        let instruction = ADC;
        let mut opcode = read(&cpu, instruction);
        let mut cycle = CPUCycle {
            instruction: instruction.name(),
            mode: AddressingMode::Absolute,
            cycle: 0,
        };
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().registers.a, 12);
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().registers.a, 12);
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().registers.a, 12);
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Complete(cycle));
        assert_eq!(cpu.borrow().registers.a, 15);
    }

    #[test]
    fn test_modify() {
        let mut cpu = CPU::mock();
        cpu.registers.x = 3;
        cpu.memory.set(cpu.registers.pc, 0x23);
        cpu.memory.set(cpu.registers.pc + 1, 0x44);
        cpu.memory.set(0x4423, 0b0100_0101);
        let cpu = Rc::new(RefCell::new(cpu));
        let instruction = ASL;
        let mut opcode = modify(&cpu, instruction);
        let mut cycle = CPUCycle {
            instruction: instruction.name(),
            mode: AddressingMode::Absolute,
            cycle: 0,
        };
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().memory.get(0x4423), 0b0100_0101);
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().memory.get(0x4423), 0b0100_0101);
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().memory.get(0x4423), 0b0100_0101);
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().memory.get(0x4423), 0b0100_0101);
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().memory.get(0x4423), 0b0100_0101);
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Complete(cycle));
        assert_eq!(cpu.borrow().memory.get(0x4423), 0b1000_1010);
    }
}
