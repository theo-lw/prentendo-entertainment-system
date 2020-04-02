/// Helper trait for defining bit operations 
pub trait BitOps {
    fn is_bit_set(&self, val: usize) -> bool;
}

impl BitOps for u8 {
    fn is_bit_set(&self, val: usize) -> bool {
        (self & 1 << val) != 0
    }
}

