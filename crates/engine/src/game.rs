use super::{bitboard::Bitboard, player::Player};

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
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{Bitboard, Game, Player};

    #[test]
    fn new_game_correctly_sets_game() {
        let game = Game::new();

        assert_eq!(game.bitboards[0], Bitboard::new());
        assert_eq!(game.bitboards[1], Bitboard::new());
        assert_eq!(game.player_turn, Player::X);
    }
}
