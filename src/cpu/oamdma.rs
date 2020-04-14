use crate::state::cpu::{Memory, OAMDMA};
use std::cell::RefCell;
use std::ops::Generator;

pub fn oamdma<'a, S: OAMDMA + Memory>(
    cpu: &'a RefCell<S>,
) -> impl Generator<Yield = (), Return = ()> + 'a {
    move || {
        yield;
        if cpu.borrow().is_odd_cycle() {
            yield;
        }
        for i in 0..=0xFF {
            let byte = cpu
                .borrow()
                .get_mem(u16::from_be_bytes([cpu.borrow().get_oam_dma(), i]));
            yield;
            cpu.borrow_mut().write_oam(usize::from(i), byte);
            yield;
        }
    }
}
