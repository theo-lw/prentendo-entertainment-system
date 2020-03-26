pub trait AddressMap {
    fn get(&self, index: u16) -> u8;
    fn set(&mut self, index: u16, val: u8);
}
