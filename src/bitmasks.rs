pub const EMPTY_BOARD_BITMASK: u16 = 0;
pub const BOARD_FULL_BITMASK: u16 = (1u16 << 9) - 1;

pub const WIN_CONDITIONS_BITMASKS: [u16; 8] = [
    // Rows
    (1u16 << 0) | (1u16 << 1) | (1u16 << 2),
    (1u16 << 3) | (1u16 << 4) | (1u16 << 5),
    (1u16 << 6) | (1u16 << 7) | (1u16 << 8),
    // Columns
    (1u16 << 0) | (1u16 << 3) | (1u16 << 6),
    (1u16 << 1) | (1u16 << 4) | (1u16 << 7),
    (1u16 << 2) | (1u16 << 5) | (1u16 << 8),
    // Diagonals
    (1u16 << 0) | (1u16 << 4) | (1u16 << 8),
    (1u16 << 2) | (1u16 << 4) | (1u16 << 6),
];
