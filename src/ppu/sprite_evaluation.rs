use crate::bitops::BitOps;
use crate::state::ppu::oam::OAM;
use crate::state::ppu::{Cycle, Memory, Sprites};
use std::cell::RefCell;
use std::ops::Generator;

const SECONDARY_OAM_SIZE: usize = 32;

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
        let mut sprite_count: usize = 0;
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
                sprite_count += 1;
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
        for i in 0..sprite_count {
            let y_coordinate: u8 = secondary_oam[i];
            yield;
            let tile_index: u8 = secondary_oam[i + 1];
            yield;
            let attributes: u8 = secondary_oam[i + 2];
            let vertical_flip: bool = attributes.is_bit_set(7);
            let horizontal_flip: bool = attributes.is_bit_set(6);
            yield;
            let x_coordinate: u8 = secondary_oam[i + 3];
            yield;
            let mut y_offset: u8 = ppu.borrow().get_scanline() as u8 - y_coordinate;
            if vertical_flip {
                y_offset = u8::from(ppu.borrow().get_sprite_height()) - y_offset;
            }
            let low_tile_addr: u16 = ppu.borrow().get_sprite_tile_addr_low(tile_index, y_offset);
            yield;
            let mut pattern_low: u8 = ppu.borrow().get(low_tile_addr);
            if horizontal_flip {
                pattern_low = pattern_low.reverse_bits();
            }
            yield;
            let high_tile_addr: u16 = ppu.borrow().get_sprite_tile_addr_high(tile_index, y_offset);
            yield;
            let mut pattern_high: u8 = ppu.borrow().get(high_tile_addr);
            if horizontal_flip {
                pattern_high = pattern_high.reverse_bits();
            }
            yield;
            cycle_count += 8;
            result.push(Sprite {
                x_coordinate,
                attributes,
                pattern_low,
                pattern_high,
                ..Default::default()
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

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Sprite {
    x_coordinate: u8,
    attributes: u8,
    pattern_low: u8,
    pattern_high: u8,
    shift_count: u8,
}

impl Sprite {
    fn get_palette_index(&self) -> u8 {
        ((self.pattern_high >> 7) << 1) | (self.pattern_low >> 7)
    }

    pub fn is_front_priority(&self) -> bool {
        !self.attributes.is_bit_set(5)
    }

    pub fn is_transparent(&self) -> bool {
        self.get_palette_index() == 0
    }

    pub fn get_current_pixel_palette_addr(&self) -> u16 {
        let base_palettte: u16 = match self.attributes & 0b11 {
            0 => 0x3F10,
            1 => 0x3F14,
            2 => 0x3F18,
            3 => 0x3F1C,
            _ => unreachable!(),
        };
        base_palettte + u16::from(self.get_palette_index())
    }

    pub fn is_active(&self) -> bool {
        self.x_coordinate == 0 && self.shift_count < 8
    }

    pub fn shift(&mut self) {
        if self.x_coordinate != 0 {
            self.x_coordinate -= 1;
        } else if self.is_active() {
            self.pattern_low = self.pattern_low << 1;
            self.pattern_high = self.pattern_high << 1;
            self.shift_count += 1;
        }
    }
}
