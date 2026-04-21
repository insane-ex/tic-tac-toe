use std::ops::{BitAnd, BitOr};

use super::bitmask::{EMPTY_BOARD_BITMASK, FULL_BOARD_BITMASK};

#[derive(Debug, PartialEq, Eq)]
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
    pub const fn is_empty(&self) -> bool {
        self.bits == EMPTY_BOARD_BITMASK
    }

    #[must_use]
    pub const fn is_full(&self) -> bool {
        self.bits == FULL_BOARD_BITMASK
    }

    pub const fn set(&mut self, bit: u16) {
        self.bits |= 1 << bit;
    }

    #[must_use]
    pub const fn has(&self, bit: u16) -> bool {
        (self.bits & (1 << bit)) != 0
    }

    #[must_use]
    pub const fn has_mask(&self, mask: u16) -> bool {
        (self.bits & mask) == mask
    }
}

impl Default for Bitboard {
    fn default() -> Self {
        Self::new()
    }
}

impl BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits & rhs.bits,
        }
    }
}

impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits | rhs.bits,
        }
    }
}

impl BitOr for &Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard {
            bits: self.bits | rhs.bits,
        }
    }
}

#[cfg(test)]
impl Bitboard {
    #[must_use]
    pub const fn from_bits(bits: u16) -> Self {
        Self { bits }
    }
}

#[cfg(test)]
mod tests {
    use super::{Bitboard, FULL_BOARD_BITMASK};

    #[test]
    fn new_bitboard_is_empty() {
        let bitboard = Bitboard::new();

        assert!(bitboard.is_empty());
    }

    #[test]
    fn set_correctly_sets_bit() {
        let mut bitboard = Bitboard::new();

        bitboard.set(0);

        assert!(bitboard.has(0));
    }

    #[test]
    fn has_returns_false() {
        let bitboard = Bitboard::new();

        assert!(!bitboard.has(0));
    }

    #[test]
    fn bitwise_or_combines_bits() {
        let a = Bitboard::from_bits(0b01);
        let b = Bitboard::from_bits(0b10);

        let result = a | b;

        assert!(result.has(0));
        assert!(result.has(1));
    }

    #[test]
    fn bitwise_and_intersects_bits() {
        let a = Bitboard::from_bits(0b0101);
        let b = Bitboard::from_bits(0b0011);

        let result = a & b;

        assert_eq!(result, Bitboard::from_bits(0b0001));
    }

    #[test]
    fn bitwise_and_returns_empty_when_no_intersection() {
        let a = Bitboard::from_bits(0b0001);
        let b = Bitboard::from_bits(0b0010);

        let result = a & b;

        assert!(result.is_empty());
    }

    #[test]
    fn is_full_returns_true() {
        let bitboard = Bitboard::from_bits(FULL_BOARD_BITMASK);

        assert!(bitboard.is_full());
    }

    #[test]
    fn is_full_returns_false() {
        let bitboard = Bitboard::new();

        assert!(!bitboard.is_full());
    }

    #[test]
    fn has_mask_returns_true() {
        let bitboard = Bitboard::from_bits(0b000_000_111);

        assert!(bitboard.has_mask(0b111));
    }

    #[test]
    fn has_mask_returns_false() {
        let bitboard = Bitboard::new();

        assert!(!bitboard.has_mask(0b111));
    }
}
