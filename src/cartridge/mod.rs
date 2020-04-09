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
    fn as_cpu_mapper(&self) -> &dyn CPUMapper;
    fn as_ppu_mapper(&self) -> &dyn PPUMapper;
    fn as_cpu_mapper_mut(&mut self) -> &mut dyn CPUMapper;
    fn as_ppu_mapper_mut(&mut self) -> &mut dyn PPUMapper;
}

impl<T: CPUMapper + PPUMapper> Mapper for T {
    fn as_cpu_mapper(&self) -> &dyn CPUMapper {
        self
    }
    fn as_ppu_mapper(&self) -> &dyn PPUMapper {
        self
    }
    fn as_cpu_mapper_mut(&mut self) -> &mut dyn CPUMapper {
        self
    }
    fn as_ppu_mapper_mut(&mut self) -> &mut dyn PPUMapper {
        self
    }
}
