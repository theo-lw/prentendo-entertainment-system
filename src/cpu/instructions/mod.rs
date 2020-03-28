pub mod adc;
pub mod asl;
pub mod pha;
pub mod php;
pub mod pla;
pub mod plp;

use crate::cpu::state::CPU;
use std::{cell::RefCell, fmt::Debug, rc::Rc};

pub trait Instruction {
    fn name(&self) -> InstructionName;
}

pub trait Read: Instruction {
    fn execute(&self, cpu: &Rc<RefCell<CPU>>, addr: u16);
}

pub trait Write: Instruction {
    fn execute(&self, cpu: &Rc<RefCell<CPU>>, addr: u16);
}

pub trait Modify: Instruction {
    fn execute(&self, cpu: &Rc<RefCell<CPU>>, addr: u16, old_val: u8);
}

pub trait Implied: Instruction {
    fn execute(&self, cpu: &Rc<RefCell<CPU>>);
}

pub trait PushStack: Instruction {
    fn get(&self, cpu: &Rc<RefCell<CPU>>) -> u8;
}

pub trait PullStack: Instruction {
    fn set(&self, cpu: &Rc<RefCell<CPU>>, val: u8);
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum InstructionName {
    ADC,
    ASL,
    BRK,
    RTI,
    RTS,
    PHA,
    PHP,
    PLA,
    PLP,
    JSR
}
