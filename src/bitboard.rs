use crate::bitmasks::EMPTY_BOARD_BITMASK;

pub struct Bitboard {
    bits: u16,
}

impl Bitboard {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            bits: EMPTY_BOARD_BITMASK,
        }
    }

    #[must_use]
    pub const fn has_bit(&self, bit: u16) -> bool {
        (self.bits & (1 << bit)) != 0
    }

    pub const fn set_bit(&mut self, bit: u16) {
        self.bits |= 1 << bit;
    }
}

impl Default for Bitboard {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{Bitboard, EMPTY_BOARD_BITMASK};

    #[test]
    fn new_bitboard_starts_empty() {
        let bitboard = Bitboard::default();

        assert_eq!(bitboard.bits, EMPTY_BOARD_BITMASK);
    }

    #[test]
    fn set_bit_sets_correct_bit() {
        let mut bitboard = Bitboard::default();

        bitboard.set_bit(1);

        assert!(bitboard.has_bit(1));
    }

    #[test]
    fn has_bit_returns_false_for_unset_bits() {
        let bitboard = Bitboard::default();

        assert!(!bitboard.has_bit(0));
    }
}
