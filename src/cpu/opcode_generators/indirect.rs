use crate::{
    address::AddressMap,
    cpu::{
        instructions::InstructionName,
        opcode_generators::{AddressingMode, CPUCycle},
        state::CPU,
    },
};
use std::{cell::RefCell, ops::Generator, pin::Pin, rc::Rc};

/// Creates the JMP opcode with indirect addressing
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
        cycle
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::GeneratorState;

    #[test]
    fn test_jmp() {
        let mut cpu = CPU::mock();
        let new_pc_low: u8 = 0x32;
        let new_pc_high: u8 = 0x14;
        cpu.registers.pc = 0;
        cpu.memory.set(cpu.registers.pc, 0x34);
        cpu.memory.set(cpu.registers.pc + 1, 0x41);
        cpu.memory.set(0x4134, new_pc_low);
        cpu.memory.set(0x4135, new_pc_high);
        let cpu = Rc::new(RefCell::new(cpu));
        let mut opcode = jmp(&cpu);
        let mut cycle = CPUCycle {
            instruction: InstructionName::JMP,
            mode: AddressingMode::Indirect,
            cycle: 0,
        };
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().registers.pc, 0);
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().registers.pc, 1);
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().registers.pc, 2);
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Yielded(cycle));
        assert_eq!(cpu.borrow().registers.pc, 2);
        cycle.next();
        let state = opcode.as_mut().resume(());
        assert_eq!(state, GeneratorState::Complete(cycle));
        assert_eq!(
            cpu.borrow().registers.pc,
            u16::from_be_bytes([new_pc_high, new_pc_low])
        );
    }
}
