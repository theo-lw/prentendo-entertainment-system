mod background_evaluation;
mod pipeline;
mod sprite_evaluation;

use crate::state::PPU;
use background_evaluation::evaluate_background;
use sprite_evaluation::evaluate_sprites;
use std::cell::RefCell;
use std::ops::Generator;

/// This function decides what happens on each cycle, yielding pixels along the way
pub fn cycle<'a, T: PPU>(
    ppu: &'a RefCell<T>,
) -> impl Generator<Yield = Option<Pixel>, Return = ()> + 'a {
    move || {
        let mut background_generator = evaluate_background(ppu);
        let mut sprite_generator = evaluate_sprites(ppu);
        loop {
            let scanline: usize = ppu.borrow().get_scanline();
            let tick: usize = ppu.borrow().get_tick();

            // the zero tick is always idle - nothing happens other than a cycle update
            if tick == 0 {
                yield None;
                ppu.borrow_mut().update_cycle();
                continue;
            }

            ppu.borrow_mut().update_cycle();
        }
    }
}

fn should_run_background(scanline: usize, tick: usize) -> bool {
    ((1..=256).contains(&tick) || (321..=336).contains(&tick))
        && ((0..=239).contains(&scanline) || scanline == 261)
        && tick != 0
}

fn should_run_sprites(scanline: usize, tick: usize) -> bool {
    (0..=239).contains(&scanline) && tick != 0
}

fn should_increment_x(scanline: usize, tick: usize) -> bool {
    (tick % 8) == 0 && should_run_background(scanline, tick)
}

fn should_increment_y(scanline: usize, tick: usize) -> bool {
    tick == 256 && should_run_background(scanline, tick)
}

fn should_reset_x(scanline: usize, tick: usize) -> bool {
    tick == 257 && should_run_background(scanline, tick)
}

fn should_reset_y(scanline: usize, tick: usize) -> bool {
    (280..=304).contains(&tick) && scanline == 261
}

fn should_set_vblank(scanline: usize, tick: usize) -> bool {
    scanline == 241 && tick == 1
}

fn should_clear_flags(scanline: usize, tick: usize) -> bool {
    scanline == 261 && tick == 1
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Pixel {
    pub x: usize,
    pub y: usize,
    pub color: Color,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
