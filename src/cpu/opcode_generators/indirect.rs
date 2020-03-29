use crate::{
    address::AddressMap,
    cpu::{
        instructions::{InstructionName},
        opcode_generators::{AddressingMode, CPUCycle},
        state::CPU,
    },
};
use std::{cell::RefCell, ops::Generator, pin::Pin, rc::Rc};

pub fn jmp<'a>(
    cpu: &'a Rc<RefCell<CPU>>,
) -> Pin<Box<dyn Generator<Yield = CPUCycle, Return = CPUCycle> + 'a>> {
    Box::pin(move || {
        let mut cycle = CPUCycle {
            instruction: InstructionName::JMP,
            mode: AddressingMode::Indirect,
            cycle: 0,
        };
        yield cycle;
        cycle.next();
        let pointer_low: u8 = cpu.borrow_mut().get_and_increment_pc();
        yield cycle;
        cycle.next();
        let pointer_high: u8 = cpu.borrow_mut().get_and_increment_pc();
        let pointer: u16 = u16::from_be_bytes([pointer_high, pointer_low]);
        yield cycle;
        cycle.next();
        let low_byte: u8 = cpu.borrow().memory.get(pointer);
        yield cycle;
        cycle.next();
        let high_byte: u8 = cpu.borrow().memory.get(pointer.wrapping_add(1));
        cpu.borrow_mut().registers.pc = u16::from_be_bytes([high_byte, low_byte]);
        return cycle;
    })
}
