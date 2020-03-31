use crate::cpu::{
    instructions::Branch,
    opcode_generators::{AddressingMode, CPUCycle},
    state::CPU,
};
use std::{cell::RefCell, ops::Generator, pin::Pin, rc::Rc};

/// Creates the opcode for instructions with relative addressing.
pub fn relative<'a, T: Branch + 'a>(
    cpu: &'a Rc<RefCell<CPU>>,
    instruction: T,
) -> Pin<Box<dyn Generator<Yield = CPUCycle, Return = CPUCycle> + 'a>> {
    Box::pin(move || {
        let mut cycle = CPUCycle {
            instruction: instruction.name(),
            mode: AddressingMode::Relative,
            cycle: 0,
        };
        yield cycle;
        cycle.next();
        let operand: u8 = cpu.borrow_mut().get_and_increment_pc();
        let branch: bool = instruction.should_branch(cpu);
        if branch {
            yield cycle;
            cycle.next();
            let [pc_high, pc_low]: [u8; 2] = cpu.borrow().registers.pc.to_be_bytes();
            let new_pc_low: u8 = pc_low.wrapping_add(operand);
            cpu.borrow_mut().registers.set_pcl(new_pc_low);
            // if a negative overflow occurs
            if new_pc_low > pc_low && (operand as i8) < 0 {
                yield cycle;
                cycle.next();
                let pc_high = pc_high.wrapping_sub(1);
                cpu.borrow_mut().registers.set_pch(pc_high);
            // if a positive overflow occurs
            } else if new_pc_low < pc_low && (operand as i8) > 0 {
                yield cycle;
                cycle.next();
                let pc_high = pc_high.wrapping_add(1);
                cpu.borrow_mut().registers.set_pch(pc_high);
            }
        }
        cycle
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        address::AddressMap,
        cpu::{
            instructions::{bcc::BCC, Instruction},
            state::registers::Flag,
        },
    };
    use std::ops::GeneratorState;

    #[test]
    fn test_relative_positive_overflow() {
        let mut cpu = CPU::mock();
        cpu.registers.clear_flag(Flag::C);
        cpu.registers.pc = 0x10F5;
        cpu.memory.set(cpu.registers.pc, 13i8 as u8);
        let cpu = Rc::new(RefCell::new(cpu));
        let instruction = BCC;
        let mut cycle = CPUCycle {
            instruction: instruction.name(),
            mode: AddressingMode::Relative,
            cycle: 0,
        };
        let mut opcode = relative(&cpu, instruction);
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().registers.pc, 0x10F5);
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().registers.pc, 0x10F6);
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().registers.pc, 0x1003);
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Complete(cycle));
        assert_eq!(cpu.borrow().registers.pc, 0x1103);
    }

    #[test]
    fn test_relative_negative_overflow() {
        let mut cpu = CPU::mock();
        cpu.registers.clear_flag(Flag::C);
        cpu.registers.pc = 0x2204;
        cpu.memory.set(cpu.registers.pc, -30i8 as u8);
        let cpu = Rc::new(RefCell::new(cpu));
        let instruction = BCC;
        let mut cycle = CPUCycle {
            instruction: instruction.name(),
            mode: AddressingMode::Relative,
            cycle: 0,
        };
        let mut opcode = relative(&cpu, instruction);
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().registers.pc, 0x2204);
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().registers.pc, 0x2205);
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().registers.pc, 0x22E7);
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Complete(cycle));
        assert_eq!(cpu.borrow().registers.pc, 0x21E7);
    }

    #[test]
    fn test_relative_no_overflow() {
        let mut cpu = CPU::mock();
        cpu.registers.clear_flag(Flag::C);
        cpu.registers.pc = 0x2204;
        cpu.memory.set(cpu.registers.pc, 30i8 as u8);
        let cpu = Rc::new(RefCell::new(cpu));
        let instruction = BCC;
        let mut cycle = CPUCycle {
            instruction: instruction.name(),
            mode: AddressingMode::Relative,
            cycle: 0,
        };
        let mut opcode = relative(&cpu, instruction);
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().registers.pc, 0x2204);
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().registers.pc, 0x2205);
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Complete(cycle));
        assert_eq!(cpu.borrow().registers.pc, 0x2223);
    }

    #[test]
    fn test_relative_no_branch() {
        let mut cpu = CPU::mock();
        cpu.registers.set_flag(Flag::C);
        cpu.registers.pc = 0x2204;
        cpu.memory.set(cpu.registers.pc, 30i8 as u8);
        let cpu = Rc::new(RefCell::new(cpu));
        let instruction = BCC;
        let mut cycle = CPUCycle {
            instruction: instruction.name(),
            mode: AddressingMode::Relative,
            cycle: 0,
        };
        let mut opcode = relative(&cpu, instruction);
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().registers.pc, 0x2204);
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Complete(cycle));
        assert_eq!(cpu.borrow().registers.pc, 0x2205);
    }
}
