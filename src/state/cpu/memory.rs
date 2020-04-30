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
                _ => unreachable!(),
            },
            0x4015 => self.apu.get_status() | self.cpu.open_bus.get(),
            0x4016 => self.io.read() | self.cpu.open_bus.get(),
            0x4020..=0xFFFF => self.cartridge.as_cpu_mapper().get(addr),
            _ => self.cpu.open_bus.get(),
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
                _ => unreachable!(),
            },
            0x4000 => self.apu.pulse1.set_flags(val),
            0x4001 => self.apu.pulse1.set_sweep(val),
            0x4002 => self.apu.pulse1.set_timer_low(val),
            0x4003 => self.apu.pulse1.set_length(val),
            0x4004 => self.apu.pulse2.set_flags(val),
            0x4005 => self.apu.pulse2.set_sweep(val),
            0x4006 => self.apu.pulse2.set_timer_low(val),
            0x4007 => self.apu.pulse2.set_length(val),
            0x4008 => self.apu.triangle.set_linear(val),
            0x4009 => {} // unused
            0x400A => self.apu.triangle.set_timer_low(val),
            0x400B => self.apu.triangle.set_length(val),
            0x400C => self.apu.noise.set_flags(val),
            0x400D => {} // unused
            0x400E => self.apu.noise.set_period(val),
            0x400F => self.apu.noise.set_length(val),
            0x4010 => self.apu.dmc.set_flags(val),
            0x4011 => self.apu.dmc.set_output(val),
            0x4012 => self.apu.dmc.set_addr(val),
            0x4013 => self.apu.dmc.set_length(val),
            0x4014 => {
                self.cpu.oam_dma_triggered = true;
                self.cpu.oam_dma = val;
            }
            0x4015 => self.apu.set_status(val),
            0x4016 => self.io.write(val),
            0x4017 => self.apu.frame_counter.set(val),
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
