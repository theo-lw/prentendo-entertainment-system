use std::ops::Generator;
use crate::state::cpu::{OAMDMA, Memory};
use std::cell::RefCell;

pub fn oamdma<'a, S: OAMDMA + Memory>(
    cpu: &'a RefCell<S>,
) -> impl Generator<Yield = (), Return = ()> + 'a {
    move || {
        yield;
        if cpu.borrow().is_odd_cycle() {
            yield;
        }
        for i in 0..=0xFF {
            let byte = cpu.borrow().get_mem(u16::from_be_bytes([cpu.borrow().get_oam_dma(), i]));
            yield;
            cpu.borrow_mut().write_oam(byte);
            yield;
        }
    }
}
