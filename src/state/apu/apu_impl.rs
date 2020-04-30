use super::APU;
use crate::state::NES;

impl<'a> APU<'a> for NES {
    fn get_buffer(&'a self) -> &'a [f32] {
        &self.apu.buffer
    }

    fn apu_cycle(&mut self) {
        
    }
}
