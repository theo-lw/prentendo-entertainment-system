use crate::{
    address::AddressMap,
    cpu::{
        instructions::{Read, ReadModifyWrite, Write},
        state::CPU,
    },
};
use std::{cell::RefCell, ops::Generator, pin::Pin, rc::Rc};

pub fn generate_read<'a>(
    cpu: &'a Rc<RefCell<CPU>>,
    instruction: impl Read + 'a,
) -> impl Generator + 'a {
    move || {
        let debug: String = format!("Absolute X Read {:?}", instruction);
        yield debug.clone();
        let low_byte: u8 = cpu.borrow().memory.get(cpu.borrow().registers.pc);
        cpu.borrow_mut().registers.pc += 1;
        yield debug.clone();
        let mut high_byte: u8 = cpu.borrow().memory.get(cpu.borrow().registers.pc);
        let (low_byte, overflow): (u8, bool) = low_byte.overflowing_add(cpu.borrow().registers.x);
        cpu.borrow_mut().registers.pc += 1;
        yield debug.clone();
        if overflow {
            high_byte = high_byte.wrapping_add(1);
            yield debug.clone();
        } 
        instruction.execute(cpu, u16::from_be_bytes([high_byte, low_byte]));
        return;
    }
}

pub fn generate_write<'a>(
    cpu: &'a Rc<RefCell<CPU>>,
    instruction: impl Write + 'a,
) -> impl Generator + 'a {
    move || {
        let debug: String = format!("Absolute X Write {:?}", instruction);
        yield debug.clone();
        let low_byte: u8 = cpu.borrow().memory.get(cpu.borrow().registers.pc);
        cpu.borrow_mut().registers.pc += 1;
        yield debug.clone();
        let mut high_byte: u8 = cpu.borrow().memory.get(cpu.borrow().registers.pc);
        let (low_byte, overflow): (u8, bool) = low_byte.overflowing_add(cpu.borrow().registers.x);
        cpu.borrow_mut().registers.pc += 1;
        yield debug.clone();
        if overflow {
            high_byte = high_byte.wrapping_add(1);
        }
        yield debug.clone();
        instruction.execute(cpu, u16::from_be_bytes([high_byte, low_byte]));
        return;
    }
}

pub fn generate_read_modify_write<'a>(
    cpu: &'a Rc<RefCell<CPU>>,
    instruction: impl ReadModifyWrite + 'a,
) -> impl Generator + 'a {
    move || {
        let debug: String = format!("Absolute X Read Modify Write {:?}", instruction);
        yield debug.clone();
        let low_byte: u8 = cpu.borrow().memory.get(cpu.borrow().registers.pc);
        cpu.borrow_mut().registers.pc += 1;
        yield debug.clone();
        let mut high_byte: u8 = cpu.borrow().memory.get(cpu.borrow().registers.pc);
        let (low_byte, overflow): (u8, bool) = low_byte.overflowing_add(cpu.borrow().registers.x);
        cpu.borrow_mut().registers.pc += 1;
        yield debug.clone();
        if overflow {
            high_byte = high_byte.wrapping_add(1);
        }
        yield debug.clone();
        let addr = u16::from_be_bytes([high_byte, low_byte]);
        let val: u8 = cpu.borrow().memory.get(addr);
        yield debug.clone();
        cpu.borrow_mut().memory.set(addr, val);
        yield debug.clone(); 
        instruction.execute(cpu, addr, val);
        return;
    }
}
