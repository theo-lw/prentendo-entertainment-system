#![feature(generator_trait)]
use pretendo_entertainment_system::cartridge::ines::{ROMError, INES};
use pretendo_entertainment_system::cartridge::Mapper;
use pretendo_entertainment_system::cpu;
use pretendo_entertainment_system::cpu::InstructionState;
use pretendo_entertainment_system::state::cpu::{Registers};
use pretendo_entertainment_system::state::NES;
use std::cell::RefCell;
use std::env;
use std::fs::File;
use std::io::Write;
use std::ops::{Generator, GeneratorState};
use std::path::PathBuf;
use std::pin::Pin;
use file_diff::diff;

#[test]
fn nes_test_cpu() -> Result<(), ROMError> {
    // Initialize NES
    let rom_path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "data", "nestest.nes"]
        .iter()
        .collect();
    let mut rom = File::open(rom_path)?;
    let mapper: Box<dyn Mapper> = INES::from_file(&mut rom)?.to_mapper();
    let nes: RefCell<NES> = RefCell::new(NES::new(mapper));
    nes.borrow_mut().set_pc(0xC000);
    nes.borrow_mut().set_p(0x24);

    // Create log
    let log_path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "data", "test.log"]
        .iter()
        .collect();
    let mut log = File::create(log_path)?;
    let mut cycle: u16 = 0;
    write!(
        log,
        "{:04X} A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} CYC:{:>3}\n",
        nes.borrow().get_pc(),
        nes.borrow().get_a(),
        nes.borrow().get_x(),
        nes.borrow().get_y(),
        nes.borrow().get_p(),
        nes.borrow().get_s(),
        cycle
    )?;
    
    // Execute instructions cycle by cycle
    let mut cpu_generator = cpu::cycle(&nes);
    // at this point the ROM starts executing unofficial unstructions
    while nes.borrow().get_pc() != 0xC6BD {
        cycle += 3;
        cycle %= 341;
        if let GeneratorState::Yielded(InstructionState::Complete(_)) =
            Pin::new(&mut cpu_generator).resume(())
        {
            write!(
                log,
                "{:04X} A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} CYC:{:>3}\n",
                nes.borrow().get_pc(),
                nes.borrow().get_a(),
                nes.borrow().get_x(),
                nes.borrow().get_y(),
                nes.borrow().get_p(),
                nes.borrow().get_s(),
                cycle
            )?;
        }
    }
    let correct_log_path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "data", "nestest.log"]
        .iter()
        .collect();
    let log_path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "data", "test.log"]
        .iter()
        .collect();
    assert!(diff(log_path.to_str().unwrap(), correct_log_path.to_str().unwrap()));
    Ok(())
}
