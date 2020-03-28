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
