pub mod apu;
pub mod cpu;
pub mod io;
pub mod ppu;

use crate::cartridge::Mapper;
use apu::APUState;
use cpu::CPUState;
use io::IOState;
use ppu::PPUState;

#[cfg(test)]
use crate::cartridge::mapper0::Mapper0;

/// This module holds the code that encapsulates the NES's state
/// Components of the NES are able to mutate each other's state - for instance, the CPU can write
/// to the PPU's memory-mapped registers. This poses a challenge because shared mutable state is
/// difficult to represent in Rust. I had initially planned to wrap Rc<RefCell> around the shared
/// pieces, but this is complicated and adds unnecessary overhead. Furthermore, it's not strictly
/// correct to use Rc for purposes other than shared ownership, and there is no shared ownership in
/// the NES. The CPU owns its own state, as does the PPU and the APU - I just want them to
/// communicate with each other in some way.
///
/// I ended up choosing a simpler route, which was to simply create a struct holding *all* of the
/// NES's internal state. Although this design makes it easy for one component to mutate another
/// component's state, it makes encapsulation harder. CPU instructions only be aware of the CPU and
/// not all the other components in the NES. To make encapsulation possible, I created traits that
/// represent components of the NES. For instance, functions acting on the CPU should take as an
/// argument the `CPU` trait and not the `NES` struct.

/// Trait representing the CPU
pub trait CPU: cpu::Registers + cpu::Memory + cpu::Stack + cpu::OAMDMA + cpu::Interrupt {}
impl CPU for NES {}

/// Trait representing the PPU
pub trait PPU: ppu::Memory + ppu::Cycle + ppu::VBlank + ppu::Sprites + ppu::Background {}
impl PPU for NES {}

/// The struct holding all of the NES's internal state.
pub struct NES {
    cpu: CPUState,
    ppu: PPUState,
    apu: APUState,
    io: IOState,
    cartridge: Box<dyn Mapper>,
}

impl NES {
    #[cfg(test)]
    pub fn mock() -> Self {
        NES {
            cpu: CPUState::mock(),
            ppu: PPUState::mock(),
            apu: APUState::mock(),
            io: IOState::mock(),
            cartridge: Box::new(Mapper0::mock()),
        }
    }

    pub fn new(cartridge: Box<dyn Mapper>) -> Self {
        NES {
            cpu: CPUState::new(),
            ppu: PPUState::new(),
            apu: APUState::new(),
            io: IOState::new(),
            cartridge,
        }
    }
}
