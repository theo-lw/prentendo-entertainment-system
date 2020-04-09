use super::Registers;
use crate::bitops::BitOps;
use crate::cpu::variables::Flag;
use crate::state::NES;

impl Registers for NES {
    fn get_a(&self) -> u8 {
        self.cpu.a
    }
    fn get_x(&self) -> u8 {
        self.cpu.x
    }
    fn get_y(&self) -> u8 {
        self.cpu.y
    }
    fn get_pc(&self) -> u16 {
        self.cpu.pc
    }
    fn get_pch(&self) -> u8 {
        self.cpu.pc.to_be_bytes()[0]
    }
    fn get_pcl(&self) -> u8 {
        self.cpu.pc.to_be_bytes()[1]
    }
    fn get_s(&self) -> u8 {
        self.cpu.s
    }
    fn get_p(&self) -> u8 {
        self.cpu.p
    }
    fn set_a(&mut self, val: u8) {
        self.cpu.a = val;
    }
    fn set_x(&mut self, val: u8) {
        self.cpu.x = val;
    }
    fn set_y(&mut self, val: u8) {
        self.cpu.y = val;
    }
    fn set_pch(&mut self, val: u8) {
        let [_, pcl] = self.cpu.pc.to_be_bytes();
        self.cpu.pc = u16::from_be_bytes([val, pcl]);
    }
    fn set_pcl(&mut self, val: u8) {
        let [pch, _] = self.cpu.pc.to_be_bytes();
        self.cpu.pc = u16::from_be_bytes([pch, val]);
    }
    fn set_pc(&mut self, val: u16) {
        self.cpu.pc = val;
    }
    fn set_s(&mut self, val: u8) {
        self.cpu.s = val;
    }
    fn set_p(&mut self, val: u8) {
        self.cpu.p = val;
        self.set_flag(Flag::U); // this bit is always set
        // this bit doesn't actually exist in the NES, so we let it be zero
        self.clear_flag(Flag::B);
    }
    fn increment_pc(&mut self) {
        self.set_pc(self.get_pc().wrapping_add(1));
    }

    /// Sets the given flag
    fn set_flag(&mut self, flag: Flag) {
        self.cpu.p.set_bit(flag as usize);
    }

    /// Clears the given flag
    fn clear_flag(&mut self, flag: Flag) {
        self.cpu.p.clear_bit(flag as usize);
    }

    /// Returns whether the flag is set or not
    fn is_flag_set(&self, flag: Flag) -> bool {
        self.cpu.p.is_bit_set(flag as usize)
    }

    /// Sets the flag if true, clears it otherwise
    fn assign_flag(&mut self, flag: Flag, val: bool) {
        if val {
            self.set_flag(flag);
        } else {
            self.clear_flag(flag);
        }
    }
}

/*
use crate::bitops::BitOps;
use crate::cpu::variables::Flag;

/// Represents the registers of the NES CPU
#[derive(Debug)]
pub struct Registers {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub pc: u16,
    pub s: u8,
    pub p: u8,
}

impl Registers {
    #[cfg(test)]
    pub fn mock() -> Self {
        Default::default()
    }

    /// Sets the high byte of the PC
    pub fn set_pch(&mut self, high: u8) {
        let [_, low]: [u8; 2] = self.pc.to_be_bytes();
        self.pc = u16::from_be_bytes([high, low]);
    }

    /// Sets the lower byte of the PC
    pub fn set_pcl(&mut self, low: u8) {
        let [high, _]: [u8; 2] = self.pc.to_be_bytes();
        self.pc = u16::from_be_bytes([high, low]);
    }

    /// Gets the high byte of the PC
    pub fn get_pch(&self) -> u8 {
        let [high, _]: [u8; 2] = self.pc.to_be_bytes();
        high
    }

    /// Gets the low byte of the PC
    pub fn get_pcl(&self) -> u8 {
        let [_, low]: [u8; 2] = self.pc.to_be_bytes();
        low
    }

    /// Increments the PC
    pub fn increment_pc(&mut self) {
        self.pc = self.pc.wrapping_add(1);
    }

    /// Sets the given flag
    pub fn set_flag(&mut self, flag: Flag) {
        self.p.set_bit(flag as usize);
    }

    /// Clears the given flag
    pub fn clear_flag(&mut self, flag: Flag) {
        self.p.clear_bit(flag as usize);
    }

    /// Returns whether the flag is set or not
    pub fn is_flag_set(&self, flag: Flag) -> bool {
        self.p.is_bit_set(flag as usize)
    }
}

impl Default for Registers {
    fn default() -> Self {
        Registers {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            s: 0,
            p: 0b0011_0000,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_flag() {
        let mut registers = Registers::mock();
        registers.p = 0b0010_0000;
        registers.set_flag(Flag::B);
        assert_eq!(registers.p, 0b0011_0000);
        registers.set_flag(Flag::B);
        assert_eq!(registers.p, 0b0011_0000);
    }

    #[test]
    fn test_is_flag_set() {
        let mut registers = Registers::mock();
        registers.p = 0b0010_0000;
        assert_eq!(registers.is_flag_set(Flag::C), false);
        registers.p = 0b0010_0001;
        assert_eq!(registers.is_flag_set(Flag::C), true);
    }

    #[test]
    fn test_clear_flag() {
        let mut registers = Registers::mock();
        registers.p = 0b1010_0000;
        registers.clear_flag(Flag::N);
        assert_eq!(registers.p, 0b0010_0000);
        registers.clear_flag(Flag::N);
        assert_eq!(registers.p, 0b0010_0000);
    }

    #[test]
    fn test_set_pch() {
        let mut registers = Registers::mock();
        registers.pc = 0x4030;
        registers.set_pch(0x31);
        assert_eq!(registers.pc, 0x3130);
    }

    #[test]
    fn test_set_pcl() {
        let mut registers = Registers::mock();
        registers.pc = 0x4030;
        registers.set_pcl(0x41);
        assert_eq!(registers.pc, 0x4041);
    }

    #[test]
    fn test_get_pch() {
        let mut registers = Registers::mock();
        registers.pc = 0x4030;
        assert_eq!(registers.get_pch(), 0x40);
    }

    #[test]
    fn test_get_pcl() {
        let mut registers = Registers::mock();
        registers.pc = 0x4030;
        assert_eq!(registers.get_pcl(), 0x30);
    }

    #[test]
    fn test_increment_pc() {
        let mut registers = Registers::mock();
        registers.pc = 0x4030;
        registers.increment_pc();
        assert_eq!(registers.pc, 0x4031);
    }
}
*/
