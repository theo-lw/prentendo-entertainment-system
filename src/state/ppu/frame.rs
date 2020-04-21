use super::{Frame, Background, Sprites};
use crate::state::NES;

impl Frame for NES {
    fn is_short_frame(&self) -> bool {
        self.ppu.current_cycle.is_odd_frame
            && (self.should_render_sprites() || self.should_render_background())
    }
}
