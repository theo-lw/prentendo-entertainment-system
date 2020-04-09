pub mod a_register;
pub mod p_register;
pub mod stack_pointer;
pub mod x_register;
pub mod y_register;

use crate::state::CPU;

/// This module contains code representing the 'variables' acted on by instructions.

/// Trait representing register variables
pub trait Register {
    fn name(&self) -> RegisterName;
    fn flags_set_on_change(&self) -> bool;
}

/// Trait for register variables that can be queried
pub trait Get: Register {
    fn get(&self, cpu: &dyn CPU) -> u8;
}

/// Trait for register variables that can be set
pub trait Set: Register {
    fn set(&self, cpu: &mut dyn CPU, val: u8);
}

/// Enum representing flag variables
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Flag {
    C = 0,
    Z = 1,
    I = 2,
    D = 3,
    V = 6,
    N = 7,
    B = 4,
    U = 5,
}

/// Enum representing register names
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RegisterName {
    X,
    Y,
    A,
    S,
    P,
}
