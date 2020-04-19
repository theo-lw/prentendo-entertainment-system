#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CycleStatus {
    pub tick: usize,
    pub scanline: usize,
    pub is_odd_frame: bool,
}

impl CycleStatus {
    pub const MAX_SCANLINES: usize = 262;
    pub const MAX_TICKS: usize = 341;

    pub fn new() -> Self {
        CycleStatus {
            tick: 0,
            scanline: 261,
            is_odd_frame: false,
        }
    }

    pub fn is_on_render_line(&self) -> bool {
        (0..=239).contains(&self.scanline) || self.scanline == 261
    }
}
