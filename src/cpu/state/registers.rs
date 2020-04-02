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
        self.p |= 0b1 << (flag as u8);
    }

    /// Clears the given flag
    pub fn clear_flag(&mut self, flag: Flag) {
        self.p &= !(0b1 << (flag as u8));
    }

    /// Returns the value of the flag (1 if it is on, 0 otherwise)
    pub fn get_flag(&self, flag: Flag) -> u8 {
        (self.p & (1 << (flag as u8))) >> (flag as u8)
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
            p: 0b0010_0000,
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
    fn test_get_flag() {
        let mut registers = Registers::mock();
        registers.p = 0b0010_0000;
        assert_eq!(registers.get_flag(Flag::C), 0);
        registers.p = 0b0010_0001;
        assert_eq!(registers.get_flag(Flag::C), 1);
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
