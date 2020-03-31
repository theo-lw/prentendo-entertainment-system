/// Trait to be implemented by memory mappings
/// This exists because the address buses in the CPU and the PPU can map addresses to strange memory
/// locations
pub trait AddressMap {
    fn get(&self, index: u16) -> u8;
    fn set(&mut self, index: u16, val: u8);
}
