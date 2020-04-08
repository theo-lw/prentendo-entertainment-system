use crate::{
    cpu::{
        instructions::{InstructionName, Modify, Read, Write},
        opcode_generators::{AddressingMode, CPUCycle},
    },
    state::CPU,
};
use std::{cell::RefCell, ops::Generator, pin::Pin};

/// Creates the opcode for 'Read' instructions with absolute addressing
pub fn read<'a, T: Read<S> + 'a, S: CPU>(
    cpu: &'a RefCell<S>,
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
        instruction.execute(
            &mut cpu.borrow_mut(),
            u16::from_be_bytes([high_byte, low_byte]),
        );
        cycle
    })
}

/// Creates the opcode for 'Write' instructions with absolute addressing
pub fn write<'a, T: Write<S> + 'a, S: CPU>(
    cpu: &'a RefCell<S>,
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
        instruction.execute(
            &mut cpu.borrow_mut(),
            u16::from_be_bytes([high_byte, low_byte]),
        );
        cycle
    })
}

/// Creates the opcode for 'Modify' instructions with absolute addressing
pub fn modify<'a, T: Modify<S> + 'a, S: CPU>(
    cpu: &'a RefCell<S>,
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

/// Creates the JSR opcode with absolute addressing
pub fn jsr<'a, S: CPU>(
    cpu: &'a RefCell<S>,
) -> Pin<Box<dyn Generator<Yield = CPUCycle, Return = CPUCycle> + 'a>> {
    Box::pin(move || {
        let mut cycle = CPUCycle {
            instruction: InstructionName::JSR,
            mode: AddressingMode::Absolute,
            cycle: 0,
        };
        yield cycle;
        cycle.next();
        let low_byte: u8 = cpu.borrow_mut().get_and_increment_pc();
        yield cycle;
        cycle.next();
        // undocumented internal operation
        yield cycle;
        cycle.next();
        let pc_high: u8 = cpu.borrow().get_pch();
        cpu.borrow_mut().push_stack(pc_high);
        yield cycle;
        cycle.next();
        let pc_low: u8 = cpu.borrow().get_pcl();
        cpu.borrow_mut().push_stack(pc_low);
        yield cycle;
        cycle.next();
        let high_byte: u8 = cpu.borrow_mut().get_and_increment_pc();
        cpu.borrow_mut().set_pc(u16::from_be_bytes([high_byte, low_byte]));
        cycle
    })
}

