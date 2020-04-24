use crate::state::ppu::{Background, Memory};
use std::cell::RefCell;
use std::ops::Generator;

pub fn evaluate_background<'a, T: Background + Memory>(
    ppu: &'a RefCell<T>,
) -> impl Generator<Yield = (), Return = BackgroundTile> + 'a {
    move || {
        let nametable_addr: u16 = ppu.borrow().get_nametable_addr();
        yield;
        let nametable: u8 = ppu.borrow().get(nametable_addr);
        yield;
        let attribute_addr: u16 = ppu.borrow().get_attribute_addr();
        yield;
        let attribute: u8 =
            (ppu.borrow().get(attribute_addr) >> ppu.borrow().get_attribute_shift()) & 0b11;
        yield;
        let tile_addr_low: u16 = ppu.borrow().get_background_tile_addr_low(nametable);
        yield;
        let pattern_low: u8 = ppu.borrow().get(tile_addr_low);
        yield;
        let tile_addr_high: u16 = ppu.borrow().get_background_tile_addr_high(nametable);
        yield;
        let pattern_high: u8 = ppu.borrow().get(tile_addr_high);
        BackgroundTile {
            attribute,
            pattern_high,
            pattern_low,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BackgroundTile {
    pub attribute: u8,
    pub pattern_high: u8,
    pub pattern_low: u8,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::NES;

    #[test]
    fn test_evaluate_background_cycles() {
        let nes = RefCell::new(NES::mock());
        let mut background_generator = evaluate_background(&nes);
    }
}
