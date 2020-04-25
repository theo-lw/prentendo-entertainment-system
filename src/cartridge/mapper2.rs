use super::ines::INES;
use super::{CPUMapper, NametableMirroring, PPUMapper};

const BANK_SIZE: usize = 0x4000;

pub struct Mapper2 {
    total_banks: usize,
    lower_bank: usize,
    upper_bank: usize,
    rom: INES,
}

impl Mapper2 {
    pub fn new(rom: INES) -> Self {
        Mapper2 {
            total_banks: (rom.prg.len() / BANK_SIZE),
            lower_bank: 0,
            upper_bank: (rom.prg.len() / BANK_SIZE) - 1,
            rom,
        }
    }
}

impl CPUMapper for Mapper2 {
    fn get(&self, addr: u16) -> u8 {
        match addr {
            0x4020..=0x5FFF => 0, // stand-in garbage value
            0x8000..=0xBFFF => {
                self.rom.prg[self.lower_bank * BANK_SIZE + usize::from(addr - 0x8000)]
            }
            0xC000..=0xFFFF => {
                self.rom.prg[self.upper_bank * BANK_SIZE + usize::from(addr - 0xC000)]
            }
            _ => unreachable!(),
        }
    }

    fn set(&mut self, _: u16, val: u8) {
        self.lower_bank = usize::from(val) % self.total_banks;
    }
}

impl PPUMapper for Mapper2 {
    fn get(&self, addr: u16) -> u8 {
        match addr {
            0x0..=0x1FFF => self.rom.chr[addr as usize],
            _ => unreachable!(),
        }
    }

    fn set(&mut self, addr: u16, val: u8) {
        match addr {
            0x0..=0x1FFF => self.rom.chr[addr as usize] = val,
            _ => unreachable!(),
        }
    }

    fn get_nametable_mirroring(&self) -> NametableMirroring {
        self.rom.get_nametable_mirroring()
    }
}
