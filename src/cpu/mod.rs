pub mod instructions;
pub mod opcode_generators;
pub mod state;
pub mod variables;

use crate::address::AddressMap;
use instructions::{adc::ADC, nop::NOP, Instruction};
use opcode_generators::{absolute, absolute_x, implied, CPUCycle};
use state::CPU;
use std::{
    cell::RefCell,
    ops::{Generator, GeneratorState},
    pin::Pin,
    rc::Rc,
};

/// This module contains CPU-related code.
/// It has three sub-modules: `instructions`, `opcode_generators` and `state`.
///
/// `state` is the simplest of them - it holds the state of the CPU, which includes code related to
/// the registers and the memory map.
///
/// To understand `opcode_generators` and `instructions`, we have to understand the anatomy of an
/// opcode. Consider the following opcode: `ADC #10`. It consists of two parts, an *instruction* and
/// an *addressing mode*. `ADC` specifies the *instruction*, which is to add the contents of a memory
/// location to the accumulator together with the carry bit. The memory location is specified by the
/// *addressing mode*. In this case an immediate addressing mode is used, so the memory location is
/// the byte after the opcode.
///
/// The code related to addressing modes can be found in `opcode_generators`.
///
/// Instruction-related code can be found under `instructions`.

pub fn cycle<'a, T: Instruction>(cpu: &'a Rc<RefCell<CPU>>) -> impl Generator + 'a {
    move || loop {
        let mut generator = get_instruction(cpu); 
        'opcode: loop {
            match generator.as_mut().resume(()) {
                GeneratorState::Yielded(x) => {
                    yield x;
                }
                GeneratorState::Complete(x) => {
                    yield x;
                    break 'opcode;
                }
            }
        }
    }
}

fn get_instruction<'a>(
    cpu: &'a Rc<RefCell<CPU>>,
) -> Pin<Box<dyn Generator<Yield = CPUCycle, Return = CPUCycle> + 'a>> {
    let opcode: u8 = cpu.borrow_mut().get_and_increment_pc();
    match opcode {
        // we treat unofficial opcodes (and unimplemented ones) as being NOP
        // it is not strictly correct, but it will have to do for now 
        _ => implied::implied(cpu, NOP)
    }
}
