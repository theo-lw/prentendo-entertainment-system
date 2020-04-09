use super::mapper0::Mapper0;
use super::Mapper;
use crate::bitops::BitOps;
use std::io;

const PRG_PAGE_SIZE: usize = 0x4000;
const CHR_PAGE_SIZE: usize = 0x2000;
const TRAINER_SIZE: usize = 0x200;

pub struct INES {
    prg: Vec<u8>,
    chr: Vec<u8>,
    flags6: u8,
    flags7: u8,
    flags8: u8,
    flags9: u8,
    flags10: u8,
}

impl INES {
    pub fn from_file(file: &mut impl io::Read) -> Result<Self, ROMError> {
        let header: Vec<u8> = take(file, 16)?;
        if &header[0..4] != b"NES\x1A" {
            return Err(ROMError::ParseError);
        }
        let prg_pages = header[4];
        let chr_pages = header[5];
        let flags6 = header[6];
        let flags7 = header[7];
        let flags8 = header[8];
        let flags9 = header[9];
        let flags10 = header[10];
        if flags6.is_bit_set(2) {
            take(file, TRAINER_SIZE)?;
        }
        let prg = take(file, prg_pages as usize * PRG_PAGE_SIZE)?;
        let chr = take(file, chr_pages as usize * CHR_PAGE_SIZE)?;
        Ok(INES {
            prg,
            chr,
            flags6,
            flags7,
            flags8,
            flags9,
            flags10,
        })
    }

    pub fn to_mapper(self) -> Box<dyn Mapper> {
        Box::new(Mapper0::new(self.prg, self.chr))
    }
}

fn take(file: &mut impl io::Read, length: usize) -> Result<Vec<u8>, ROMError> {
    let mut result = vec![0; length];
    file.read_exact(&mut result)?;
    Ok(result)
}

#[derive(Debug)]
pub enum ROMError {
    ParseError,
    IOError(io::Error),
}

impl From<io::Error> for ROMError {
    fn from(err: io::Error) -> ROMError {
        ROMError::IOError(err)
    }
}
