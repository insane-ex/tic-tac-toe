use engine::game::Game;

use super::{error::CliError, input};

/// # Errors
///
/// - Returns `CliError::Flush` if flushing stdout fails.
/// - Returns `CliError::Read` if reading from stdin fails.
/// - Returns `CliError::Parse` if parsing the input fails.
pub fn run() -> Result<(), CliError> {
    let mut game = Game::default();

    loop {
        game.print_board();

        let raw_input = input::read("Enter a board position (1-9):")?;
        let position = input::parse::<u16>(&raw_input)?;

        if let Err(err) = game.make_move(position) {
            eprintln!("{err}");
        }

        if game.is_draw() {
            game.print_board();

            println!("It's a draw!");

            break;
        }
    }

    Ok(())
}
