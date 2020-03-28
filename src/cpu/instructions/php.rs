use super::{Instruction, InstructionName, PushStack};
use crate::cpu::state::CPU;
use std::{cell::RefCell, rc::Rc};

pub struct PHP;

impl Instruction for PHP {
    fn name(&self) -> InstructionName {
        InstructionName::PHP
    }
}

impl PushStack for PHP {
    fn get(&self, cpu: &Rc<RefCell<CPU>>) -> u8 {
        cpu.borrow().registers.p
    }
}
