use crate::{
    bitops::BitOps,
    cpu::{
        instructions::{Implied, InstructionName, PullStack, PushStack},
        opcode_generators::{AddressingMode, CPUCycle},
        variables::Flag,
    },
    state::CPU,
};
use std::{cell::RefCell, ops::Generator, pin::Pin};

const BRK_VECTOR: u16 = 0xFFFE;

/// Creates the opcode for instructions with implied/accumulator addressing.
pub fn implied<'a, T: Implied<S> + 'a, S: CPU>(
    cpu: &'a RefCell<S>,
    instruction: T,
) -> Pin<Box<dyn Generator<Yield = CPUCycle, Return = CPUCycle> + 'a>> {
    Box::pin(move || {
        let mut cycle = CPUCycle {
            instruction: instruction.name(),
            mode: AddressingMode::Implied,
            cycle: 0,
        };
        yield cycle;
        cycle.next();
        instruction.execute(&mut cpu.borrow_mut());
        cycle
    })
}

/// Creates the opcode for instructions that push values to the stack (with implied addressing)
pub fn push_stack<'a, T: PushStack<S> + 'a, S: CPU>(
    cpu: &'a RefCell<S>,
    instruction: T,
) -> Pin<Box<dyn Generator<Yield = CPUCycle, Return = CPUCycle> + 'a>> {
    Box::pin(move || {
        let mut cycle = CPUCycle {
            instruction: instruction.name(),
            mode: AddressingMode::Implied,
            cycle: 0,
        };
        yield cycle;
        cycle.next();
        // read and throw away next byte
        yield cycle;
        cycle.next();
        let register: u8 = instruction.get(&cpu.borrow_mut());
        cpu.borrow_mut().push_stack(register);
        cycle
    })
}

/// Creates the opcode for instructions that pull values from the stack (with implied addressing)
pub fn pull_stack<'a, T: PullStack<S> + 'a, S: CPU>(
    cpu: &'a RefCell<S>,
    instruction: T,
) -> Pin<Box<dyn Generator<Yield = CPUCycle, Return = CPUCycle> + 'a>> {
    Box::pin(move || {
        let mut cycle = CPUCycle {
            instruction: instruction.name(),
            mode: AddressingMode::Implied,
            cycle: 0,
        };
        yield cycle;
        cycle.next();
        // read and throw away next byte
        yield cycle;
        cycle.next();
        cpu.borrow_mut().pop_stack();
        yield cycle;
        cycle.next();
        let top: u8 = cpu.borrow().top_stack();
        instruction.set(&mut cpu.borrow_mut(), top);
        cycle
    })
}

/// Creates the RTI opcode (which has implied addressing)
pub fn rti<'a, S: CPU>(
    cpu: &'a RefCell<S>,
) -> Pin<Box<dyn Generator<Yield = CPUCycle, Return = CPUCycle> + 'a>> {
    Box::pin(move || {
        let mut cycle = CPUCycle {
            instruction: InstructionName::RTI,
            mode: AddressingMode::Implied,
            cycle: 0,
        };
        yield cycle;
        cycle.next();
        // throw away next instruction byte
        yield cycle;
        cycle.next();
        cpu.borrow_mut().pop_stack();
        yield cycle;
        cycle.next();
        let top: u8 = cpu.borrow().top_stack();
        cpu.borrow_mut().set_p(top);
        cpu.borrow_mut().pop_stack();
        yield cycle;
        cycle.next();
        let top: u8 = cpu.borrow().top_stack();
        cpu.borrow_mut().set_pcl(top);
        cpu.borrow_mut().pop_stack();
        yield cycle;
        cycle.next();
        let top: u8 = cpu.borrow().top_stack();
        cpu.borrow_mut().set_pch(top);
        cycle
    })
}

