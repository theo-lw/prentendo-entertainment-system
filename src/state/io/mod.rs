/// Represents internal IO state
pub struct IOState {
    pub joy1: u8,
    pub joy2: u8,
    pub unused1: u8,
    pub unused2: u8,
    pub test_data: [u8; 8],
}

impl IOState {
    #[cfg(test)]
    pub fn mock() -> Self {
        Self::new()
    }

    pub fn new() -> Self {
        IOState {
            joy1: 0,
            joy2: 0,
            unused1: 0,
            unused2: 0,
            test_data: [0; 8],
        }
    }
}
