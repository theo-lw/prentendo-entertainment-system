use super::Memory;
use crate::cartridge::NametableMirroring;
use crate::state::NES;

impl Memory for NES {
    fn get(&self, addr: u16) -> u8 {
        match addr {
            0..=0x1FFF => self.cartridge.as_ppu_mapper().get(addr),
            0x2000..=0x3EFF => {
                let reduced_addr = 0x2000 + (addr - 0x2000) % 0x1000;
                match reduced_addr {
                    0x2000..=0x23FF => self.ppu.ram.nametable_a[usize::from(reduced_addr - 0x2000)],
                    0x2400..=0x27FF => match self.cartridge.get_nametable_mirroring() {
                        NametableMirroring::Horizontal => {
                            self.ppu.ram.nametable_a[usize::from(reduced_addr - 0x400)]
                        }
                        NametableMirroring::Vertical => {
                            self.ppu.ram.nametable_b[usize::from(reduced_addr - 0x400)]
                        }
                        NametableMirroring::FourScreen => {
                            self.ppu.ram.nametable_b[usize::from(reduced_addr - 0x400)]
                        }
                    },
                    0x2800..=0x2BFF => match self.cartridge.get_nametable_mirroring() {
                        NametableMirroring::Horizontal => {
                            self.ppu.ram.nametable_b[usize::from(reduced_addr - 0x2800)]
                        }
                        NametableMirroring::Vertical => {
                            self.ppu.ram.nametable_a[usize::from(reduced_addr - 0x2800)]
                        }
                        NametableMirroring::FourScreen => {
                            self.cartridge.as_ppu_mapper().get(reduced_addr)
                        }
                    },
                    0x2C00..=0x2FFF => match self.cartridge.get_nametable_mirroring() {
                        NametableMirroring::Horizontal => {
                            self.ppu.ram.nametable_b[usize::from(reduced_addr - 0x2C00)]
                        }
                        NametableMirroring::Vertical => {
                            self.ppu.ram.nametable_b[usize::from(reduced_addr - 0x2C00)]
                        }
                        NametableMirroring::FourScreen => {
                            self.cartridge.as_ppu_mapper().get(reduced_addr)
                        }
                    },
                    _ => unreachable!(),
                }
            }
            0x3F00..=0x3FFF => {
                let mut reduced_addr = usize::from(addr - 0x3F00) % 0x20;
                if reduced_addr == 0x10
                    || reduced_addr == 0x14
                    || reduced_addr == 0x18
                    || reduced_addr == 0x1C
                {
                    reduced_addr -= 0x10;
                }
                self.ppu.ram.palatte_ram[reduced_addr]
            }
            // anything outside the given range is unreachable because the internal vram address
            // only goes up to 12 bits
            _ => unreachable!(),
        }
    }

    fn set(&mut self, addr: u16, val: u8) {
        match addr {
            0..=0x1FFF => self.cartridge.as_ppu_mapper_mut().set(addr, val),
            0x2000..=0x3EFF => {
                let reduced_addr = 0x2000 + (addr - 0x2000) % 0x1000;
                match reduced_addr {
                    0x2000..=0x23FF => {
                        self.ppu.ram.nametable_a[usize::from(reduced_addr - 0x2000)] = val
                    }
                    0x2400..=0x27FF => match self.cartridge.get_nametable_mirroring() {
                        NametableMirroring::Horizontal => {
                            self.ppu.ram.nametable_a[usize::from(reduced_addr - 0x400)] = val
                        }
                        NametableMirroring::Vertical => {
                            self.ppu.ram.nametable_b[usize::from(reduced_addr - 0x400)] = val
                        }
                        NametableMirroring::FourScreen => {
                            self.ppu.ram.nametable_b[usize::from(reduced_addr - 0x400)] = val
                        }
                    },
                    0x2800..=0x2BFF => match self.cartridge.get_nametable_mirroring() {
                        NametableMirroring::Horizontal => {
                            self.ppu.ram.nametable_b[usize::from(reduced_addr - 0x2800)] = val
                        }
                        NametableMirroring::Vertical => {
                            self.ppu.ram.nametable_a[usize::from(reduced_addr - 0x2800)] = val
                        }
                        NametableMirroring::FourScreen => {
                            self.cartridge.as_ppu_mapper_mut().set(reduced_addr, val)
                        }
                    },
                    0x2C00..=0x2FFF => match self.cartridge.get_nametable_mirroring() {
                        NametableMirroring::Horizontal => {
                            self.ppu.ram.nametable_b[usize::from(reduced_addr - 0x2C00)] = val
                        }
                        NametableMirroring::Vertical => {
                            self.ppu.ram.nametable_b[usize::from(reduced_addr - 0x2C00)] = val
                        }
                        NametableMirroring::FourScreen => {
                            self.cartridge.as_ppu_mapper_mut().set(reduced_addr, val)
                        }
                    },
                    _ => unreachable!(),
                }
            }
            // anything outside the given range is unreachable because the internal vram address
            // only goes up to 12 bits
            0x3F00..=0x3FFF => {
                let mut reduced_addr = usize::from(addr - 0x3F00) % 0x20;
                if reduced_addr == 0x10
                    || reduced_addr == 0x14
                    || reduced_addr == 0x18
                    || reduced_addr == 0x1C
                {
                    reduced_addr -= 0x10;
                }
                self.ppu.ram.palatte_ram[reduced_addr] = val;
            }
            _ => unreachable!(),
        }
    }
}