/// Creates the JMP opcode with absolute addressing
pub fn jmp<'a, S: CPU>(
    cpu: &'a RefCell<S>,
) -> Pin<Box<dyn Generator<Yield = CPUCycle, Return = CPUCycle> + 'a>> {
    Box::pin(move || {
        let mut cycle = CPUCycle {
            instruction: InstructionName::JMP,
            mode: AddressingMode::Absolute,
            cycle: 0,
        };
        yield cycle;
        cycle.next();
        let low_byte: u8 = cpu.borrow_mut().get_and_increment_pc();
        yield cycle;
        cycle.next();
        let high_byte: u8 = cpu.borrow_mut().get_and_increment_pc();
        cpu.borrow_mut().set_pc(u16::from_be_bytes([high_byte, low_byte]));
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
        cpu.set_mem(cpu.get_pc() + 1, 0x00);
        cpu.set_mem(0x0023, 3);
        cpu.set_a(12);
        let cpu = RefCell::new(cpu);
        let instruction = ADC;
        let mut opcode = read(&cpu, instruction);
        let mut cycle = CPUCycle {
            instruction: instruction.name(),
            mode: AddressingMode::Absolute,
            cycle: 0,
        };
        for _ in 0..3 {
            let state = opcode.as_mut().resume(());
            assert_eq!(state, GeneratorState::Yielded(cycle));
            assert_eq!(cpu.borrow().get_a(), 12);
            cycle.next();
        }
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Complete(cycle));
        assert_eq!(cpu.borrow().get_a(), 15);
        assert_eq!(cpu.borrow().get_pc(), 2);
    }

    #[test]
    fn test_modify() {
        let mut cpu = NES::mock();
        cpu.set_x(3);
        cpu.set_pc(0);
        cpu.set_mem(cpu.get_pc(), 0x23);
        cpu.set_mem(cpu.get_pc() + 1, 0x01);
        cpu.set_mem(0x0123, 0b0100_0101);
        let cpu = RefCell::new(cpu);
        let instruction = ASL;
        let mut opcode = modify(&cpu, instruction);
        let mut cycle = CPUCycle {
            instruction: instruction.name(),
            mode: AddressingMode::Absolute,
            cycle: 0,
        };
        for _ in 0..5 {
            let state = opcode.as_mut().resume(());
            assert_eq!(state, GeneratorState::Yielded(cycle));
            assert_eq!(cpu.borrow().get_mem(0x0123), 0b0100_0101);
            cycle.next();
        }
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Complete(cycle));
        assert_eq!(cpu.borrow().get_mem(0x0123), 0b1000_1010);
        assert_eq!(cpu.borrow().get_pc(), 2);
    }

    #[test]
    fn test_write() {
        let mut cpu = NES::mock();
        cpu.set_a(30);
        cpu.set_pc(0);
        cpu.set_mem(cpu.get_pc(), 0x23);
        cpu.set_mem(cpu.get_pc() + 1, 0x01);
        cpu.set_mem(0x0123, 0);
        let cpu = RefCell::new(cpu);
        let instruction = ST(A);
        let mut opcode = write(&cpu, instruction);
        let mut cycle = CPUCycle {
            instruction: instruction.name(),
            mode: AddressingMode::Absolute,
            cycle: 0,
        };
        for _ in 0..3 {
            let state = opcode.as_mut().resume(());
            assert_eq!(state, GeneratorState::Yielded(cycle));
            assert_eq!(cpu.borrow().get_mem(0x0123), 0);
            cycle.next();
        }
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Complete(cycle));
        assert_eq!(cpu.borrow().get_mem(0x0123), 30);
        assert_eq!(cpu.borrow().get_pc(), 2);
    }

    #[test]
    fn test_jsr() {
        let mut cpu = NES::mock();
        let pc_low: u8 = 0x60;
        let pc_high: u8 = 0x11;
        let new_pc_low: u8 = 0x34;
        let new_pc_high: u8 = 0x41;
        let initial_sp: u8 = 0xFF;
        cpu.set_s(initial_sp);
        cpu.set_pc(u16::from_be_bytes([pc_high, pc_low]));
        cpu.set_mem(cpu.get_pc(), new_pc_low);
        cpu.set_mem(cpu.get_pc() + 1, new_pc_high);
        let cpu = RefCell::new(cpu);
        let mut opcode = jsr(&cpu);
        let mut cycle = CPUCycle {
            instruction: InstructionName::JSR,
            mode: AddressingMode::Absolute,
            cycle: 0,
        };
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(
            cpu.borrow().get_pc(),
            u16::from_be_bytes([pc_high, pc_low])
        );
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(
            cpu.borrow().get_pc(),
            u16::from_be_bytes([pc_high, pc_low]) + 1
        );
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(
            cpu.borrow().get_pc(),
            u16::from_be_bytes([pc_high, pc_low]) + 1
        );
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().get_s(), initial_sp - 1);
        assert_eq!(cpu.borrow().get_mem(0x01FF), pc_high);
        assert_eq!(
            cpu.borrow().get_pc(),
            u16::from_be_bytes([pc_high, pc_low]) + 1
        );
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().get_s(), initial_sp - 2);
        assert_eq!(cpu.borrow().get_mem(0x01FF), pc_high);
        assert_eq!(cpu.borrow().get_mem(0x01FE), pc_low + 1);
        assert_eq!(
            cpu.borrow().get_pc(),
            u16::from_be_bytes([pc_high, pc_low]) + 1
        );
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Complete(cycle));
        assert_eq!(cpu.borrow().get_s(), initial_sp - 2);
        assert_eq!(cpu.borrow().get_mem(0x01FF), pc_high);
        assert_eq!(cpu.borrow().get_mem(0x01FE), pc_low + 1);
        assert_eq!(
            cpu.borrow().get_pc(),
            u16::from_be_bytes([new_pc_high, new_pc_low])
        );
    }

    #[test]
    fn test_jmp() {
        let mut cpu = NES::mock();
        cpu.set_pc(0);
        cpu.set_mem(0, 0x34);
        cpu.set_mem(1, 0x41);
        let cpu = RefCell::new(cpu);
        let mut opcode = jmp(&cpu);
        let mut cycle = CPUCycle {
            instruction: InstructionName::JMP,
            mode: AddressingMode::Absolute,
            cycle: 0,
        };
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().get_pc(), 0);
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().get_pc(), 1);
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Complete(cycle));
        assert_eq!(cpu.borrow().get_pc(), 0x4134);
    }
}
