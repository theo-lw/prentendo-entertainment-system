pub mod instructions;
pub mod opcode_generators;
pub mod state;
pub mod variables;

use instructions::{
    adc::ADC, and::AND, asl::ASL, bcf::BC, bit::BIT, bsf::BS, clf::CL, cpr::CP, dec::DEC, der::DE,
    eor::EOR, inc::INC, inr::IN, ldr::LD, lsr::LSR, nop::NOP, ora::ORA, phr::PH, plr::PL, rol::ROL,
    ror::ROR, sbc::SBC, sef::SE, str::ST, trr::T, Instruction,
};
use opcode_generators::{
    absolute, absolute_x, absolute_y, immediate, implied, indirect, indirect_x, indirect_y,
    relative, zero, zero_x, zero_y, CPUCycle,
};
use state::CPU;
use std::{
    cell::RefCell,
    ops::{Generator, GeneratorState},
    pin::Pin,
    rc::Rc,
};
use variables::{
    a_register::A, p_register::P, stack_pointer::S, x_register::X, y_register::Y, Flag,
};

/// This module contains CPU-related code.
/// It contains four sub-modules: `instructions`, `opcode_generators`, `state`, and 'variables'.
///
/// `state` is the simplest of them - it holds the state of the CPU, which includes code related to
/// the registers and the memory map.
///
/// To understand `opcode_generators` and `instructions`, we have to understand the anatomy of an
/// opcode. Consider the following opcode: `ADC #10`. It consists of two parts, an *instruction* and
/// an *addressing mode*. `ADC` specifies the *instruction*, which is to add the contents of a memory
/// location to the accumulator together with the carry bit. The memory location is specified by the
/// *addressing mode*. In this case an immediate addressing mode is used, so the memory location is
/// the byte after the opcode.
///
/// The code related to addressing modes can be found in `opcode_generators`.
///
/// Instruction-related code can be found under `instructions`.
///
/// If you look at the instruction set, you'll notice that there exist many instructions that do
/// the same thing but on different flags and registers. The `variables` module is an attempt to
/// decouple instructions from the data they act on. 

/// Executes a CPU cycle
pub fn cycle<'a, T: Instruction>(cpu: &'a Rc<RefCell<CPU>>) -> impl Generator + 'a {
    move || loop {
        let mut instruction = get_instruction(cpu);
        'opcode: loop {
            match instruction.as_mut().resume(()) {
                GeneratorState::Yielded(x) => {
                    yield x;
                }
                GeneratorState::Complete(x) => {
                    yield x;
                    break 'opcode;
                }
            }
        }
    }
}

