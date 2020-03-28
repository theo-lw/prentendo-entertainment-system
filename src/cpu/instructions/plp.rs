use super::{Instruction, InstructionName, PullStack};
use crate::cpu::state::CPU;
use std::{cell::RefCell, rc::Rc};

pub struct PLP;

impl Instruction for PLP {
    fn name(&self) -> InstructionName {
        InstructionName::PLP
    }
}

impl PullStack for PLP {
    fn set(&self, cpu: &Rc<RefCell<CPU>>, val: u8) {
        cpu.borrow_mut().registers.p = val;
    }
}
