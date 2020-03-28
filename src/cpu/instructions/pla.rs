use super::{Instruction, InstructionName, PullStack};
use crate::cpu::state::CPU;
use std::{cell::RefCell, rc::Rc};

pub struct PLA;

impl Instruction for PLA {
    fn name(&self) -> InstructionName {
        InstructionName::PLA
    }
}

impl PullStack for PLA {
    fn set(&self, cpu: &Rc<RefCell<CPU>>, val: u8) {
        cpu.borrow_mut().registers.a = val;
    }
}
