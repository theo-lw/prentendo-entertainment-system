use crate::{
    cpu::{
        instructions::{Modify, Read, Write},
        opcode_generators::{AddressingMode, CPUCycle},
    },
    state::CPU,
};
use std::{cell::RefCell, ops::Generator, pin::Pin};

/// Creates the opcode for 'Read' instructions with zero X addressing
pub fn read<'a, T: Read<S> + 'a, S: CPU>(
    cpu: &'a RefCell<S>,
    instruction: T,
) -> Pin<Box<dyn Generator<Yield = CPUCycle, Return = CPUCycle> + 'a>> {
    Box::pin(move || {
        let mut cycle = CPUCycle {
            instruction: instruction.name(),
            mode: AddressingMode::ZeroX,
            cycle: 0,
        };
        yield cycle;
        cycle.next();
        let address: u8 = cpu.borrow_mut().get_and_increment_pc();
        yield cycle;
        cycle.next();
        let address: u8 = address.wrapping_add(cpu.borrow().get_x());
        yield cycle;
        cycle.next();
        instruction.execute(&mut cpu.borrow_mut(), u16::from_be_bytes([0, address]));
        cycle
    })
}

/// Creates the opcode for 'Write' instructions with zero X addressing
pub fn write<'a, T: Write<S> + 'a, S: CPU>(
    cpu: &'a RefCell<S>,
    instruction: T,
) -> Pin<Box<dyn Generator<Yield = CPUCycle, Return = CPUCycle> + 'a>> {
    Box::pin(move || {
        let mut cycle = CPUCycle {
            instruction: instruction.name(),
            mode: AddressingMode::ZeroX,
            cycle: 0,
        };
        yield cycle;
        cycle.next();
        let address: u8 = cpu.borrow_mut().get_and_increment_pc();
        yield cycle;
        cycle.next();
        let address: u8 = address.wrapping_add(cpu.borrow().get_x());
        yield cycle;
        cycle.next();
        instruction.execute(&mut cpu.borrow_mut(), u16::from_be_bytes([0, address]));
        cycle
    })
}

/// Creates the opcode for 'Modify' instructions with zero X addressing
pub fn modify<'a, T: Modify<S> + 'a, S: CPU>(
    cpu: &'a RefCell<S>,
    instruction: T,
) -> Pin<Box<dyn Generator<Yield = CPUCycle, Return = CPUCycle> + 'a>> {
    Box::pin(move || {
        let mut cycle = CPUCycle {
            instruction: instruction.name(),
            mode: AddressingMode::ZeroX,
            cycle: 0,
        };
        yield cycle;
        cycle.next();
        let address: u8 = cpu.borrow_mut().get_and_increment_pc();
        yield cycle;
        cycle.next();
        let address: u8 = address.wrapping_add(cpu.borrow().get_x());
        yield cycle;
        cycle.next();
        let addr = u16::from_be_bytes([0, address]);
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
    use crate::state::cpu::{Memory, Registers};
    use crate::state::NES;
    use std::ops::GeneratorState;

    #[test]
    fn test_read() {
        let mut cpu = NES::mock();
        cpu.set_x(5);
        cpu.set_pc(0);
        cpu.set_mem(cpu.get_pc(), 0x23);
        cpu.set_mem(0x28, 19);
        cpu.set_a(133);
        let cpu = RefCell::new(cpu);
        let instruction = ADC;
        let mut opcode = read(&cpu, instruction);
        let mut cycle = CPUCycle {
            instruction: instruction.name(),
            mode: AddressingMode::ZeroX,
            cycle: 0,
        };
        for _ in 0..3 {
            let state = opcode.as_mut().resume(());
            assert_eq!(state, GeneratorState::Yielded(cycle));
            assert_eq!(cpu.borrow().get_a(), 133);
            cycle.next();
        }
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Complete(cycle));
        assert_eq!(cpu.borrow().get_a(), 152);
        assert_eq!(cpu.borrow().get_pc(), 1);
    }

    #[test]
    fn test_write() {
        let mut cpu = NES::mock();
        cpu.set_a(43);
        cpu.set_x(5);
        cpu.set_pc(0);
        cpu.set_mem(cpu.get_pc(), 0x10);
        cpu.set_mem(0x15, 0);
        let cpu = RefCell::new(cpu);
        let instruction = ST(A);
        let mut opcode = write(&cpu, instruction);
        let mut cycle = CPUCycle {
            instruction: instruction.name(),
            mode: AddressingMode::ZeroX,
            cycle: 0,
        };
        for _ in 0..3 {
            let state = opcode.as_mut().resume(());
            assert_eq!(state, GeneratorState::Yielded(cycle));
            assert_eq!(cpu.borrow().get_mem(0x15), 0);
            cycle.next();
        }
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Complete(cycle));
        assert_eq!(cpu.borrow().get_mem(0x15), 43);
        assert_eq!(cpu.borrow().get_pc(), 1);
    }

    #[test]
    fn test_modify() {
        let mut cpu = NES::mock();
        cpu.set_x(1);
        cpu.set_pc(0);
        cpu.set_mem(cpu.get_pc(), 0x29);
        cpu.set_mem(0x2A, 0b0100_0101);
        let cpu = RefCell::new(cpu);
        let instruction = ASL;
        let mut opcode = modify(&cpu, instruction);
        let mut cycle = CPUCycle {
            instruction: instruction.name(),
            mode: AddressingMode::ZeroX,
            cycle: 0,
        };
        for _ in 0..5 {
            let state = opcode.as_mut().resume(());
            assert_eq!(state, GeneratorState::Yielded(cycle));
            assert_eq!(cpu.borrow().get_mem(0x2A), 0b0100_0101);
            cycle.next();
        }
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Complete(cycle));
        assert_eq!(cpu.borrow().get_mem(0x2A), 0b1000_1010);
        assert_eq!(cpu.borrow().get_pc(), 1);
    }
}
