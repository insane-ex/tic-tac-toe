use std::fmt::{self, Write};

use super::{bitboard::Bitboard, player::Player};

#[derive(Debug, PartialEq, Eq)]
enum Cell {
    Empty(u16),
    Occupied(Player),
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty(index) => write!(f, "{}", index + 1),
            Self::Occupied(player) => write!(f, "{player}"),
        }
    }
}

#[allow(unused)]
pub struct Game {
    bitboards: [Bitboard; 2],
    player_turn: Player,
}

impl Game {
    #[must_use]
    pub fn new() -> Self {
        Self {
            bitboards: [Bitboard::default(), Bitboard::default()],
            player_turn: Player::X,
        }
    }

    const fn cell_at(&self, index: u16) -> Cell {
        debug_assert!(index < 9);

        if self.bitboards[0].has(index) {
            Cell::Occupied(Player::X)
        } else if self.bitboards[1].has(index) {
            Cell::Occupied(Player::O)
        } else {
            Cell::Empty(index)
        }
    }

    fn board_string(&self) -> String {
        let mut buffer = String::new();

        buffer.push_str("+---+---+---+\n");

        for row in 0..3 {
            buffer.push('|');

            for column in 0..3 {
                let cell_idx = row * 3 + column;
                let _ = write!(buffer, " {} |", self.cell_at(cell_idx));
            }

            buffer.push_str("\n+---+---+---+");

            if row != 2 {
                buffer.push('\n');
            }
        }

        buffer
    }

    pub fn print_board(&self) {
        println!("{}", self.board_string());
    }
}

#[cfg(test)]
impl Game {
    #[must_use]
    pub fn from_bitboards(a: Bitboard, b: Bitboard) -> Self {
        Self {
            bitboards: [a, b],
            player_turn: Player::X,
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{Bitboard, Cell, Game, Player};

    #[test]
    fn new_game_correctly_sets_game() {
        let game = Game::new();

        assert_eq!(game.bitboards[0], Bitboard::new());
        assert_eq!(game.bitboards[1], Bitboard::new());
        assert_eq!(game.player_turn, Player::X);
    }

    #[test]
    fn cell_at_returns_player_x() {
        let game = Game::from_bitboards(Bitboard::from_bits(0b1), Bitboard::from_bits(0b11));

        assert_eq!(game.cell_at(0), Cell::Occupied(Player::X));
    }

    #[test]
    fn cell_at_returns_player_o() {
        let game = Game::from_bitboards(Bitboard::from_bits(0b1), Bitboard::from_bits(0b11));

        assert_eq!(game.cell_at(1), Cell::Occupied(Player::O));
    }

    #[test]
    fn cell_at_returns_empty() {
        let game = Game::new();

        assert_eq!(game.cell_at(0), Cell::Empty(0));
    }

    #[test]
    fn board_string_returns_correct_board_representation() {
        let game = Game::new();
        let board = game.board_string();

        assert_eq!(
            format!("{board}"),
            "+---+---+---+\n| 1 | 2 | 3 |\n+---+---+---+\n| 4 | 5 | 6 |\n+---+---+---+\n| 7 | 8 | 9 |\n+---+---+---+"
        );
    }
}
