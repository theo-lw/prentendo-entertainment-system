pub mod adc;
pub mod asl;
pub mod others;

use crate::cpu::state::CPU;
use std::{cell::RefCell, fmt::Debug, rc::Rc};

pub trait Instruction: Debug + Copy + Clone + PartialEq {}

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
