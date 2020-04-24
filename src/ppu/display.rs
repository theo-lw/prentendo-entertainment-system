use super::Pixel;

pub struct Display {
    texture: [u8; Self::BYTES],
}

impl<'a> Display {
    pub const BYTES_PER_PIXEL: usize = 4;

    const BYTES: usize = Self::WIDTH * Self::HEIGHT * Self::BYTES_PER_PIXEL;

    pub const WIDTH: usize = 256;

    pub const HEIGHT: usize = 240;

    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_pixel(&mut self, pixel: Pixel) {
        let base_index: usize = Self::BYTES_PER_PIXEL * (pixel.x + pixel.y * Self::WIDTH);
        self.texture[base_index] = u8::max_value();
        self.texture[base_index + 1] = pixel.color.b;
        self.texture[base_index + 2] = pixel.color.g;
        self.texture[base_index + 3] = pixel.color.r;
    }

    pub fn get(&'a self) -> &'a [u8; Self::BYTES] {
        &self.texture
    }
}

impl Default for Display {
    fn default() -> Self {
        Display {
            texture: [0; Self::BYTES],
        }
    }
}
