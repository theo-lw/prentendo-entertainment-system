pub struct RAM {
    pub nametable_a: [u8; 0x400],
    pub nametable_b: [u8; 0x400],
    pub palatte_ram: [u8; 0x20],
}

impl RAM {
    pub fn new() -> Self {
        RAM {
            nametable_a: [0; 0x400],
            nametable_b: [0; 0x400],
            palatte_ram: [0; 0x20],
        }
    }
}
