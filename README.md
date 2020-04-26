# Pretendo Entertainment System

A NES emulator written in Rust. Currently under construction. The goals of this emulator are accuracy and readability. Consequently, it is not the fastest emulator, although it does run at a healthy 60 FPS on my base model 2015 Macbook Air.

Todo:
- Second controller support
- APU emulation
- NES 2.0 file formats
- Additional mappers (currently, only Mappers 0 and 2 are supported)

## Requirements

- SDL2: graphics, keyboard I/O
- Nightly Rust: this emulator makes heavy use of generators, which is currently only available on the nightly toolchain.

## Usage

```
cargo run --release <rom>
```

## FAQ

I don't think I've received any questions about this emulator, so it doesn't quite make sense to add a FAQ. But I want a section to talk about some miscellaneous aspects of my emulator, so here goes.

### Generators?

NES opcodes are composed of a sequence of reads and writes. For instance, the ASL instruction looks like

        1    PC     R  fetch opcode, increment PC
        2    PC     R  fetch address, increment PC
        3  address  R  read from effective address
        4  address  W  write the value back to effective address,
        5  address  W  shift the value left and write the new value to effective address

The state of the NES may change in-between any one of these steps. I use generators to ensure that each part of the NES is synchronized after every cycle.
