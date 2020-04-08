mod memory;
mod registers;
mod stack;

use crate::cpu::variables::Flag;

pub trait Registers {
    fn get_a(&self) -> u8;
    fn get_x(&self) -> u8;
    fn get_y(&self) -> u8;
    fn get_pc(&self) -> u16;
    fn get_pch(&self) -> u8;
    fn get_pcl(&self) -> u8;
    fn get_s(&self) -> u8;
    fn get_p(&self) -> u8;
    fn set_a(&mut self, val: u8);
    fn set_x(&mut self, val: u8);
    fn set_y(&mut self, val: u8);
    fn set_pc(&mut self, val: u16);
    fn set_pch(&mut self, val: u8);
    fn set_pcl(&mut self, val: u8);
    fn set_s(&mut self, val: u8);
    fn set_p(&mut self, val: u8);
    fn increment_pc(&mut self);
    fn is_flag_set(&self, flag: Flag) -> bool;
    fn clear_flag(&mut self, flag: Flag);
    fn set_flag(&mut self, flag: Flag);
}

pub trait Memory {
    fn get_mem(&self, addr: u16) -> u8;
    fn set_mem(&mut self, addr: u16, val: u8);
}

pub trait Stack {
    fn push_stack(&mut self, val: u8);
    fn top_stack(&self) -> u8;
    fn pop_stack(&mut self);
}

