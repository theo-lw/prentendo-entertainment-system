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
        cpu.borrow_mut().push_stack(instruction.get(cpu));
        return cycle;
    })
}

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
        instruction.set(cpu, cpu.borrow().top_stack());
        return cycle;
    })
}

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
        cpu.borrow_mut().registers.p = cpu.borrow().top_stack();
        cpu.borrow_mut().pop_stack();
        yield cycle;
        cycle.next();
        cpu.borrow_mut().registers.set_pcl(cpu.borrow().top_stack());
        cpu.borrow_mut().pop_stack();
        yield cycle;
        cycle.next();
        cpu.borrow_mut().registers.set_pch(cpu.borrow().top_stack());
        return cycle;
    })
}

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
        cpu.borrow_mut().registers.set_pcl(cpu.borrow().top_stack());
        cpu.borrow_mut().pop_stack();
        yield cycle;
        cycle.next();
        cpu.borrow_mut().registers.set_pch(cpu.borrow().top_stack());
        yield cycle;
        cycle.next();
        cpu.borrow_mut().registers.increment_pc();
        return cycle;
    })
}

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
        cpu.borrow_mut().push_stack(cpu.borrow().registers.get_pch());
        yield cycle;
        cycle.next();
        cpu.borrow_mut().push_stack(cpu.borrow().registers.get_pcl());
        yield cycle;
        cycle.next();
        cpu.borrow_mut().registers.set_flag(Flag::B);
        cpu.borrow_mut().push_stack(cpu.borrow().registers.p);
        yield cycle;
        cycle.next();
        cpu.borrow_mut().registers.set_pcl(cpu.borrow().memory.get(0xFFFE));
        yield cycle;
        cycle.next();
        cpu.borrow_mut().registers.set_pch(cpu.borrow().memory.get(0xFFFF));
        return cycle;
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{address::AddressMap, cpu::instructions::{Instruction, asl::ASL}};
    use std::ops::GeneratorState;

    #[test]
    fn test_implied() {
        let mut cpu = CPU::mock();
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
    }

    #[test]
    fn test_push_stack() {

    }
}
