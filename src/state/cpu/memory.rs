use super::{Memory, Registers};
use crate::state::ppu::MappedRegisters;
use crate::state::NES;

impl Memory for NES {
    fn get_and_increment_pc(&mut self) -> u8 {
        let result: u8 = self.get_mem(self.get_pc());
        self.increment_pc();
        result
    }

    fn get_mem(&self, addr: u16) -> u8 {
        self.cpu.open_bus.set(match addr {
            0..=0x1FFF => self.cpu.internal_ram[usize::from(addr % 0x800)],
            0x2000..=0x3FFF => match (addr - 0x2000) % 8 {
                0 => self.get_ppu_ctrl(),
                1 => self.get_ppu_mask(),
                2 => self.get_ppu_status(),
                3 => self.get_oam_addr(),
                4 => self.get_oam_data(),
                5 => self.get_ppu_scroll(),
                6 => self.get_ppu_addr(),
                7 => self.get_ppu_data(),
            },
            0x4000 => self.apu.sq1_vol,
            0x4001 => self.apu.sq1_sweep,
            0x4002 => self.apu.sq1_lo,
            0x4003 => self.apu.sq1_hi,
            0x4004 => self.apu.sq2_vol,
            0x4005 => self.apu.sq2_sweep,
            0x4006 => self.apu.sq2_lo,
            0x4007 => self.apu.sq2_hi,
            0x4008 => self.apu.tri_linear,
            0x4009 => self.io.unused1,
            0x400A => self.apu.tri_lo,
            0x400B => self.apu.tri_hi,
            0x400C => self.apu.noise_vol,
            0x400D => self.io.unused2,
            0x400E => self.apu.noise_lo,
            0x400F => self.apu.noise_hi,
            0x4010 => self.apu.dmc_freq,
            0x4011 => self.apu.dmc_raw,
            0x4012 => self.apu.dmc_start,
            0x4013 => self.apu.dmc_len,
            0x4014 => self.cpu.open_bus.get(),
            0x4015 => self.apu.snd_chn,
            0x4016 => self.io.joy1,
            0x4017 => self.io.joy2,
            0x4018..=0x401F => self.cpu.open_bus.get(),
            0x4020..=0xFFFF => self.cartridge.as_cpu_mapper().get(addr),
        });
        self.cpu.open_bus.get()
    }

    fn set_mem(&mut self, addr: u16, val: u8) {
        match addr {
            0..=0x1FFF => self.cpu.internal_ram[usize::from(addr % 0x800)] = val,
            0x2000..=0x3FFF => match (addr - 0x2000) % 8 {
                0 => self.set_ppu_ctrl(val),
                1 => self.set_ppu_mask(val),
                2 => self.set_ppu_status(val),
                3 => self.set_oam_addr(val),
                4 => self.set_oam_data(val),
                5 => self.set_ppu_scroll(val),
                6 => self.set_ppu_addr(val),
                7 => self.set_ppu_data(val),
            },
            0x4000 => self.apu.sq1_vol = val,
            0x4001 => self.apu.sq1_sweep = val,
            0x4002 => self.apu.sq1_lo = val,
            0x4003 => self.apu.sq1_hi = val,
            0x4004 => self.apu.sq2_vol = val,
            0x4005 => self.apu.sq2_sweep = val,
            0x4006 => self.apu.sq2_lo = val,
            0x4007 => self.apu.sq2_hi = val,
            0x4008 => self.apu.tri_linear = val,
            0x4009 => self.io.unused1 = val,
            0x400A => self.apu.tri_lo = val,
            0x400B => self.apu.tri_hi = val,
            0x400C => self.apu.noise_vol = val,
            0x400D => self.io.unused2 = val,
            0x400E => self.apu.noise_lo = val,
            0x400F => self.apu.noise_hi = val,
            0x4010 => self.apu.dmc_freq = val,
            0x4011 => self.apu.dmc_raw = val,
            0x4012 => self.apu.dmc_start = val,
            0x4013 => self.apu.dmc_len = val,
            0x4014 => {
                self.cpu.oam_dma_triggered = true;
                self.cpu.oam_dma = val;
            }
            0x4015 => self.apu.snd_chn = val,
            0x4016 => self.io.joy1 = val,
            0x4017 => self.io.joy2 = val,
            0x4018..=0x401F => {} // this functionality is normally disabled
            0x4020..=0xFFFF => self.cartridge.as_cpu_mapper_mut().set(addr, val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_get_memory() {
        let mut cpu = NES::mock();
        cpu.set_mem(0x0304, 12);
        assert_eq!(cpu.get_mem(0x0304), 12);
        assert_eq!(cpu.get_mem(0xB04), 12);
        cpu.set_mem(0x2033, 5);
        assert_eq!(cpu.get_mem(0x2033), 5);
        assert_eq!(cpu.get_mem(0x2003), 5);
        cpu.set_mem(0x4005, 8);
        assert_eq!(cpu.get_mem(0x4005), 8);
    }

    #[test]
    fn test_get_and_increment_pc() {
        let mut cpu = NES::mock();
        cpu.set_pc(4);
        cpu.set_mem(4, 19);
        assert_eq!(cpu.get_and_increment_pc(), 19);
        assert_eq!(cpu.get_pc(), 5);
    }
}
