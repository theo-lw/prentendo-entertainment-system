use super::mapper0::Mapper0;
use super::mapper2::Mapper2;
use super::{Mapper, NametableMirroring, CHR_PAGE_SIZE, PRG_PAGE_SIZE, TRAINER_SIZE};
use crate::bitops::BitOps;
use std::io;

pub struct INES {
    pub prg: Vec<u8>,
    pub chr: Vec<u8>,
    flags6: u8,
    flags7: u8,
    _flags8: u8,
    _flags9: u8,
    _flags10: u8,
}

impl INES {
    #[cfg(test)]
    pub fn mock(prg: Vec<u8>, chr: Vec<u8>) -> Self {
        INES {
            prg,
            chr,
            flags6: 0,
            flags7: 0,
            _flags8: 0,
            _flags9: 0,
            _flags10: 0,
        }
    }

    pub fn from_file(file: &mut impl io::Read) -> Result<Self, ROMError> {
        let header: Vec<u8> = take(file, 16)?;
        if &header[0..4] != b"NES\x1A" {
            return Err(ROMError::ParseError);
        }
        let prg_pages = header[4];
        let chr_pages = header[5];
        let flags6 = header[6];
        let flags7 = header[7];
        let _flags8 = header[8];
        let _flags9 = header[9];
        let _flags10 = header[10];
        if flags6.is_bit_set(2) {
            take(file, TRAINER_SIZE)?;
        }
        let prg = take(file, prg_pages as usize * PRG_PAGE_SIZE)?;
        let chr = if chr_pages == 0 {
            vec![0; CHR_PAGE_SIZE]
        } else {
            take(file, chr_pages as usize * CHR_PAGE_SIZE)?
        };
        Ok(INES {
            prg,
            chr,
            flags6,
            flags7,
            _flags8,
            _flags9,
            _flags10,
        })
    }

    pub fn to_mapper(self) -> Box<dyn Mapper> {
        let mapper = (self.flags7 & 0b1111_0000) | (self.flags6 >> 4);
        match mapper {
            0 => Box::new(Mapper0::new(self)),
            2 => Box::new(Mapper2::new(self)),
            _ => unimplemented!(),
        }
    }

    pub fn get_nametable_mirroring(&self) -> NametableMirroring {
        match self.flags6 & 0b1001 {
            0 => NametableMirroring::Horizontal,
            1 => NametableMirroring::Vertical,
            _ => NametableMirroring::FourScreen,
        }
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
