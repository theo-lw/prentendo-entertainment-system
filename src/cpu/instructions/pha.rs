use super::{Instruction, InstructionName, PullStack, PushStack};
use crate::cpu::state::CPU;
use std::{cell::RefCell, rc::Rc};

pub struct PHA;

impl Instruction for PHA {
    fn name(&self) -> InstructionName {
        InstructionName::PHA
    }
}

impl PushStack for PHA {
    fn get(&self, cpu: &Rc<RefCell<CPU>>) -> u8 {
        cpu.borrow().registers.a
    }
}
