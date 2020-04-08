pub mod cpu;

/// This module holds code related to the NES's state.

pub trait CPU: cpu::Registers + cpu::Memory + cpu::Stack {
    fn get_and_increment_pc(&mut self) -> u8 {
        let result: u8 = self.get_mem(self.get_pc());
        self.increment_pc();
        result
    }
}

/// Represents the CPU's internal state
struct CPUState {
    a: u8,
    x: u8,
    y: u8,
    pc: u16,
    s: u8,
    p: u8,
    internal_ram: [u8; 0x800],
}

/// Represents the NES state
pub struct NES {
    cpu: CPUState
}

impl NES {
    #[cfg(test)]
    pub fn mock() -> Self {
        NES {
            cpu: CPUState {
                a: 0,
                x: 0,
                y: 0,
                pc: 0,
                s: 0,
                p: 0,
                internal_ram: [0; 0x800]
            }
        }
    }
}

impl CPU for NES {}

/*
impl NES {
    #[cfg(test)]
    pub fn mock() -> Self {
        NES {
            registers: Registers::mock(),
            memory: Memory::mock(),
        }
    }

    const STACK_PAGE: u8 = 0x01;

    /// Pushes a value onto the stack
    pub fn push_stack(&mut self, val: u8) {
        self.memory.set(
            u16::from_be_bytes([Self::STACK_PAGE, self.registers.s]),
            val,
        );
        self.registers.s = self.registers.s.wrapping_sub(1);
    }

    /// Retrieves the top of the stack (i.e, what's pointed to by the stack pointer)
    pub fn top_stack(&self) -> u8 {
        self.memory
            .get(u16::from_be_bytes([Self::STACK_PAGE, self.registers.s]))
    }

    /// Removes the top of the stack by decrementing the stack pointer
    pub fn pop_stack(&mut self) {
        self.registers.s = self.registers.s.wrapping_add(1);
    }

    /// Gets byte at PC and increments PC
    pub fn get_and_increment_pc(&mut self) -> u8 {
        let result: u8 = self.memory.get(self.registers.pc);
        self.registers.increment_pc();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack() {
        let mut cpu = CPU::mock();
        cpu.registers.s = 0xFF;
        cpu.push_stack(13);
        assert_eq!(cpu.memory.get(0x01FF), 13);
        assert_eq!(cpu.registers.s, 0xFE);
        cpu.pop_stack();
        assert_eq!(cpu.registers.s, 0xFF);
        assert_eq!(cpu.memory.get(0x01FF), 13);
        assert_eq!(cpu.top_stack(), 13);
    }

    #[test]
    fn test_get_and_increment_pc() {
        let mut cpu = CPU::mock();
        cpu.registers.pc = 3;
        cpu.memory.set(3, 14);
        assert_eq!(cpu.get_and_increment_pc(), 14);
        assert_eq!(cpu.registers.pc, 4);
    }
}
*/
