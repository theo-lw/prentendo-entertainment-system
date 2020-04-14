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
        // this bit is always set
        self.cpu.p.set_bit(Flag::U as usize);
        // this bit doesn't actually exist in the NES, so we let it be zero
        self.cpu.p.clear_bit(Flag::B as usize);
    }
    fn increment_pc(&mut self) {
        self.set_pc(self.get_pc().wrapping_add(1));
    }
    fn is_flag_set(&self, flag: Flag) -> bool {
        self.cpu.p.is_bit_set(flag as usize)
    }
    fn assign_flag(&mut self, flag: Flag, val: bool) {
        self.cpu.p.assign_bit(flag as usize, val);
        // this bit is always set
        self.cpu.p.set_bit(Flag::U as usize);
        // this bit doesn't actually exist in the NES, so we let it be zero
        self.cpu.p.clear_bit(Flag::B as usize);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pch() {
        let mut cpu = NES::mock();
        cpu.set_pc(0);
        cpu.set_pch(0xFF);
        assert_eq!(cpu.get_pch(), 0xFF);
        assert_eq!(cpu.get_pc(), 0xFF00);
    }

    #[test]
    fn test_pcl() {
        let mut cpu = NES::mock();
        cpu.set_pc(0);
        cpu.set_pcl(0xFF);
        assert_eq!(cpu.get_pcl(), 0xFF);
        assert_eq!(cpu.get_pc(), 0x00FF);
    }

    #[test]
    fn test_pc() {
        let mut cpu = NES::mock();
        cpu.set_pc(0x10FA);
        assert_eq!(cpu.get_pc(), 0x10FA);
        cpu.increment_pc();
        assert_eq!(cpu.get_pc(), 0x10FB);
    }

    #[test]
    fn test_a() {
        let mut cpu = NES::mock();
        cpu.set_a(0xAB);
        assert_eq!(cpu.get_a(), 0xAB);
    }

    #[test]
    fn test_x() {
        let mut cpu = NES::mock();
        cpu.set_x(0xAB);
        assert_eq!(cpu.get_x(), 0xAB);
    }

    #[test]
    fn test_y() {
        let mut cpu = NES::mock();
        cpu.set_y(0xAB);
        assert_eq!(cpu.get_y(), 0xAB);
    }

    #[test]
    fn test_s() {
        let mut cpu = NES::mock();
        cpu.set_s(0xAB);
        assert_eq!(cpu.get_s(), 0xAB);
    }

    #[test]
    fn test_p() {
        let mut cpu = NES::mock();
        cpu.set_p(0b0101_1001);
        assert_eq!(cpu.get_p(), 0b0110_1001);
    }

    #[test]
    fn test_flag() {
        let mut cpu = NES::mock();
        cpu.assign_flag(Flag::V, false);
        assert!(!cpu.is_flag_set(Flag::V));
        cpu.assign_flag(Flag::V, true);
        assert!(cpu.is_flag_set(Flag::V));
        cpu.assign_flag(Flag::B, true);
        assert!(!cpu.is_flag_set(Flag::B));
        cpu.assign_flag(Flag::U, false);
        assert!(cpu.is_flag_set(Flag::U));
    }
}
