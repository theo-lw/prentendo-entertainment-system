/// Helper trait for defining bit operations
pub trait BitOps {
    fn is_bit_set(&self, val: usize) -> bool;
    fn set_bit(&mut self, val: usize);
    fn clear_bit(&mut self, val: usize);
}

impl BitOps for u8 {
    fn is_bit_set(&self, val: usize) -> bool {
        (self & 1 << val) != 0
    }

    fn set_bit(&mut self, val: usize) {
        *self |= 1 << val;
    }

    fn clear_bit(&mut self, val: usize) {
        *self &= !(1 << val);
    }
}
