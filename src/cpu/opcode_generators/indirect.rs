use crate::{
    cpu::{
        instructions::InstructionName,
        opcode_generators::{AddressingMode, CPUCycle},
    },
    state::CPU,
};
use std::{cell::RefCell, ops::Generator, pin::Pin};

/// Creates the JMP opcode with indirect addressing
pub fn jmp<'a, S: CPU>(
    cpu: &'a RefCell<S>,
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
        let low_byte: u8 = cpu.borrow().get_mem(pointer);
        yield cycle;
        cycle.next();
        let high_byte: u8 = if pointer_low == 0xFF {
            cpu.borrow().get_mem(u16::from_be_bytes([pointer_high, 0]))
        } else {
            cpu.borrow().get_mem(pointer.wrapping_add(1))
        };
        cpu.borrow_mut()
            .set_pc(u16::from_be_bytes([high_byte, low_byte]));
        cycle
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::cpu::{Memory, Registers};
    use crate::state::NES;
    use std::ops::GeneratorState;

    #[test]
    fn test_jmp() {
        let mut cpu = NES::mock();
        let new_pc_low: u8 = 0x32;
        let new_pc_high: u8 = 0x14;
        cpu.set_pc(0);
        cpu.set_mem(cpu.get_pc(), 0x34);
        cpu.set_mem(cpu.get_pc() + 1, 0x05);
        cpu.set_mem(0x0534, new_pc_low);
        cpu.set_mem(0x0535, new_pc_high);
        let cpu = RefCell::new(cpu);
        let mut opcode = jmp(&cpu);
        let mut cycle = CPUCycle {
            instruction: InstructionName::JMP,
            mode: AddressingMode::Indirect,
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
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().get_pc(), 2);
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().get_pc(), 2);
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Complete(cycle));
        assert_eq!(
            cpu.borrow().get_pc(),
            u16::from_be_bytes([new_pc_high, new_pc_low])
        );
    }
}
