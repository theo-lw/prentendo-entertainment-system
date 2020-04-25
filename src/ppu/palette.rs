use super::Color;

pub const PALETTE_BACKGROUND_BASE: u16 = 0x3F00;
pub const PALETTE_SPRITE_BASE: u16 = 0x3F10;

/// The NES color lookup table. This is hardcoded for now.
pub const NES_COLORS: [Color; 0x40] = [
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
