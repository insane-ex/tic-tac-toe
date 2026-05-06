use engine::{Game, Player};

#[test]
fn game_flow_player_wins_by_row() {
    let mut game = Game::new();

    game.make_move(1).unwrap();
    game.switch_turn();

    game.make_move(4).unwrap();
    game.switch_turn();

    game.make_move(2).unwrap();
    game.switch_turn();

    game.make_move(5).unwrap();
    game.switch_turn();

    game.make_move(3).unwrap();

    assert!(game.has_winner());
    assert!(game.is_over());
}

#[test]
fn game_flow_player_wins_by_column() {
    let mut game = Game::new();

    game.make_move(1).unwrap();
    game.switch_turn();

    game.make_move(2).unwrap();
    game.switch_turn();

    game.make_move(4).unwrap();
    game.switch_turn();

    game.make_move(3).unwrap();
    game.switch_turn();

    game.make_move(7).unwrap();

    assert!(game.has_winner());
    assert!(game.is_over());
}

#[test]
fn game_flow_player_wins_by_diagonal() {
    let mut game = Game::new();

    game.make_move(1).unwrap();
    game.switch_turn();

    game.make_move(2).unwrap();
    game.switch_turn();

    game.make_move(5).unwrap();
    game.switch_turn();

    game.make_move(3).unwrap();
    game.switch_turn();

    game.make_move(9).unwrap();

    assert!(game.has_winner());
    assert!(game.is_over());
}

#[test]
fn game_flow_ends_in_draw() {
    let mut game = Game::new();

    let moves = [1, 2, 3, 5, 4, 6, 8, 7, 9];

    for (i, m) in moves.iter().enumerate() {
        game.make_move(*m).unwrap();

        if i < moves.len() - 1 {
            game.switch_turn();
        }
    }

    assert!(game.is_draw());
    assert!(game.is_over());
}

#[test]
fn game_flow_cannot_play_after_end() {
    let mut game = Game::new();

    game.make_move(1).unwrap();
    game.switch_turn();
    game.make_move(4).unwrap();
    game.switch_turn();
    game.make_move(2).unwrap();
    game.switch_turn();
    game.make_move(5).unwrap();
    game.switch_turn();
    game.make_move(3).unwrap();

    let result = game.make_move(6);

    assert!(result.is_err());
}

#[test]
fn game_flow_turns_alternate() {
    let mut game = Game::new();

    assert_eq!(game.current_player(), &Player::X);

    game.make_move(1).unwrap();
    game.switch_turn();

    assert_eq!(game.current_player(), &Player::O);
}
