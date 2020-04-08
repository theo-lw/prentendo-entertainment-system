use pretendo_entertainment_system::cartridge::ines::{ROMError, INES};
use std::cell::RefCell;
use std::env;
use std::fs::File;
use std::path::PathBuf;
use std::rc::Rc;

#[test]
fn nes_test_cpu() -> Result<(), ROMError> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("data");
    path.push("nestest.nes");
    let mut file = File::open(path)?;
    let mapper = INES::from_file(&mut file)?.to_mapper();
    Ok(())
}
