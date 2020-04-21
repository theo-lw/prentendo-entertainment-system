#![feature(generators, generator_trait)]
use pretendo_entertainment_system::cartridge::ines::{ROMError, INES};
use pretendo_entertainment_system::cartridge::Mapper;
use pretendo_entertainment_system::cpu;
use pretendo_entertainment_system::ppu;
use pretendo_entertainment_system::ppu::display::Display;
use pretendo_entertainment_system::ppu::Pixel;
use pretendo_entertainment_system::state::ppu::Frame;
use pretendo_entertainment_system::state::NES;
use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use std::cell::RefCell;
use std::fs::File;
use std::ops::{Generator, GeneratorState};
use std::path::PathBuf;
use std::pin::Pin;
use std::time::Duration;
use structopt::StructOpt;

const BASE_CYCLES_PER_FRAME: u16 = 29780;
const PPU_CYCLES_PER_CPU_CYCLE: u8 = 3;
const PIXEL_SCALE: u32 = 2;

#[derive(StructOpt)]
#[structopt(version = "0.1", author = "Theodore Wang")]
struct Opts {
    /// Input ROM
    #[structopt(parse(from_os_str))]
    rom: PathBuf,
}

fn main() -> Result<(), ROMError> {
    // Parse arguments
    let opts = Opts::from_args();

    // Initialize NES
    let mut rom = File::open(opts.rom)?;
    let mapper: Box<dyn Mapper> = INES::from_file(&mut rom)?.to_mapper();
    let nes: RefCell<NES> = RefCell::new(NES::new(mapper));
    let mut display: Display = Display::new();
    let mut cpu_generator = cpu::cycle(&nes);
    let mut ppu_generator = ppu::cycle(&nes);

    // Set up SDL
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window(
            "Pretendo Entertainment System",
            Display::WIDTH as u32 * PIXEL_SCALE,
            Display::HEIGHT as u32 * PIXEL_SCALE,
        )
        .position_centered()
        .build()
        .unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut canvas: WindowCanvas = window.into_canvas().present_vsync().build().unwrap();
    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    let mut texture: Texture = texture_creator
        .create_texture_streaming(
            PixelFormatEnum::RGB888,
            Display::WIDTH as u32,
            Display::HEIGHT as u32,
        )
        .expect("Could not create texture!");

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        if nes.borrow().is_short_frame() {
            Pin::new(&mut cpu_generator).resume(());
            for _ in 0..2 {
                run_ppu(&mut ppu_generator, &mut display);
            }
        } else {
            run_ppu(&mut ppu_generator, &mut display);
        }

        for _ in 0..BASE_CYCLES_PER_FRAME {
            Pin::new(&mut cpu_generator).resume(());
            for _ in 0..PPU_CYCLES_PER_CPU_CYCLE {
                run_ppu(&mut ppu_generator, &mut display);
            }
        }

        texture
            .update(
                None,
                display.get(),
                Display::WIDTH * Display::BYTES_PER_PIXEL,
            )
            .expect("Could not update texture!");
        canvas.clear();
        canvas
            .copy(&texture, None, None)
            .expect("Could not copy texture!");
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

fn run_ppu(
    ppu: &mut (impl Generator<Yield = Option<Pixel>, Return = ()> + std::marker::Unpin),
    display: &mut Display,
) {
    match Pin::new(ppu).resume(()) {
        GeneratorState::Yielded(Some(pixel)) => display.set_pixel(pixel),
        _ => {}
    }
}
