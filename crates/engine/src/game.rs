use std::{
    error,
    fmt::{self, Write},
};

use super::{bitboard::Bitboard, player::Player};

#[derive(Debug, PartialEq, Eq)]
pub enum GameError {
    InvalidPosition,
    PositionOccupied,
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::InvalidPosition => "Invalid position. Please enter a position between 1 and 9",
            Self::PositionOccupied => "This position is already occupied. Try another",
        })
    }
}

impl error::Error for GameError {}

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

    fn is_valid_position(position: u16) -> bool {
        (1..=9).contains(&position)
    }

    fn is_position_occupied(&self, position: u16) -> bool {
        (&self.bitboards[0] | &self.bitboards[1]).has(position)
    }

    /// Places a move for the current player.
    ///
    /// Accepts a 1-based position (1..9) and converts it internally
    /// to a zero-based index (0..8) used by the bitboard.
    ///
    /// # Errors
    ///
    /// - `GameError::InvalidPosition` if position is not in 1..9.
    /// - `GameError::PositionOccupied` if the cell is already taken.
    pub fn make_move(&mut self, position: u16) -> Result<(), GameError> {
        if !Self::is_valid_position(position) {
            return Err(GameError::InvalidPosition);
        }

        let index = position - 1;

        if self.is_position_occupied(index) {
            return Err(GameError::PositionOccupied);
        }

        let board = match self.player_turn {
            Player::X => &mut self.bitboards[0],
            Player::O => &mut self.bitboards[1],
        };

        board.set(index);

        Ok(())
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
    use super::{Bitboard, Cell, Game, GameError, Player};

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

    #[test]
    fn is_valid_position_returns_true() {
        for index in 1..=9 {
            assert!(Game::is_valid_position(index));
        }
    }

    #[test]
    fn is_valid_position_returns_false() {
        assert!(!Game::is_valid_position(0));
        assert!(!Game::is_valid_position(10));
    }

    #[test]
    fn is_position_occupied_returns_true() {
        let game = Game::from_bitboards(Bitboard::from_bits(0b101), Bitboard::from_bits(0b010));

        assert!(game.is_position_occupied(0));
        assert!(game.is_position_occupied(1));
        assert!(game.is_position_occupied(2));
    }

    #[test]
    fn is_position_occupied_returns_false() {
        let game = Game::new();

        assert!(!game.is_position_occupied(0));
    }

    #[test]
    fn make_move_returns_invalid_position() {
        let mut game = Game::new();

        let result = game.make_move(10);

        assert!(result.is_err_and(|err| err == GameError::InvalidPosition));
    }

    #[test]
    fn make_move_returns_position_occupied() {
        let mut game = Game::from_bitboards(Bitboard::from_bits(0b01), Bitboard::from_bits(0b01));

        let result = game.make_move(1);

        assert!(result.is_err_and(|err| err == GameError::PositionOccupied));
    }

    #[test]
    fn make_move_for_player_x() {
        let mut game = Game::new();

        let _ = game.make_move(1);

        assert!(game.bitboards[0].has(0));
        assert_eq!(game.cell_at(0), Cell::Occupied(Player::X));
    }

    #[test]
    fn make_move_for_player_o() {
        let mut game = Game::new();

        let _ = game.make_move(1);

        game.player_turn = Player::O;

        let _ = game.make_move(2);

        assert!(game.bitboards[1].has(1));
        assert_eq!(game.cell_at(1), Cell::Occupied(Player::O));
    }
}
