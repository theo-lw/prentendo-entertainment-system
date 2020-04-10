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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_bit_set() {
        let byte = 0b0010_1100;
        assert!(byte.is_bit_set(2));
        assert!(!byte.is_bit_set(4));
    }

    #[test]
    fn test_set_bit() {
        let mut byte = 0b0010_1100;
        byte.set_bit(0);
        byte.set_bit(3);
        assert_eq!(byte, 0b0010_1101);
    }

    #[test]
    fn test_clear_bit() {
        let mut byte = 0b0010_1100;
        byte.clear_bit(5);
        byte.clear_bit(0);
        assert_eq!(byte, 0b0000_1100);
    }
}