/// Returns the next instruction
fn get_instruction<'a>(
    cpu: &'a Rc<RefCell<CPU>>,
) -> Pin<Box<dyn Generator<Yield = CPUCycle, Return = CPUCycle> + 'a>> {
    let opcode: u8 = cpu.borrow_mut().get_and_increment_pc();
    match opcode {
        // ADC
        0x69 => immediate::read(cpu, ADC),
        0x65 => zero::read(cpu, ADC),
        0x75 => zero_x::read(cpu, ADC),
        0x6D => absolute::read(cpu, ADC),
        0x7D => absolute_x::read(cpu, ADC),
        0x79 => absolute_y::read(cpu, ADC),
        0x61 => indirect_x::read(cpu, ADC),
        0x71 => indirect_y::read(cpu, ADC),
        // AND
        0x29 => immediate::read(cpu, AND),
        0x25 => zero::read(cpu, AND),
        0x35 => zero_x::read(cpu, AND),
        0x2D => absolute::read(cpu, AND),
        0x3D => absolute_x::read(cpu, AND),
        0x39 => absolute_y::read(cpu, AND),
        0x21 => indirect_x::read(cpu, AND),
        0x31 => indirect_y::read(cpu, AND),
        // ASL
        0x0A => implied::implied(cpu, ASL),
        0x06 => zero::modify(cpu, ASL),
        0x16 => zero_x::modify(cpu, ASL),
        0x0E => absolute::modify(cpu, ASL),
        0x1E => absolute_x::modify(cpu, ASL),
        // BCC
        0x90 => relative::relative(cpu, BC(Flag::C)),
        // BCS
        0xB0 => relative::relative(cpu, BS(Flag::C)),
        // BEQ
        0xF0 => relative::relative(cpu, BS(Flag::Z)),
        // BIT
        0x24 => zero::read(cpu, BIT),
        0x2C => absolute::read(cpu, BIT),
        // BMI
        0x30 => relative::relative(cpu, BS(Flag::N)),
        // BNE
        0xD0 => relative::relative(cpu, BC(Flag::Z)),
        // BPL
        0x10 => relative::relative(cpu, BC(Flag::N)),
        // BRK
        0x00 => implied::brk(cpu),
        // BVC
        0x50 => relative::relative(cpu, BC(Flag::V)),
        // BVS
        0x70 => relative::relative(cpu, BS(Flag::V)),
        // CLC
        0x18 => implied::implied(cpu, CL(Flag::C)),
        // CLD
        0xD8 => implied::implied(cpu, CL(Flag::D)),
        // CLI
        0x58 => implied::implied(cpu, CL(Flag::I)),
        // CLV
        0xB8 => implied::implied(cpu, CL(Flag::V)),
        // CMP
        0xC9 => immediate::read(cpu, CP(A)),
        0xC5 => zero::read(cpu, CP(A)),
        0xD5 => zero_x::read(cpu, CP(A)),
        0xCD => absolute::read(cpu, CP(A)),
        0xDD => absolute_x::read(cpu, CP(A)),
        0xD9 => absolute_y::read(cpu, CP(A)),
        0xC1 => indirect_x::read(cpu, CP(A)),
        0xD1 => indirect_y::read(cpu, CP(A)),
        // CPX
        0xE0 => immediate::read(cpu, CP(X)),
        0xE4 => zero::read(cpu, CP(X)),
        0xEC => absolute::read(cpu, CP(X)),
        // CPY
        0xC0 => immediate::read(cpu, CP(Y)),
        0xC4 => zero::read(cpu, CP(Y)),
        0xCC => absolute::read(cpu, CP(Y)),
        // DEC
        0xC6 => zero::modify(cpu, DEC),
        0xD6 => zero_x::modify(cpu, DEC),
        0xCE => absolute::modify(cpu, DEC),
        0xDE => absolute_x::modify(cpu, DEC),
        // DEX
        0xCA => implied::implied(cpu, DE(X)),
        // DEY
        0x88 => implied::implied(cpu, DE(Y)),
        // EOR
        0x49 => immediate::read(cpu, EOR),
        0x45 => zero::read(cpu, EOR),
        0x55 => zero_x::read(cpu, EOR),
        0x4D => absolute::read(cpu, EOR),
        0x5D => absolute_x::read(cpu, EOR),
        0x59 => absolute_y::read(cpu, EOR),
        0x41 => indirect_x::read(cpu, EOR),
        0x51 => indirect_y::read(cpu, EOR),
        // INC
        0xE6 => zero::modify(cpu, INC),
        0xF6 => zero_x::modify(cpu, INC),
        0xEE => absolute::modify(cpu, INC),
        0xFE => absolute_x::modify(cpu, INC),
        // INX
        0xE8 => implied::implied(cpu, IN(X)),
        // INY
        0xC8 => implied::implied(cpu, IN(Y)),
        // JMP
        0x4C => absolute::jmp(cpu),
        0x6C => indirect::jmp(cpu),
        // JSR
        0x20 => absolute::jsr(cpu),
        // LDA
        0xA9 => immediate::read(cpu, LD(A)),
        0xA5 => zero::read(cpu, LD(A)),
        0xB5 => zero_x::read(cpu, LD(A)),
        0xAD => absolute::read(cpu, LD(A)),
        0xBD => absolute_x::read(cpu, LD(A)),
        0xB9 => absolute_y::read(cpu, LD(A)),
        0xA1 => indirect_x::read(cpu, LD(A)),
        0xB1 => indirect_y::read(cpu, LD(A)),
        // LDX
        0xA2 => immediate::read(cpu, LD(X)),
        0xA6 => zero::read(cpu, LD(X)),
        0xB6 => zero_y::read(cpu, LD(X)),
        0xAE => absolute::read(cpu, LD(X)),
        0xBE => absolute_y::read(cpu, LD(X)),
        // LDY
        0xA0 => immediate::read(cpu, LD(Y)),
        0xA4 => zero::read(cpu, LD(Y)),
        0xB4 => zero_x::read(cpu, LD(Y)),
        0xAC => absolute::read(cpu, LD(Y)),
        0xBC => absolute_x::read(cpu, LD(Y)),
        // LSR
        0x4A => implied::implied(cpu, LSR),
        0x46 => zero::modify(cpu, LSR),
        0x56 => zero_x::modify(cpu, LSR),
        0x4E => absolute::modify(cpu, LSR),
        0x5E => absolute_x::modify(cpu, LSR),
        // NOP
        0xEA => implied::implied(cpu, NOP),
        // ORA
        0x09 => immediate::read(cpu, ORA),
        0x05 => zero::read(cpu, ORA),
        0x15 => zero_x::read(cpu, ORA),
        0x0D => absolute::read(cpu, ORA),
        0x1D => absolute_x::read(cpu, ORA),
        0x19 => absolute_y::read(cpu, ORA),
        0x01 => indirect_x::read(cpu, ORA),
        0x11 => indirect_y::read(cpu, ORA),
        // PHA
        0x48 => implied::push_stack(cpu, PH(A)),
        // PHP
        0x08 => implied::push_stack(cpu, PH(P)),
        // PLA
        0x68 => implied::pull_stack(cpu, PL(A)),
        // PLP
        0x28 => implied::pull_stack(cpu, PL(P)),
        // ROL
        0x2A => implied::implied(cpu, ROL),
        0x26 => zero::modify(cpu, ROL),
        0x36 => zero_x::modify(cpu, ROL),
        0x2E => absolute::modify(cpu, ROL),
        0x3E => absolute_x::modify(cpu, ROL),
        // ROR
        0x6A => implied::implied(cpu, ROR), 
        0x66 => zero::modify(cpu, ROR),
        0x76 => zero_x::modify(cpu, ROR),
        0x6E => absolute::modify(cpu, ROR),
        0x7E => absolute_x::modify(cpu, ROR),
        // RTI
        0x40 => implied::rti(cpu),
        // RTS
        0x60 => implied::rts(cpu),
        // SBC
        0xE9 => immediate::read(cpu, SBC),
        0xE5 => zero::read(cpu, SBC),
        0xF5 => zero_x::read(cpu, SBC),
        0xED => absolute::read(cpu, SBC),
        0xFD => absolute_x::read(cpu, SBC),
        0xF9 => absolute_y::read(cpu, SBC),
        0xE1 => indirect_x::read(cpu, SBC),
        0xF1 => indirect_y::read(cpu, SBC),
        // SEC
        0x38 => implied::implied(cpu, SE(Flag::C)),
        // SED
        0xF8 => implied::implied(cpu, SE(Flag::D)),
        // SEI
        0x78 => implied::implied(cpu, SE(Flag::I)),
        // STA
        0x85 => zero::write(cpu, ST(A)),
        0x95 => zero_x::write(cpu, ST(A)),
        0x8D => absolute::write(cpu, ST(A)),
        0x9D => absolute_x::write(cpu, ST(A)),
        0x99 => absolute_y::write(cpu, ST(A)),
        0x81 => indirect_x::write(cpu, ST(A)),
        0x91 => indirect_y::write(cpu, ST(A)),
        // STX
        0x86 => zero::write(cpu, ST(X)),
        0x96 => zero_y::write(cpu, ST(X)),
        0x8E => absolute::write(cpu, ST(X)),
        // STY
        0x84 => zero::write(cpu, ST(Y)),
        0x94 => zero_x::write(cpu, ST(Y)),
        0x8C => absolute::write(cpu, ST(Y)),
        // TAX
        0xAA => implied::implied(cpu, T(A, X)),
        // TAY
        0xA8 => implied::implied(cpu, T(A, Y)),
        // TSX 
        0xBA => implied::implied(cpu, T(S, X)),
        // TXA
        0x8A => implied::implied(cpu, T(X, A)),
        // TXS
        0x9A => implied::implied(cpu, T(X, S)),
        // TYA
        0x98 => implied::implied(cpu, T(Y, A)),
        // we treat unofficial opcodes (and unimplemented ones) as being NOP
        // it is not strictly correct, but it will have to do for now
        _ => implied::implied(cpu, NOP),
    }
}
