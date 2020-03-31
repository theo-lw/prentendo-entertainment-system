pub mod instructions;
pub mod opcode_generators;
pub mod state;

use crate::address::AddressMap;
use instructions::{adc::ADC, Instruction};
use opcode_generators::{absolute, absolute_x};
use state::CPU;
use std::{
    cell::RefCell,
    ops::{Generator, GeneratorState},
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
        let opcode: u8 = cpu.borrow().memory.get(cpu.borrow().registers.pc);
        cpu.borrow_mut().registers.pc += 1;
        let mut generator = if opcode == 0 {
            absolute_x::read(cpu, ADC)
        } else {
            absolute::read(cpu, ADC)
        };
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
