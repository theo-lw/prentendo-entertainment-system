use crate::{
    cpu::{
        instructions::{
            InstructionName, Implied, PullStack, PushStack,
        },
        opcode_generators::{AddressingMode, CPUCycle},
        state::CPU,
    },
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

/*
pub fn create_opcode<'a>(
    cpu: &'a Rc<RefCell<CPU>>,
    instruction: impl Instruction + 'a,
) -> impl Generator + 'a {
    move || {
        yield format!("Implied {:?}", instruction);
        instruction.execute(cpu);
        return;
    }
}
*/
