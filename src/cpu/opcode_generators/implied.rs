use crate::{
    cpu::{
        instructions::{
            InstructionName, Implied, PushStack, PullStack
        },
        opcode_generators::{AddressingMode, CPUCycle},
        state::{CPU, registers::Flag},
    },
    address::AddressMap,
};
use std::{cell::RefCell, ops::Generator, pin::Pin, rc::Rc};

/// Creates the opcode for instructions with implied addressing
pub fn implied<'a, T: Implied + 'a>(
    cpu: &'a Rc<RefCell<CPU>>,
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
        instruction.execute(cpu);
        return cycle;
    })
}

/// Creates the opcode for instructions that push values to the stack (with implied addressing)
pub fn push_stack<'a, T: PushStack + 'a>(
    cpu: &'a Rc<RefCell<CPU>>,
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
        let register: u8 = instruction.get(cpu);
        cpu.borrow_mut().push_stack(register);
        return cycle;
    })
}

/// Creates the opcode for instructions that pull values from the stack (with implied addressing)
pub fn pull_stack<'a, T: PullStack + 'a>(
    cpu: &'a Rc<RefCell<CPU>>,
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
        instruction.set(cpu, top);
        return cycle;
    })
}

/// Creates the RTI opcode (which has implied addressing)
pub fn rti<'a>(
    cpu: &'a Rc<RefCell<CPU>>,
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
        cpu.borrow_mut().registers.p = top;
        cpu.borrow_mut().pop_stack();
        yield cycle;
        cycle.next();
        let top: u8 = cpu.borrow().top_stack();
        cpu.borrow_mut().registers.set_pcl(top);
        cpu.borrow_mut().pop_stack();
        yield cycle;
        cycle.next();
        let top: u8 = cpu.borrow().top_stack();
        cpu.borrow_mut().registers.set_pch(top);
        return cycle;
    })
}

/// Creates the RTS opcode (which has implied addressing)
pub fn rts<'a>(
    cpu: &'a Rc<RefCell<CPU>>,
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
        cpu.borrow_mut().registers.set_pcl(top);
        cpu.borrow_mut().pop_stack();
        yield cycle;
        cycle.next();
        let top: u8 = cpu.borrow().top_stack();
        cpu.borrow_mut().registers.set_pch(top);
        yield cycle;
        cycle.next();
        cpu.borrow_mut().registers.increment_pc();
        return cycle;
    })
}

