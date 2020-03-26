use super::Read;
use crate::cpu::state::{CPU, Flag};
use crate::address::AddressMap;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct ADC;

impl Read for ADC {
    fn execute(&self, cpu: &Rc<RefCell<CPU>>, addr: u16) {
        let byte: u8 = cpu.borrow().memory.get(addr);
        let c: u8 = cpu.borrow().registers.get_flag(Flag::C);
        let a: u8 = cpu.borrow().registers.a;
        let (result, overflow1): (u8, bool) = a.overflowing_add(byte);
        let (result, overflow2): (u8, bool) = result.overflowing_add(c);
        if result & (1 << 7) != 0 {
            cpu.borrow_mut().registers.set_flag(Flag::N);
        }
        if result == 0 {
            cpu.borrow_mut().registers.set_flag(Flag::Z);
        }
        if overflow1 || overflow2 {
            cpu.borrow_mut().registers.set_flag(Flag::C);
        }
        // if result's sign is opposite to a and byte has the same sign as a
        if (result ^ a) & !(byte ^ a) & 0b1000_000 != 0 {
            cpu.borrow_mut().registers.set_flag(Flag::V);
        }
        cpu.borrow_mut().registers.a = result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}
