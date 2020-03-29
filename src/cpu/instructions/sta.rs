use super::{Instruction, Write, InstructionName};
use crate::address::AddressMap;
use crate::cpu::state::{CPU};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct STA;

impl Instruction for STA {
    fn name(&self) -> InstructionName {
        InstructionName::STA
    }
}

impl Write for STA {
    fn execute(&self, cpu: &Rc<RefCell<CPU>>, addr: u16) {
        let a: u8 = cpu.borrow().registers.a;
        cpu.borrow_mut().memory.set(addr, a)
    }
}
