pub struct OAM {
    pub memory: [u8; Self::SIZE],
    pub addr: u8,
}

impl OAM {
    pub const SIZE: usize = 256;
    pub const SPRITE_COUNT: u8 = 64;
    pub const BYTES_PER_SPRITE: u8 = 4;

    pub fn new() -> Self {
        OAM {
            memory: [0; Self::SIZE],
            addr: 0,
        }
    }
}
