use super::variables::Flag;
use crate::bitops::BitOps;
use crate::state::cpu::{Memory, Registers, Stack};
use std::cell::RefCell;
use std::ops::Generator;

// TODO: implement interrupt hijacking and branch instruction interrupt polling

const NMI_VECTOR: u16 = 0xFFFE;
const IRQ_VECTOR: u16 = 0xFFFA;

pub fn nmi<'a, S: Memory + Stack + Registers>(
    cpu: &'a RefCell<S>,
) -> impl Generator<Yield = (), Return = ()> + 'a {
    move || {
        yield;
        yield;
        let pc_high: u8 = cpu.borrow().get_pch();
        cpu.borrow_mut().push_stack(pc_high);
        yield;
        let pc_low: u8 = cpu.borrow().get_pcl();
        cpu.borrow_mut().push_stack(pc_low);
        yield;
        let mut p_register: u8 = cpu.borrow().get_p();
        p_register.clear_bit(Flag::B as usize);
        cpu.borrow_mut().push_stack(p_register);
        yield;
        let interrupt_low: u8 = cpu.borrow().get_mem(NMI_VECTOR);
        cpu.borrow_mut().set_pcl(interrupt_low);
        yield;
        let interrupt_high: u8 = cpu.borrow().get_mem(NMI_VECTOR + 1);
        cpu.borrow_mut().set_pch(interrupt_high);
        yield;
    }
}

pub fn irq<'a, S: Memory + Stack + Registers>(
    cpu: &'a RefCell<S>,
) -> impl Generator<Yield = (), Return = ()> + 'a {
    move || {
        yield;
        yield;
        let pc_high: u8 = cpu.borrow().get_pch();
        cpu.borrow_mut().push_stack(pc_high);
        yield;
        let pc_low: u8 = cpu.borrow().get_pcl();
        cpu.borrow_mut().push_stack(pc_low);
        yield;
        let mut p_register: u8 = cpu.borrow().get_p();
        p_register.clear_bit(Flag::B as usize);
        cpu.borrow_mut().push_stack(p_register);
        yield;
        let interrupt_low: u8 = cpu.borrow().get_mem(IRQ_VECTOR);
        cpu.borrow_mut().set_pcl(interrupt_low);
        yield;
        let interrupt_high: u8 = cpu.borrow().get_mem(IRQ_VECTOR);
        cpu.borrow_mut().set_pch(interrupt_high);
        yield;
    }
}
