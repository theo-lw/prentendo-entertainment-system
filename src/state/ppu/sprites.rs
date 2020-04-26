use super::mapped_registers::SpriteHeight;
use super::oam::OAM;
use super::Sprites;
use crate::state::NES;

impl Sprites for NES {
    fn should_render_sprites(&self) -> bool {
        self.ppu.mask.should_render_sprites()
    }
    fn get_sprite_tile_addr_low(&self, sprite_index: u8, y_offset: u8) -> u16 {
        match self.ppu.ctrl.get_sprite_height() {
            SpriteHeight::Eight => {
                self.ppu.ctrl.get_sprite_pattern_table_addr()
                    + (u16::from(sprite_index) << 4)
                    + u16::from(y_offset)
            }
            SpriteHeight::Sixteen => {
                let table = sprite_index & 0b1;
                let sprite_index =
                    sprite_index.wrapping_add(if y_offset > 0b111 { 1 } else { 0 });
                (u16::from(table) << 12)
                    + (u16::from(sprite_index) << 4)
                    + u16::from(y_offset & 0b111)
            }
        }
    }
    fn get_sprite_tile_addr_high(&self, sprite_index: u8, y_offset: u8) -> u16 {
        self.get_sprite_tile_addr_low(sprite_index, y_offset) + 0b1000
    }
    fn get_sprite_height(&self) -> SpriteHeight {
        self.ppu.ctrl.get_sprite_height()
    }
    fn read_oam(&self, tile: u8, offset: u8) -> u8 {
        self.ppu.oam.memory[usize::from(OAM::BYTES_PER_SPRITE * tile + offset)]
    }
    fn trigger_sprite_overflow(&mut self) {
        self.ppu.status.sprite_overflow = true;
    }
    fn trigger_sprite_zero(&mut self) {
        self.ppu.status.sprite0_hit = true;
    }
    fn clear_sprite_overflow(&mut self) {
        self.ppu.status.sprite_overflow = false;
    }
    fn clear_sprite_zero(&mut self) {
        self.ppu.status.sprite0_hit = false;
    }
}
