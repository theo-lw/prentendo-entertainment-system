use super::ines::INES;
use super::{CPUMapper, NametableMirroring, PPUMapper, PRG_PAGE_SIZE, PRG_RAM_SIZE};

pub struct Mapper2 {
    total_banks: usize,
    lower_bank: usize,
    upper_bank: usize,
    prg_ram: [u8; PRG_RAM_SIZE],
    rom: INES,
}

impl Mapper2 {
    pub fn new(rom: INES) -> Self {
        Mapper2 {
            total_banks: rom.prg.len() / PRG_PAGE_SIZE,
            lower_bank: 0,
            upper_bank: (rom.prg.len() / PRG_PAGE_SIZE) - 1,
            prg_ram: [0; PRG_RAM_SIZE],
            rom,
        }
    }
}

impl CPUMapper for Mapper2 {
    fn get(&self, addr: u16) -> u8 {
        match addr {
            0x4020..=0x5FFF => 0, // stand-in garbage value
            0x6000..=0x7FFF => self.prg_ram[usize::from(addr - 0x6000)],
            0x8000..=0xBFFF => {
                self.rom.prg[self.lower_bank * PRG_PAGE_SIZE + usize::from(addr - 0x8000)]
            }
            0xC000..=0xFFFF => {
                self.rom.prg[self.upper_bank * PRG_PAGE_SIZE + usize::from(addr - 0xC000)]
            }
            _ => unreachable!(),
        }
    }

    fn set(&mut self, addr: u16, val: u8) {
        match addr {
            0x4020..=0x5FFF => {}
            0x6000..=0x7FFF => self.prg_ram[usize::from(addr - 0x6000)] = val,
            0x8000..=0xFFFF => self.lower_bank = usize::from(val) % self.total_banks,
            _ => unreachable!(),
        }
    }
}

impl PPUMapper for Mapper2 {
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
