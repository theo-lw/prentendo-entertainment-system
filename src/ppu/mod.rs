mod background_evaluation;
mod pipeline;
mod sprite_evaluation;

use crate::state::PPU;
use background_evaluation::evaluate_background;
use pipeline::Pipeline;
use sprite_evaluation::evaluate_sprites;
use std::cell::RefCell;
use std::ops::Generator;

/// This function decides what happens on each cycle, yielding pixels along the way
pub fn cycle<'a, T: PPU>(
    ppu: &'a RefCell<T>,
) -> impl Generator<Yield = Option<Pixel>, Return = ()> + 'a {
    move || {
        let mut pipeline = Pipeline::new();
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

fn should_output_pixel(scanline: usize, tick: usize) -> bool {
    (1..=256).contains(&tick) && (0..=239).contains(&scanline)
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

/// The NES color lookup table. This is hardcoded for now.
const NES_COLORS: [Color; 0x40] = [
    Color {
        r: 84,
        g: 84,
        b: 84,
    },
    Color {
        r: 0,
        g: 30,
        b: 116,
    },
    Color {
        r: 8,
        g: 16,
        b: 144,
    },
    Color {
        r: 48,
        g: 0,
        b: 136,
    },
    Color {
        r: 68,
        g: 0,
        b: 100,
    },
    Color { r: 92, g: 0, b: 48 },
    Color { r: 84, g: 4, b: 0 },
    Color { r: 60, g: 24, b: 0 },
    Color { r: 32, g: 42, b: 0 },
    Color { r: 8, g: 58, b: 0 },
    Color { r: 0, g: 64, b: 0 },
    Color { r: 0, g: 60, b: 0 },
    Color { r: 0, g: 50, b: 60 },
    Color { r: 0, g: 0, b: 0 },
    Color { r: 0, g: 0, b: 0 },
    Color { r: 0, g: 0, b: 0 },
    Color {
        r: 152,
        g: 150,
        b: 152,
    },
    Color {
        r: 8,
        g: 76,
        b: 196,
    },
    Color {
        r: 48,
        g: 50,
        b: 236,
    },
    Color {
        r: 92,
        g: 30,
        b: 228,
    },
    Color {
        r: 136,
        g: 20,
        b: 176,
    },
    Color {
        r: 160,
        g: 20,
        b: 100,
    },
    Color {
        r: 152,
        g: 34,
        b: 32,
    },
    Color {
        r: 120,
        g: 60,
        b: 0,
    },
    Color { r: 84, g: 90, b: 0 },
    Color {
        r: 40,
        g: 114,
        b: 0,
    },
    Color { r: 8, g: 124, b: 0 },
    Color {
        r: 0,
        g: 118,
        b: 40,
    },
    Color {
        r: 0,
        g: 102,
        b: 120,
    },
    Color { r: 0, g: 0, b: 0 },
    Color { r: 0, g: 0, b: 0 },
    Color { r: 0, g: 0, b: 0 },
    Color {
        r: 236,
        g: 238,
        b: 236,
    },
    Color {
        r: 76,
        g: 154,
        b: 236,
    },
    Color {
        r: 120,
        g: 124,
        b: 236,
    },
    Color {
        r: 176,
        g: 98,
        b: 236,
    },
    Color {
        r: 228,
        g: 84,
        b: 236,
    },
    Color {
        r: 236,
        g: 88,
        b: 180,
    },
    Color {
        r: 236,
        g: 106,
        b: 100,
    },
    Color {
        r: 212,
        g: 136,
        b: 32,
    },
    Color {
        r: 160,
        g: 170,
        b: 0,
    },
    Color {
        r: 116,
        g: 196,
        b: 0,
    },
    Color {
        r: 76,
        g: 208,
        b: 32,
    },
    Color {
        r: 56,
        g: 204,
        b: 108,
    },
    Color {
        r: 56,
        g: 180,
        b: 204,
    },
    Color {
        r: 60,
        g: 60,
        b: 60,
    },
    Color { r: 0, g: 0, b: 0 },
    Color { r: 0, g: 0, b: 0 },
    Color {
        r: 236,
        g: 238,
        b: 236,
    },
    Color {
        r: 168,
        g: 204,
        b: 236,
    },
    Color {
        r: 188,
        g: 188,
        b: 236,
    },
    Color {
        r: 212,
        g: 178,
        b: 236,
    },
    Color {
        r: 236,
        g: 174,
        b: 236,
    },
    Color {
        r: 236,
        g: 174,
        b: 212,
    },
    Color {
        r: 236,
        g: 180,
        b: 176,
    },
    Color {
        r: 228,
        g: 196,
        b: 144,
    },
    Color {
        r: 204,
        g: 210,
        b: 120,
    },
    Color {
        r: 180,
        g: 222,
        b: 120,
    },
    Color {
        r: 168,
        g: 226,
        b: 144,
    },
    Color {
        r: 152,
        g: 226,
        b: 180,
    },
    Color {
        r: 160,
        g: 214,
        b: 228,
    },
    Color {
        r: 160,
        g: 162,
        b: 160,
    },
    Color { r: 0, g: 0, b: 0 },
    Color { r: 0, g: 0, b: 0 },
];
