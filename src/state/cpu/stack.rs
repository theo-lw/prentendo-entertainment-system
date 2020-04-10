use super::Memory;
use super::Registers;
use super::Stack;
use crate::state::NES;

const STACK_PAGE: u8 = 0x01;

impl Stack for NES {
    fn push_stack(&mut self, val: u8) {
        self.set_mem(u16::from_be_bytes([STACK_PAGE, self.get_s()]), val);
        self.set_s(self.get_s().wrapping_sub(1));
    }
    fn top_stack(&self) -> u8 {
        self.get_mem(u16::from_be_bytes([STACK_PAGE, self.get_s()]))
    }
    fn pop_stack(&mut self) {
        self.set_s(self.get_s().wrapping_add(1));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack() {
        let mut cpu = NES::mock();
        cpu.set_s(0xFF);
        cpu.push_stack(13);
        assert_eq!(cpu.get_mem(0x01FF), 13);
        assert_eq!(cpu.get_s(), 0xFE);
        cpu.pop_stack();
        assert_eq!(cpu.get_s(), 0xFF);
        assert_eq!(cpu.get_mem(0x01FF), 13);
        assert_eq!(cpu.top_stack(), 13);
    }
}
