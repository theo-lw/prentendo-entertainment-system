use super::ines::INES;
use super::{CPUMapper, NametableMirroring, PPUMapper};

pub struct Mapper0 {
    rom: INES,
}

impl Mapper0 {
    pub fn new(rom: INES) -> Self {
        Mapper0 { rom }
    }

    #[cfg(test)]
    pub fn mock() -> Self {
        Mapper0 {
            rom: INES::mock(vec![0; 0x4000], vec![0; 0x2000]),
        }
    }
}

impl CPUMapper for Mapper0 {
    fn get(&self, addr: u16) -> u8 {
        match addr {
            0x4020..=0x7FFF => 0, // stand-in garbage value
            0x8000..=0xBFFF => self.rom.prg[addr as usize - 0x8000],
            0xC000..=0xFFFF => self.rom.prg[(addr as usize - 0x8000) % self.rom.prg.len()],
            _ => unreachable!(),
        }
    }

    fn set(&mut self, addr: u16, _: u8) {
        match addr {
            0x4020..=0xFFFF => {} // this mapper only provides read-only memory
            _ => unreachable!(),
        }
    }
}

impl PPUMapper for Mapper0 {
    fn get(&self, addr: u16) -> u8 {
        match addr {
            0x0..=0x1FFF => self.rom.chr[addr as usize],
            _ => unreachable!(),
        }
    }

    fn set(&mut self, addr: u16, val: u8) {
        match addr {
            0x0..=0x1FFF => self.rom.chr[addr as usize] = val,
            _ => unimplemented!(),
        }
    }

    fn get_nametable_mirroring(&self) -> NametableMirroring {
        self.rom.get_nametable_mirroring()
    }
}
