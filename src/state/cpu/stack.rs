use super::Memory;
use super::Registers;
use super::Stack;
use crate::state::NES;

const STACK_PAGE: u8 = 0x01;

impl Stack for NES {
    /// Pushes a value onto the stack
    fn push_stack(&mut self, val: u8) {
        self.set_mem(u16::from_be_bytes([STACK_PAGE, self.get_s()]), val);
        self.set_s(self.get_s().wrapping_sub(1));
    }

    /// Retrieves the top of the stack (i.e, what's pointed to by the stack pointer)
    fn top_stack(&self) -> u8 {
        self.get_mem(u16::from_be_bytes([STACK_PAGE, self.get_s()]))
    }

    /// Removes the top of the stack by decrementing the stack pointer
    fn pop_stack(&mut self) {
        self.set_s(self.get_s().wrapping_add(1));
    }
}
