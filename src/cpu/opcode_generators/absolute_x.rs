use crate::{
    cpu::{
        instructions::{Modify, Read, Write},
        opcode_generators::{AddressingMode, CPUCycle},
    },
    state::CPU,
};
use std::{cell::RefCell, ops::Generator, pin::Pin};

/// Creates the opcode for 'Read' instructions with absolute X addressing
pub fn read<'a, T: Read<S> + 'a, S: CPU>(
    cpu: &'a RefCell<S>,
    instruction: T,
) -> Pin<Box<dyn Generator<Yield = CPUCycle, Return = CPUCycle> + 'a>> {
    Box::pin(move || {
        let mut cycle = CPUCycle {
            instruction: instruction.name(),
            mode: AddressingMode::AbsoluteX,
            cycle: 0,
        };
        yield cycle;
        cycle.next();
        let low_byte: u8 = cpu.borrow_mut().get_and_increment_pc();
        yield cycle;
        cycle.next();
        let mut high_byte: u8 = cpu.borrow_mut().get_and_increment_pc();
        let (low_byte, overflow): (u8, bool) = low_byte.overflowing_add(cpu.borrow().get_x());
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

/// Creates the opcode for 'Write' instructions with absolute X addressing
pub fn write<'a, T: Write<S> + 'a, S: CPU>(
    cpu: &'a RefCell<S>,
    instruction: T,
) -> Pin<Box<dyn Generator<Yield = CPUCycle, Return = CPUCycle> + 'a>> {
    Box::pin(move || {
        let mut cycle = CPUCycle {
            instruction: instruction.name(),
            mode: AddressingMode::AbsoluteX,
            cycle: 0,
        };
        yield cycle;
        cycle.next();
        let low_byte: u8 = cpu.borrow_mut().get_and_increment_pc();
        yield cycle;
        cycle.next();
        let mut high_byte: u8 = cpu.borrow_mut().get_and_increment_pc();
        let (low_byte, overflow): (u8, bool) = low_byte.overflowing_add(cpu.borrow().get_x());
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

/// Creates the opcode for 'Modify' instructions with absolute X addressing
pub fn modify<'a, T: Modify<S> + 'a, S: CPU>(
    cpu: &'a RefCell<S>,
    instruction: T,
) -> Pin<Box<dyn Generator<Yield = CPUCycle, Return = CPUCycle> + 'a>> {
    Box::pin(move || {
        let mut cycle = CPUCycle {
            instruction: instruction.name(),
            mode: AddressingMode::AbsoluteX,
            cycle: 0,
        };
        yield cycle;
        cycle.next();
        let low_byte: u8 = cpu.borrow_mut().get_and_increment_pc();
        yield cycle;
        cycle.next();
        let mut high_byte: u8 = cpu.borrow_mut().get_and_increment_pc();
        let (low_byte, overflow): (u8, bool) = low_byte.overflowing_add(cpu.borrow().get_x());
        yield cycle;
        cycle.next();
        if overflow {
            high_byte = high_byte.wrapping_add(1);
        }
        yield cycle;
        cycle.next();
        let addr = u16::from_be_bytes([high_byte, low_byte]);
        let val: u8 = cpu.borrow().get_mem(addr);
        yield cycle;
        cycle.next();
        cpu.borrow_mut().set_mem(addr, val);
        yield cycle;
        cycle.next();
        instruction.execute(&mut cpu.borrow_mut(), addr, val);
        cycle
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::instructions::{adc::ADC, asl::ASL, str::ST, Instruction};
    use crate::cpu::variables::a_register::A;
    use std::ops::GeneratorState;
    use crate::state::NES;
    use crate::state::cpu::{Registers, Memory};

    #[test]
    fn test_read() {
        let mut cpu = NES::mock();
        cpu.set_x(3);
        cpu.set_pc(0);
        cpu.set_mem(cpu.get_pc(), 0x23);
        cpu.set_mem(cpu.get_pc() + 1, 0x03);
        cpu.set_mem(0x0326, 43);
        cpu.set_a(120);
        let cpu = RefCell::new(cpu);
        let instruction = ADC;
        let mut opcode = read(&cpu, instruction);
        let mut cycle = CPUCycle {
            instruction: instruction.name(),
            mode: AddressingMode::AbsoluteX,
            cycle: 0,
        };
        for _ in 0..3 {
            let state = opcode.as_mut().resume(());
            assert_eq!(state, GeneratorState::Yielded(cycle));
            assert_eq!(cpu.borrow().get_a(), 120);
            cycle.next();
        }
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Complete(cycle));
        assert_eq!(cpu.borrow().get_a(), 163);
        assert_eq!(cpu.borrow().get_pc(), 2);
    }

    #[test]
    fn test_modify() {
        let mut cpu = NES::mock();
        cpu.set_x(3);
        cpu.set_pc(0);
        cpu.set_mem(cpu.get_pc(), 0x23);
        cpu.set_mem(cpu.get_pc() + 1, 0x03);
        cpu.set_mem(0x0326, 0b0100_0101);
        let cpu = RefCell::new(cpu);
        let instruction = ASL;
        let mut opcode = modify(&cpu, instruction);
        let mut cycle = CPUCycle {
            instruction: instruction.name(),
            mode: AddressingMode::AbsoluteX,
            cycle: 0,
        };
        for _ in 0..6 {
            let state = opcode.as_mut().resume(());
            assert_eq!(state, GeneratorState::Yielded(cycle));
            assert_eq!(cpu.borrow().get_mem(0x0326), 0b0100_0101);
            cycle.next();
        }
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Complete(cycle));
        assert_eq!(cpu.borrow().get_mem(0x0326), 0b1000_1010);
        assert_eq!(cpu.borrow().get_pc(), 2);
    }

    #[test]
    fn test_write() {
        let mut cpu = NES::mock();
        cpu.set_a(43);
        cpu.set_x(4);
        cpu.set_pc(0);
        cpu.set_mem(cpu.get_pc(), 0x23);
        cpu.set_mem(cpu.get_pc() + 1, 0x03);
        cpu.set_mem(0x0327, 0);
        let cpu = RefCell::new(cpu);
        let instruction = ST(A);
        let mut opcode = write(&cpu, instruction);
        let mut cycle = CPUCycle {
            instruction: instruction.name(),
            mode: AddressingMode::AbsoluteX,
            cycle: 0,
        };
        for _ in 0..4 {
            let state = opcode.as_mut().resume(());
            assert_eq!(state, GeneratorState::Yielded(cycle));
            assert_eq!(cpu.borrow().get_mem(0x0327), 0);
            cycle.next();
        }
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Complete(cycle));
        assert_eq!(cpu.borrow().get_mem(0x0327), 43);
        assert_eq!(cpu.borrow().get_pc(), 2);
    }
}
