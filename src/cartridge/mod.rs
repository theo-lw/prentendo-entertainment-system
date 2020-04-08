pub mod ines;
pub mod mapper0;

/// The mapper visible to the CPU
/// `get` and `set` should take addresses in the range of 0x4020 - 0xFFFF
pub trait CPUMapper {
    fn get(&self, addr: u16) -> u8;
    fn set(&mut self, addr: u16, val: u8);
}

/// The mapper visible to the PPU
pub trait PPUMapper {
    fn get(&self, addr: u16) -> u8;
    fn set(&mut self, addr: u16, val: u8);
}

/// Trait representing a mapper
pub trait Mapper: CPUMapper + PPUMapper {
    fn as_cpu(&self) -> &dyn CPUMapper;
    fn as_ppu(&self) -> &dyn PPUMapper;
}

impl<T: CPUMapper + PPUMapper> Mapper for T {
    fn as_cpu(&self) -> &dyn CPUMapper {
        self
    }

    fn as_ppu(&self) -> &dyn PPUMapper {
        self
    }
}
