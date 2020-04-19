use crate::state::ppu::oam::OAM;
use crate::state::ppu::{Cycle, Memory, Sprites};
use std::cell::RefCell;
use std::ops::Generator;

const SECONDARY_OAM_SIZE: usize = 32;
const SECONDARY_OAM_SPRITE_COUNT: usize = 8;

pub fn evaluate_sprites<'a, T: Cycle + Sprites + Memory>(
    ppu: &'a RefCell<T>,
) -> impl Generator<Yield = (), Return = Vec<Sprite>> + 'a {
    move || {
        let mut secondary_oam: [u8; SECONDARY_OAM_SIZE] = [0xFF; SECONDARY_OAM_SIZE];
        let mut cycle_count: u32 = 0;

        // stall for the first 64 cycles
        for _ in 0..64 {
            yield;
            cycle_count += 1;
        }
        let mut secondary_oam_index: usize = 0;
        let mut sprite_index: u8 = 0;

        // read at most 8 sprites into secondary OAM
        while (0..OAM::SPRITE_COUNT).contains(&sprite_index)
            && (0..SECONDARY_OAM_SIZE).contains(&secondary_oam_index)
        {
            let y_coordinate: u8 = ppu.borrow().read_oam(sprite_index, 0);
            yield;
            let sprite_range = usize::from(y_coordinate)
                ..(usize::from(y_coordinate) + usize::from(ppu.borrow().get_sprite_height()));
            secondary_oam[secondary_oam_index] = y_coordinate;
            yield;
            cycle_count += 2;
            if sprite_range.contains(&ppu.borrow().get_scanline()) {
                let tile_index: u8 = ppu.borrow().read_oam(sprite_index, 1);
                yield;
                secondary_oam[secondary_oam_index + 1] = tile_index;
                yield;
                let attributes: u8 = ppu.borrow().read_oam(sprite_index, 2);
                yield;
                secondary_oam[secondary_oam_index + 2] = attributes;
                yield;
                let x_coordinate: u8 = ppu.borrow().read_oam(sprite_index, 3);
                yield;
                secondary_oam[secondary_oam_index + 3] = x_coordinate;
                yield;
                secondary_oam_index += 4;
                cycle_count += 6;
            }
            sprite_index += 1;
        }

        // check for sprite overflow
        let mut attribute_index: u8 = 0;
        while (0..OAM::SPRITE_COUNT).contains(&sprite_index) {
            let y_coordinate: u8 = ppu.borrow().read_oam(sprite_index, attribute_index);
            yield;
            let sprite_range = usize::from(y_coordinate)
                ..(usize::from(y_coordinate) + usize::from(ppu.borrow().get_sprite_height()));
            if sprite_range.contains(&ppu.borrow().get_scanline()) {
                ppu.borrow_mut().trigger_sprite_overflow();
                // the next three entries of OAM are read, but the results are discarded
                // so we only include the attribute_index and sprite_index increments
                for _ in 0..3 {
                    if attribute_index == 3 {
                        sprite_index += 1;
                    }
                    attribute_index = (attribute_index + 1) % 4;
                    yield;
                }
                cycle_count += 3;
            } else {
                attribute_index = (attribute_index + 1) % 4;
                sprite_index = (sprite_index + 1) % 4;
            }
        }

        // stall until cycle 256
        while cycle_count < 256 {
            yield;
            cycle_count += 1;
        }

        // fetch sprite data
        let mut result: Vec<Sprite> = Vec::new();
        for i in 0..SECONDARY_OAM_SPRITE_COUNT {
            let y_coordinate: u8 = secondary_oam[i];
            yield;
            let tile_index: u8 = secondary_oam[i + 1];
            yield;
            let attributes: u8 = secondary_oam[i + 2];
            yield;
            let x_coordinate: u8 = secondary_oam[i + 3];
            yield;
            let low_tile_addr: u16 = ppu.borrow().get_sprite_tile_addr_low(
                tile_index,
                ppu.borrow().get_scanline() as u8 - y_coordinate,
            );
            yield;
            let pattern_low: u8 = ppu.borrow().get(low_tile_addr);
            yield;
            let high_tile_addr: u16 = ppu.borrow().get_sprite_tile_addr_high(
                tile_index,
                ppu.borrow().get_scanline() as u8 - y_coordinate,
            );
            yield;
            let pattern_high: u8 = ppu.borrow().get(high_tile_addr);
            yield;
            cycle_count += 8;
            result.push(Sprite {
                x_coordinate,
                attributes,
                pattern_low,
                pattern_high,
            })
        }

        // stall until cycle 340
        while cycle_count < 340 {
            yield;
            cycle_count += 1;
        }
        result
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Sprite {
    x_coordinate: u8,
    attributes: u8,
    pattern_low: u8,
    pattern_high: u8,
}

