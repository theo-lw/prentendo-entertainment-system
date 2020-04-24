use super::NES;
use crate::bitops::BitOps;
use sdl2::keyboard::{KeyboardState, Scancode};
use std::cell::Cell;

pub trait Controller {
    fn update_controller(&mut self, keyboard: KeyboardState);
}

/// Represents internal IO state
pub struct IOState {
    strobe_on: bool,
    buttons: [bool; 8],
    index: Cell<usize>,
}

impl Controller for NES {
    fn update_controller(&mut self, keyboard: KeyboardState) {
        if !self.io.strobe_on {
            return;
        }
        self.io.buttons = [
            keyboard.is_scancode_pressed(Scancode::Z),
            keyboard.is_scancode_pressed(Scancode::X),
            keyboard.is_scancode_pressed(Scancode::Space),
            keyboard.is_scancode_pressed(Scancode::Return),
            keyboard.is_scancode_pressed(Scancode::Up),
            keyboard.is_scancode_pressed(Scancode::Down),
            keyboard.is_scancode_pressed(Scancode::Left),
            keyboard.is_scancode_pressed(Scancode::Right),
        ];
    }
}

impl IOState {
    #[cfg(test)]
    pub fn mock() -> Self {
        Self::new()
    }

    pub fn new() -> Self {
        IOState {
            strobe_on: false,
            buttons: [false; 8],
            index: Cell::new(0),
        }
    }

    pub fn write(&mut self, val: u8) {
        self.strobe_on = val.is_bit_set(0);
        if self.strobe_on {
            self.index.set(0);
        }
    }

    pub fn read(&self) -> u8 {
        if self.index.get() >= 8 {
            return 1;
        }
        let result: u8 = if self.buttons[self.index.get()] { 1 } else { 0 };
        self.index.set(self.index.get() + 1);
        if self.strobe_on {
            self.index.set(0);
        }
        result
    }
}
