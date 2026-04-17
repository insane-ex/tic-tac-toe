pub const EMPTY_BOARD_BITMASK: u16 = 0;
pub const FULL_BOARD_BITMASK: u16 = (1 << 9) - 1;

pub const WIN_CONDITIONS_BITMASKS: [u16; 8] = [
    // Rows
    (1 << 0) | (1 << 1) | (1 << 2),
    (1 << 3) | (1 << 4) | (1 << 5),
    (1 << 6) | (1 << 7) | (1 << 8),
    // Columns
    (1 << 0) | (1 << 3) | (1 << 6),
    (1 << 1) | (1 << 4) | (1 << 7),
    (1 << 2) | (1 << 5) | (1 << 8),
    // Diagonals
    (1 << 0) | (1 << 4) | (1 << 8),
    (1 << 2) | (1 << 4) | (1 << 6),
];