/// Creates the BRK opcode (which has implied addressing)
pub fn brk<'a>(
    cpu: &'a Rc<RefCell<CPU>>,
) -> Pin<Box<dyn Generator<Yield = CPUCycle, Return = CPUCycle> + 'a>> {
    Box::pin(move || {
        let mut cycle = CPUCycle {
            instruction: InstructionName::BRK,
            mode: AddressingMode::Implied,
            cycle: 0,
        };
        yield cycle;
        cycle.next();
        cpu.borrow_mut().registers.increment_pc();
        yield cycle;
        cycle.next();
        let pc_high: u8 = cpu.borrow().registers.get_pch();
        cpu.borrow_mut().push_stack(pc_high);
        yield cycle;
        cycle.next();
        let pc_low: u8 = cpu.borrow().registers.get_pcl();
        cpu.borrow_mut().push_stack(pc_low);
        yield cycle;
        cycle.next();
        cpu.borrow_mut().registers.set_flag(Flag::B);
        let p_register: u8 = cpu.borrow().registers.p;
        cpu.borrow_mut().push_stack(p_register);
        yield cycle;
        cycle.next();
        let interrupt_low: u8 = cpu.borrow().memory.get(0xFFFE);
        cpu.borrow_mut().registers.set_pcl(interrupt_low);
        yield cycle;
        cycle.next();
        let interrupt_high: u8 = cpu.borrow().memory.get(0xFFFF);
        cpu.borrow_mut().registers.set_pch(interrupt_high);
        return cycle;
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{address::AddressMap, cpu::instructions::{Instruction, asl::ASL, pha::PHA, plp::PLP}};
    use std::ops::GeneratorState;

    #[test]
    fn test_implied() {
        let mut cpu = CPU::mock();
        cpu.registers.pc = 0;
        cpu.registers.a = 0b0110_1010;
        let cpu = Rc::new(RefCell::new(cpu));
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
            assert_eq!(cpu.borrow().registers.a, 0b0110_1010);
            cycle.next();
        }
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Complete(cycle));
        assert_eq!(cpu.borrow().registers.a, 0b1101_0100);
        assert_eq!(cpu.borrow().registers.pc, 0);
    }

    #[test]
    fn test_push_stack() {
        let mut cpu = CPU::mock();
        let a_register: u8 = 0b0110_1010; 
        cpu.registers.a = a_register;
        let initial_stack_pointer: u8 = 0xFF;
        cpu.registers.s = initial_stack_pointer;
        cpu.memory.set(0x01FF, 2);
        let cpu = Rc::new(RefCell::new(cpu));
        let instruction = PHA;
        let mut opcode = push_stack(&cpu, instruction);
        let mut cycle = CPUCycle {
            instruction: instruction.name(),
            mode: AddressingMode::Implied,
            cycle: 0,
        };
        for _ in 0..2 {
            let state = opcode.as_mut().resume(());
            assert_eq!(state, GeneratorState::Yielded(cycle));
            assert_eq!(cpu.borrow().registers.s, initial_stack_pointer);
            assert_eq!(cpu.borrow().top_stack(), 2);
            cycle.next();
        }
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Complete(cycle));
        assert_eq!(cpu.borrow().registers.s, initial_stack_pointer - 1);
        cpu.borrow_mut().pop_stack();
        assert_eq!(cpu.borrow().top_stack(), a_register);
    }

    #[test]
    fn test_pull_stack() {
        let mut cpu = CPU::mock();
        let p_register_old: u8 = 0b0110_1010;
        let p_register_new: u8 = 0b1111_0000;
        let initial_stack_pointer: u8 = 0xFF;
        cpu.registers.p = p_register_old;
        cpu.registers.s = initial_stack_pointer;
        cpu.push_stack(p_register_new);
        let cpu = Rc::new(RefCell::new(cpu));
        let instruction = PLP;
        let mut opcode = pull_stack(&cpu, instruction);
        let mut cycle = CPUCycle {
            instruction: instruction.name(),
            mode: AddressingMode::Implied,
            cycle: 0,
        };
        for _ in 0..2 {
            let state = opcode.as_mut().resume(());
            assert_eq!(state, GeneratorState::Yielded(cycle));
            assert_eq!(cpu.borrow().registers.s, initial_stack_pointer - 1);
            assert_eq!(cpu.borrow().registers.p, p_register_old);
            cycle.next();
        }
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().registers.s, initial_stack_pointer);
        assert_eq!(cpu.borrow().registers.p, p_register_old);
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Complete(cycle));
        assert_eq!(cpu.borrow().registers.s, initial_stack_pointer);
        assert_eq!(cpu.borrow().registers.p, p_register_new);
    }

    #[test]
    fn test_rti() {
        let mut cpu = CPU::mock();
        let p_register_old: u8 = 0;
        let pc_old: u16 = 0;
        let initial_stack_pointer: u8 = 0xFF;
        cpu.registers.p = p_register_old;
        cpu.registers.pc = pc_old;
        cpu.registers.s = initial_stack_pointer;
        let p_register_new: u8 = 0b1001_0101;
        let pc_low_new: u8 = 0b1101_0010; 
        let pc_high_new: u8 = 0b1111_0000;
        cpu.push_stack(pc_high_new);
        cpu.push_stack(pc_low_new);
        cpu.push_stack(p_register_new);
        let cpu = Rc::new(RefCell::new(cpu));
        let mut opcode = rti(&cpu);
        let mut cycle = CPUCycle {
            instruction: InstructionName::RTI,
            mode: AddressingMode::Implied,
            cycle: 0,
        };
        for _ in 0..2 {
            let state = opcode.as_mut().resume(());
            assert_eq!(state, GeneratorState::Yielded(cycle));
            assert_eq!(cpu.borrow().registers.s, initial_stack_pointer - 3);
            assert_eq!(cpu.borrow().registers.p, p_register_old);
            assert_eq!(cpu.borrow().registers.pc, pc_old);
            cycle.next();
        }
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().registers.s, initial_stack_pointer - 2);
        assert_eq!(cpu.borrow().registers.p, p_register_old);
        assert_eq!(cpu.borrow().registers.pc, pc_old);
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().registers.s, initial_stack_pointer - 1);
        assert_eq!(cpu.borrow().registers.p, p_register_new);
        assert_eq!(cpu.borrow().registers.pc, pc_old);
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().registers.s, initial_stack_pointer);
        assert_eq!(cpu.borrow().registers.p, p_register_new);
        assert_eq!(cpu.borrow().registers.pc, u16::from_be_bytes([0, pc_low_new]));
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Complete(cycle));
        assert_eq!(cpu.borrow().registers.s, initial_stack_pointer);
        assert_eq!(cpu.borrow().registers.p, p_register_new);
        assert_eq!(cpu.borrow().registers.pc, u16::from_be_bytes([pc_high_new, pc_low_new]));
    }

    #[test]
    fn test_rts() {
        let mut cpu = CPU::mock();
        let initial_stack_pointer: u8 = 0xFF;
        let pc_old: u16 = 0;
        let pc_low_new: u8 = 0b1101_0010;
        let pc_high_new: u8 = 0b1111_0000;
        cpu.registers.pc = pc_old;
        cpu.registers.s = initial_stack_pointer;
        cpu.push_stack(pc_high_new);
        cpu.push_stack(pc_low_new);
        let cpu = Rc::new(RefCell::new(cpu));
        let mut opcode = rts(&cpu);
        let mut cycle = CPUCycle {
            instruction: InstructionName::RTS,
            mode: AddressingMode::Implied,
            cycle: 0,
        };
        for _ in 0..2 {
            let state = opcode.as_mut().resume(());
            assert_eq!(state, GeneratorState::Yielded(cycle));
            assert_eq!(cpu.borrow().registers.s, initial_stack_pointer - 2);
            assert_eq!(cpu.borrow().registers.pc, pc_old);
            cycle.next();
        }
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().registers.s, initial_stack_pointer - 1);
        assert_eq!(cpu.borrow().registers.pc, pc_old);
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().registers.s, initial_stack_pointer);
        assert_eq!(cpu.borrow().registers.pc, u16::from_be_bytes([0, pc_low_new]));
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().registers.s, initial_stack_pointer);
        assert_eq!(cpu.borrow().registers.pc, u16::from_be_bytes([pc_high_new, pc_low_new]));
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Complete(cycle));
        assert_eq!(cpu.borrow().registers.s, initial_stack_pointer);
        assert_eq!(cpu.borrow().registers.pc, u16::from_be_bytes([pc_high_new, pc_low_new])+1);
    }

    #[test]
    fn test_brk() {
        let mut cpu = CPU::mock();
        let [pc_high_old, pc_low_old]: [u8; 2] = [0b1101_0010, 0b0100_1100];
        let p_register = 0b0010_1000;
        let [pc_high_new, pc_low_new]: [u8; 2] = [0b1001_0001, 0b0110_0010];
        let initial_stack_pointer: u8 = 0xFF;
        cpu.registers.pc = u16::from_be_bytes([pc_high_old, pc_low_old]);
        cpu.registers.p = p_register;
        cpu.registers.s = initial_stack_pointer;
        cpu.memory.set(0xFFFE, pc_low_new);
        cpu.memory.set(0xFFFF, pc_high_new);
        let cpu = Rc::new(RefCell::new(cpu));
        let mut opcode = brk(&cpu);
        let mut cycle = CPUCycle {
            instruction: InstructionName::BRK,
            mode: AddressingMode::Implied,
            cycle: 0,
        };
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().registers.s, initial_stack_pointer);
        assert_eq!(cpu.borrow().registers.pc, u16::from_be_bytes([pc_high_old, pc_low_old]));
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().registers.s, initial_stack_pointer);
        assert_eq!(cpu.borrow().registers.pc, u16::from_be_bytes([pc_high_old, pc_low_old])+1);
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().registers.s, initial_stack_pointer - 1);
        assert_eq!(cpu.borrow().memory.get(0x01FF), pc_high_old);
        assert_eq!(cpu.borrow().registers.pc, u16::from_be_bytes([pc_high_old, pc_low_old])+1);
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().registers.s, initial_stack_pointer - 2);
        assert_eq!(cpu.borrow().memory.get(0x01FF), pc_high_old);
        assert_eq!(cpu.borrow().memory.get(0x01FE), pc_low_old+1);
        assert_eq!(cpu.borrow().registers.pc, u16::from_be_bytes([pc_high_old, pc_low_old])+1);
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().registers.s, initial_stack_pointer - 3);
        assert_eq!(cpu.borrow().memory.get(0x01FF), pc_high_old);
        assert_eq!(cpu.borrow().memory.get(0x01FE), pc_low_old+1);
        assert_eq!(cpu.borrow().memory.get(0x01FD), p_register | 0b0001_0000);
        assert_eq!(cpu.borrow().registers.pc, u16::from_be_bytes([pc_high_old, pc_low_old])+1);
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().registers.s, initial_stack_pointer - 3);
        assert_eq!(cpu.borrow().memory.get(0x01FF), pc_high_old);
        assert_eq!(cpu.borrow().memory.get(0x01FE), pc_low_old+1);
        assert_eq!(cpu.borrow().memory.get(0x01FD), p_register | 0b0001_0000);
        assert_eq!(cpu.borrow().registers.pc, u16::from_be_bytes([pc_high_old, pc_low_new]));
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Complete(cycle));
        assert_eq!(cpu.borrow().registers.s, initial_stack_pointer - 3);
        assert_eq!(cpu.borrow().memory.get(0x01FF), pc_high_old);
        assert_eq!(cpu.borrow().memory.get(0x01FE), pc_low_old+1);
        assert_eq!(cpu.borrow().memory.get(0x01FD), p_register | 0b0001_0000);
        assert_eq!(cpu.borrow().registers.pc, u16::from_be_bytes([pc_high_new, pc_low_new]));
    }
}
