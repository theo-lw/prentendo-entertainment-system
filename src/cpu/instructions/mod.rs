pub mod adc;

use crate::cpu::state::CPU;
use std::{cell::RefCell, fmt::Debug, rc::Rc};

pub trait Read: Debug {
    fn execute(&self, cpu: &Rc<RefCell<CPU>>, addr: u16);
}

pub trait Write: Debug {
    fn execute(&self, cpu: &Rc<RefCell<CPU>>, addr: u16);
}

pub trait ReadModifyWrite: Debug {
    fn execute(&self, cpu: &Rc<RefCell<CPU>>, addr: u16, old_val: u8);
}
