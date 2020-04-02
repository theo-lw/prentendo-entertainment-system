pub mod x_register;
pub mod y_register;
pub mod a_register;
pub mod stack_pointer;
pub mod p_register;

use std::cell::RefCell;
use std::rc::Rc;
use crate::cpu::state::CPU;

pub trait Register {
    fn name(&self) -> RegisterName;
}

pub trait Get: Register {
    fn get(&self, cpu: &Rc<RefCell<CPU>>) -> u8;
}

pub trait Set: Register {
    fn set(&self, cpu: &Rc<RefCell<CPU>>, val: u8);
}

/// Flags are a special type of variable
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Flag {
    C = 0,
    Z = 1,
    I = 2,
    D = 3,
    V = 6,
    N = 7,
    B = 4,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RegisterName {
    X,
    Y,
    A,
    SP,
    P,
}
