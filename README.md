# Pretendo Entertainment System

A NES (Nintendo Entertainment System) emulator written in Rust. It includes a cycle-accurate CPU, a somewhat accurate PPU and a less accurate APU. Because this emulator strives for accuracy, it is not the fastest emulator, although it does run at a healthy 60 FPS on my base model 2015 Macbook Air.

## Demos

Please don't sue me Nintendo :(

[![Super Mario Bros.](https://i.gyazo.com/03d0ff27a57cd58624a9ddaf848e0b33.gif)](https://gyazo.com/03d0ff27a57cd58624a9ddaf848e0b33)

[![Donkey Kong](https://i.gyazo.com/7776e4cd783c69e54f49de8d156ae795.gif)](https://gyazo.com/7776e4cd783c69e54f49de8d156ae795)

[![Contra](https://i.gyazo.com/51a6abe2cc313fde4540dfc227ba204a.gif)](https://gyazo.com/51a6abe2cc313fde4540dfc227ba204a)

[![Mega Man](https://i.gyazo.com/71d01fcddde4a8a4244dde9556e82ed8.gif)](https://gyazo.com/71d01fcddde4a8a4244dde9556e82ed8)

## Requirements

- SDL2: graphics, sound, keyboard I/O
- Nightly Rust: this emulator makes heavy use of generators, which is currently only available on the nightly toolchain.

## Usage

```
cargo run --release <rom>
```

```
Keyboard Map

Enter - Start
Space - Select
Up    - Up
Down  - Down
Left  - Left
Z     - A
X     - B
```

## Todo
- Second controller support
- Customizable keyboard mappings
- Debug views
- Save states
- NES 2.0 file formats (only INES file formats are supported)
- Additional mappers (only Mappers 0 and 2 are supported)
- Miscellaneous APU and PPU bugfixes.

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
