use crate::{
    address::AddressMap,
    cpu::{
        instructions::{Implied, Instruction, others::{RTI, RTS, PHA, PHP, PLA, PLP}},
        opcode_generators::{AddressingMode, CPUCycle},
        state::CPU,
    },
};
use std::{cell::RefCell, ops::Generator, pin::Pin, rc::Rc};

pub fn implied<'a, T: Implied + 'a>(
    cpu: &'a Rc<RefCell<CPU>>,
    instruction: T,
) -> Pin<Box<dyn Generator<Yield = CPUCycle<T>, Return = CPUCycle<T>> + 'a>> {
    Box::pin(move || {
        let mut cycle = CPUCycle {
            instruction,
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
) -> Pin<Box<dyn Generator<Yield = CPUCycle<RTI>, Return = CPUCycle<RTI>> + 'a>> {
    Box::pin(move || {
        let mut cycle = CPUCycle {
            instruction: RTI,
            mode: AddressingMode::Implied,
            cycle: 0,
        };
        yield cycle;
        cycle.next();
        yield cycle;
        cycle.next();
        cpu.borrow_mut().pop_stack();
        yield cycle;
        cycle.next();
        cpu.borrow_mut().registers.p = cpu.borrow().top_stack();
        cpu.borrow_mut().pop_stack();
        yield cycle;
        cycle.next();
        let [high, _]: [u8; 2] = cpu.borrow().registers.pc.to_be_bytes();
        cpu.borrow_mut().registers.pc = u16::from_be_bytes([high, cpu.borrow().top_stack()]);
        cpu.borrow_mut().pop_stack();
        yield cycle;
        cycle.next();
        let [_, low]: [u8; 2] = cpu.borrow().registers.pc.to_be_bytes();
        cpu.borrow_mut().registers.pc = u16::from_be_bytes([cpu.borrow().top_stack(), low]);
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
