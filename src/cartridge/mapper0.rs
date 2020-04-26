use super::ines::INES;
use super::{CPUMapper, NametableMirroring, PPUMapper, PRG_RAM_SIZE};

#[cfg(test)]
use super::{CHR_PAGE_SIZE, PRG_PAGE_SIZE};

pub struct Mapper0 {
    prg_ram: [u8; PRG_RAM_SIZE],
    rom: INES,
}

impl Mapper0 {
    pub fn new(rom: INES) -> Self {
        Mapper0 {
            prg_ram: [0; 0x2000],
            rom,
        }
    }

    #[cfg(test)]
    pub fn mock() -> Self {
        Mapper0 {
            prg_ram: [0; PRG_RAM_SIZE],
            rom: INES::mock(vec![0; PRG_PAGE_SIZE], vec![0; CHR_PAGE_SIZE]),
        }
    }
}

impl CPUMapper for Mapper0 {
    fn get(&self, addr: u16) -> u8 {
        match addr {
            0x4020..=0x5FFF => 0, // stand-in garbage value
            0x6000..=0x7FFF => self.prg_ram[(addr as usize - 0x6000) % PRG_RAM_SIZE],
            0x8000..=0xBFFF => self.rom.prg[addr as usize - 0x8000],
            0xC000..=0xFFFF => self.rom.prg[(addr as usize - 0x8000) % self.rom.prg.len()],
            _ => unreachable!(),
        }
    }

    fn set(&mut self, addr: u16, val: u8) {
        match addr {
            0x6000..=0x7FFF => self.prg_ram[(addr as usize - 0x6000) % PRG_RAM_SIZE] = val,
            0x4020..=0xFFFF => {} // this mapper only provides read-only memory
            _ => unreachable!(),
        }
    }
}

impl PPUMapper for Mapper0 {
    fn get(&self, addr: u16) -> u8 {
        match addr {
            0x0..=0x1FFF => self.rom.chr[usize::from(addr)],
            _ => unreachable!(),
        }
    }

    fn set(&mut self, addr: u16, val: u8) {
        match addr {
            0x0..=0x1FFF => self.rom.chr[usize::from(addr)] = val,
            _ => unreachable!(),
        }
    }

    fn get_nametable_mirroring(&self) -> NametableMirroring {
        self.rom.get_nametable_mirroring()
    }
}
