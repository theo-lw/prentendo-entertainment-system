use super::background_evaluation::BackgroundTile;
use super::sprite_evaluation::Sprite;
use crate::bitops::BitOps;

pub struct Pipeline {
    sprites: Option<Vec<Sprite>>,
    background_shift_high: Option<u16>,
    background_shift_low: Option<u16>,
    background_attribute_current: Option<u8>,
    background_attribute_next: Option<u8>,
    background_shift_count: u8,
}

impl Pipeline {
    pub fn new() -> Self {
        Pipeline {
            sprites: None,
            background_shift_low: None,
            background_shift_high: None,
            background_attribute_next: None,
            background_attribute_current: None,
            background_shift_count: 0,
        }
    }

    /// Returns an optional tuple containing:
    /// 1. The address of the next pixel's palette color
    /// 2. Whether a sprite0 hit has occurred
    pub fn get_next_palette_addr(&self, fine_x: u8, fine_y: u8) -> Option<(u16, bool)> {
        let background_attr: Option<u8> = if fine_x + self.background_shift_count >= 8 {
            self.background_attribute_next
        } else {
            self.background_attribute_current
        };

        let background_palette: Option<u16> = background_attr.map(|x| 0x3F00 + u16::from(x) * 4);

        let background_palette_index: Option<u16> = map2(
            self.background_shift_high,
            self.background_shift_low,
            |a, b| (((a >> (15 - fine_x)) << 1) | (b >> (15 - fine_x))) & 0b11,
        );

        let background_palette_addr: Option<u16> =
            map2(background_palette, background_palette_index, |a, b| a + b);

        let first_active_sprite: Option<&Sprite> = self.sprites.as_ref().and_then(|vec| {
            for sprite in vec {
                if sprite.is_active() {
                    return Some(sprite);
                }
            }
            None
        });

        // sprite priority and sprite-zero code
        match (
            background_palette_index,
            first_active_sprite.map(|x| x.is_transparent()),
            first_active_sprite.map(|x| x.is_front_priority()),
        ) {
            (Some(_), None, None) => background_palette_addr.map(|x| (x, false)),
            (None, Some(_), Some(_)) => {
                first_active_sprite.map(|x| (x.get_current_pixel_palette_addr(), false))
            }
            (Some(_), Some(true), Some(_)) => background_palette_addr.map(|x| (x, false)),
            (Some(a), Some(b), Some(false)) => background_palette_addr.map(|x| (x, a != 0 && !b)),
            (Some(0), Some(_), Some(_)) => {
                first_active_sprite.map(|x| (x.get_current_pixel_palette_addr(), false))
            }
            (Some(a), Some(b), Some(true)) => {
                first_active_sprite.map(|x| (x.get_current_pixel_palette_addr(), a != 0 && !b))
            }
            _ => None,
        }
    }

    pub fn advance_pipeline(&mut self) {
        self.background_shift_high = self.background_shift_high.map(|x| x << 1);
        self.background_shift_low = self.background_shift_low.map(|x| x << 1);
        if let Some(vec) = &mut self.sprites {
            for sprite in vec {
                sprite.shift();
            }
        }
        self.background_shift_count = (self.background_shift_count + 1) % 8;
    }

    pub fn load_background_tile(&mut self, tile: BackgroundTile) {
        self.background_attribute_current = self.background_attribute_next;
        self.background_attribute_next = Some(tile.attribute);
        self.background_shift_high = match self.background_shift_high {
            None => Some(u16::from(tile.pattern_high)),
            Some(x) => Some(x.replace_bits(0b1111_1111, u16::from(tile.pattern_high))),
        };
        self.background_shift_low = match self.background_shift_low {
            None => Some(u16::from(tile.pattern_low)),
            Some(x) => Some(x.replace_bits(0b1111_1111, u16::from(tile.pattern_low))),
        };
        self.background_shift_count = 0;
    }

    pub fn load_sprites(&mut self, sprites: Vec<Sprite>) {
        self.sprites = Some(sprites);
    }
}

fn map2<T, U, V, F: Fn(T, U) -> V>(a: Option<T>, b: Option<U>, f: F) -> Option<V> {
    match a {
        Some(x) => match b {
            Some(y) => Some(f(x, y)),
            None => None,
        },
        None => None,
    }
}
