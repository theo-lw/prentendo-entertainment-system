use super::{CPUMapper, Mapper, PPUMapper};

pub struct Mapper0 {
    prg: Vec<u8>,
    chr: Vec<u8>,
}

impl Mapper0 {
    pub fn new(prg: Vec<u8>, chr: Vec<u8>) -> Self {
        Mapper0 { prg, chr }
    }

    #[cfg(test)]
    pub fn mock() -> Self {
        Mapper0 {
            prg: vec![0; 0x4000],
            chr: vec![0; 0x2000],
        }
    }
}

impl CPUMapper for Mapper0 {
    fn get(&self, addr: u16) -> u8 {
        match addr {
            0x4020..=0x7FFF => 0, // stand-in garbage value
            0x8000..=0xBFFF => self.prg[addr as usize - 0x8000],
            0xC000..=0xFFFF => self.prg[(addr as usize - 0x8000) % self.prg.len()],
            _ => panic!("Address outside of mapper range!"),
        }
    }

    fn set(&mut self, addr: u16, _: u8) {
        match addr {
            0x4020..=0xFFFF => {} // this mapper only provides read-only memory
            _ => panic!("Address outside of mapper range!"),
        }
    }
}

impl PPUMapper for Mapper0 {
    fn get(&self, addr: u16) -> u8 {
        match addr {
            _ => unimplemented!(),
        }
    }

    fn set(&mut self, addr: u16, _: u8) {
        match addr {
            _ => unimplemented!(),
        }
    }
}