/// Creates the RTS opcode (which has implied addressing)
pub fn rts<'a, S: CPU>(
    cpu: &'a RefCell<S>,
) -> Pin<Box<dyn Generator<Yield = CPUCycle, Return = CPUCycle> + 'a>> {
    Box::pin(move || {
        let mut cycle = CPUCycle {
            instruction: InstructionName::RTS,
            mode: AddressingMode::Implied,
            cycle: 0,
        };
        yield cycle;
        cycle.next();
        // throw away next instruction byte
        yield cycle;
        cycle.next();
        cpu.borrow_mut().pop_stack();
        yield cycle;
        cycle.next();
        let top: u8 = cpu.borrow().top_stack();
        cpu.borrow_mut().set_pcl(top);
        cpu.borrow_mut().pop_stack();
        yield cycle;
        cycle.next();
        let top: u8 = cpu.borrow().top_stack();
        cpu.borrow_mut().set_pch(top);
        yield cycle;
        cycle.next();
        cpu.borrow_mut().increment_pc();
        cycle
    })
}

/// Creates the BRK opcode (which has implied addressing)
pub fn brk<'a, S: CPU>(
    cpu: &'a RefCell<S>,
) -> Pin<Box<dyn Generator<Yield = CPUCycle, Return = CPUCycle> + 'a>> {
    Box::pin(move || {
        let mut cycle = CPUCycle {
            instruction: InstructionName::BRK,
            mode: AddressingMode::Implied,
            cycle: 0,
        };
        yield cycle;
        cycle.next();
        cpu.borrow_mut().increment_pc();
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
        let mut p_register: u8 = cpu.borrow().get_p();
        p_register.set_bit(Flag::B as usize);
        cpu.borrow_mut().push_stack(p_register);
        yield cycle;
        cycle.next();
        let interrupt_low: u8 = cpu.borrow().get_mem(BRK_VECTOR);
        cpu.borrow_mut().set_pcl(interrupt_low);
        cpu.borrow_mut().assign_flag(Flag::I, true);
        yield cycle;
        cycle.next();
        let interrupt_high: u8 = cpu.borrow().get_mem(BRK_VECTOR + 1);
        cpu.borrow_mut().set_pch(interrupt_high);
        cycle
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::instructions::{asl::ASL, phr::PH, plr::PL, Instruction};
    use crate::cpu::variables::{a_register::A, p_register::P};
    use crate::state::cpu::{Memory, Registers, Stack};
    use crate::state::NES;
    use std::ops::GeneratorState;

    #[test]
    fn test_implied() {
        let mut cpu = NES::mock();
        cpu.set_pc(0);
        cpu.set_a(0b0110_1010);
        let cpu = RefCell::new(cpu);
        let instruction = ASL;
        let mut opcode = implied(&cpu, instruction);
        let mut cycle = CPUCycle {
            instruction: instruction.name(),
            mode: AddressingMode::Implied,
            cycle: 0,
        };
        for _ in 0..1 {
            let state = opcode.as_mut().resume(());
            assert_eq!(state, GeneratorState::Yielded(cycle));
            assert_eq!(cpu.borrow().get_a(), 0b0110_1010);
            cycle.next();
        }
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Complete(cycle));
        assert_eq!(cpu.borrow().get_a(), 0b1101_0100);
        assert_eq!(cpu.borrow().get_pc(), 0);
    }

    #[test]
    fn test_push_stack() {
        let mut cpu = NES::mock();
        let a_register: u8 = 0b0110_1010;
        cpu.set_a(a_register);
        let initial_stack_pointer: u8 = 0xFF;
        cpu.set_s(initial_stack_pointer);
        cpu.set_mem(0x01FF, 2);
        let cpu = RefCell::new(cpu);
        let instruction = PH(A);
        let mut opcode = push_stack(&cpu, instruction);
        let mut cycle = CPUCycle {
            instruction: instruction.name(),
            mode: AddressingMode::Implied,
            cycle: 0,
        };
        for _ in 0..2 {
            let state = opcode.as_mut().resume(());
            assert_eq!(state, GeneratorState::Yielded(cycle));
            assert_eq!(cpu.borrow().get_s(), initial_stack_pointer);
            assert_eq!(cpu.borrow().top_stack(), 2);
            cycle.next();
        }
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Complete(cycle));
        assert_eq!(cpu.borrow().get_s(), initial_stack_pointer - 1);
        cpu.borrow_mut().pop_stack();
        assert_eq!(cpu.borrow().top_stack(), a_register);
    }

    #[test]
    fn test_pull_stack() {
        let mut cpu = NES::mock();
        let p_register_old: u8 = 0b0110_1010;
        let p_register_new: u8 = 0b1111_0000;
        let initial_stack_pointer: u8 = 0xFF;
        cpu.set_p(p_register_old);
        cpu.set_s(initial_stack_pointer);
        cpu.push_stack(p_register_new);
        let cpu = RefCell::new(cpu);
        let instruction = PL(P);
        let mut opcode = pull_stack(&cpu, instruction);
        let mut cycle = CPUCycle {
            instruction: instruction.name(),
            mode: AddressingMode::Implied,
            cycle: 0,
        };
        for _ in 0..2 {
            let state = opcode.as_mut().resume(());
            assert_eq!(state, GeneratorState::Yielded(cycle));
            assert_eq!(cpu.borrow().get_s(), initial_stack_pointer - 1);
            assert_eq!(cpu.borrow().get_p(), p_register_old);
            cycle.next();
        }
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().get_s(), initial_stack_pointer);
        assert_eq!(cpu.borrow().get_p(), p_register_old);
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Complete(cycle));
        assert_eq!(cpu.borrow().get_s(), initial_stack_pointer);
        assert_eq!(cpu.borrow().get_p(), 0b1110_0000);
    }

    #[test]
    fn test_rti() {
        let mut cpu = NES::mock();
        let p_register_old: u8 = 0b0010_0000;
        let pc_old: u16 = 0;
        let initial_stack_pointer: u8 = 0xFF;
        cpu.set_p(p_register_old);
        cpu.set_pc(pc_old);
        cpu.set_s(initial_stack_pointer);
        let p_register_new: u8 = 0b1000_0101;
        let pc_low_new: u8 = 0b1101_0010;
        let pc_high_new: u8 = 0b1111_0000;
        cpu.push_stack(pc_high_new);
        cpu.push_stack(pc_low_new);
        cpu.push_stack(p_register_new);
        let cpu = RefCell::new(cpu);
        let mut opcode = rti(&cpu);
        let mut cycle = CPUCycle {
            instruction: InstructionName::RTI,
            mode: AddressingMode::Implied,
            cycle: 0,
        };
        for _ in 0..2 {
            let state = opcode.as_mut().resume(());
            assert_eq!(state, GeneratorState::Yielded(cycle));
            assert_eq!(cpu.borrow().get_s(), initial_stack_pointer - 3);
            assert_eq!(cpu.borrow().get_p(), p_register_old);
            assert_eq!(cpu.borrow().get_pc(), pc_old);
            cycle.next();
        }
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().get_s(), initial_stack_pointer - 2);
        assert_eq!(cpu.borrow().get_p(), p_register_old);
        assert_eq!(cpu.borrow().get_pc(), pc_old);
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().get_s(), initial_stack_pointer - 1);
        assert_eq!(cpu.borrow().get_p(), p_register_new | 0b0010_0000);
        assert_eq!(cpu.borrow().get_pc(), pc_old);
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().get_s(), initial_stack_pointer);
        assert_eq!(cpu.borrow().get_p(), p_register_new | 0b0010_0000);
        assert_eq!(cpu.borrow().get_pc(), u16::from_be_bytes([0, pc_low_new]));
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Complete(cycle));
        assert_eq!(cpu.borrow().get_s(), initial_stack_pointer);
        assert_eq!(cpu.borrow().get_p(), p_register_new | 0b0010_0000);
        assert_eq!(
            cpu.borrow().get_pc(),
            u16::from_be_bytes([pc_high_new, pc_low_new])
        );
    }

    #[test]
    fn test_rts() {
        let mut cpu = NES::mock();
        let initial_stack_pointer: u8 = 0xFF;
        let pc_old: u16 = 0;
        let pc_low_new: u8 = 0b1101_0010;
        let pc_high_new: u8 = 0b1111_0000;
        cpu.set_pc(pc_old);
        cpu.set_s(initial_stack_pointer);
        cpu.push_stack(pc_high_new);
        cpu.push_stack(pc_low_new);
        let cpu = RefCell::new(cpu);
        let mut opcode = rts(&cpu);
        let mut cycle = CPUCycle {
            instruction: InstructionName::RTS,
            mode: AddressingMode::Implied,
            cycle: 0,
        };
        for _ in 0..2 {
            let state = opcode.as_mut().resume(());
            assert_eq!(state, GeneratorState::Yielded(cycle));
            assert_eq!(cpu.borrow().get_s(), initial_stack_pointer - 2);
            assert_eq!(cpu.borrow().get_pc(), pc_old);
            cycle.next();
        }
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().get_s(), initial_stack_pointer - 1);
        assert_eq!(cpu.borrow().get_pc(), pc_old);
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().get_s(), initial_stack_pointer);
        assert_eq!(cpu.borrow().get_pc(), u16::from_be_bytes([0, pc_low_new]));
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().get_s(), initial_stack_pointer);
        assert_eq!(
            cpu.borrow().get_pc(),
            u16::from_be_bytes([pc_high_new, pc_low_new])
        );
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Complete(cycle));
        assert_eq!(cpu.borrow().get_s(), initial_stack_pointer);
        assert_eq!(
            cpu.borrow().get_pc(),
            u16::from_be_bytes([pc_high_new, pc_low_new]) + 1
        );
    }

    #[test]
    fn test_brk() {
        let mut cpu = NES::mock();
        let [pc_high_old, pc_low_old]: [u8; 2] = [0b1101_0010, 0b0100_1100];
        let p_register = 0b0010_1000;
        let [pc_high_new, pc_low_new]: [u8; 2] = [0, 0];
        let initial_stack_pointer: u8 = 0xFF;
        cpu.set_p(p_register);
        cpu.set_pc(u16::from_be_bytes([pc_high_old, pc_low_old]));
        cpu.set_s(initial_stack_pointer);
        let cpu = RefCell::new(cpu);
        let mut opcode = brk(&cpu);
        let mut cycle = CPUCycle {
            instruction: InstructionName::BRK,
            mode: AddressingMode::Implied,
            cycle: 0,
        };
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().get_s(), initial_stack_pointer);
        assert_eq!(
            cpu.borrow().get_pc(),
            u16::from_be_bytes([pc_high_old, pc_low_old])
        );
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().get_s(), initial_stack_pointer);
        assert_eq!(
            cpu.borrow().get_pc(),
            u16::from_be_bytes([pc_high_old, pc_low_old]) + 1
        );
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().get_s(), initial_stack_pointer - 1);
        assert_eq!(cpu.borrow().get_mem(0x01FF), pc_high_old);
        assert_eq!(
            cpu.borrow().get_pc(),
            u16::from_be_bytes([pc_high_old, pc_low_old]) + 1
        );
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().get_s(), initial_stack_pointer - 2);
        assert_eq!(cpu.borrow().get_mem(0x01FF), pc_high_old);
        assert_eq!(cpu.borrow().get_mem(0x01FE), pc_low_old + 1);
        assert_eq!(
            cpu.borrow().get_pc(),
            u16::from_be_bytes([pc_high_old, pc_low_old]) + 1
        );
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().get_s(), initial_stack_pointer - 3);
        assert_eq!(cpu.borrow().get_mem(0x01FF), pc_high_old);
        assert_eq!(cpu.borrow().get_mem(0x01FE), pc_low_old + 1);
        assert_eq!(cpu.borrow().get_mem(0x01FD), p_register | 0b0001_0000);
        assert_eq!(
            cpu.borrow().get_pc(),
            u16::from_be_bytes([pc_high_old, pc_low_old]) + 1
        );
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().get_s(), initial_stack_pointer - 3);
        assert_eq!(cpu.borrow().get_mem(0x01FF), pc_high_old);
        assert_eq!(cpu.borrow().get_mem(0x01FE), pc_low_old + 1);
        assert_eq!(cpu.borrow().get_mem(0x01FD), p_register | 0b0001_0000);
        assert_eq!(
            cpu.borrow().get_pc(),
            u16::from_be_bytes([pc_high_old, pc_low_new])
        );
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Complete(cycle));
        assert_eq!(cpu.borrow().get_s(), initial_stack_pointer - 3);
        assert_eq!(cpu.borrow().get_mem(0x01FF), pc_high_old);
        assert_eq!(cpu.borrow().get_mem(0x01FE), pc_low_old + 1);
        assert_eq!(cpu.borrow().get_mem(0x01FD), p_register | 0b0001_0000);
        assert_eq!(
            cpu.borrow().get_pc(),
            u16::from_be_bytes([pc_high_new, pc_low_new])
        );
    }
}
