use engine::game::Game;

use super::{error::CliError, input};

/// # Errors
///
/// - Returns `CliError::Flush` if flushing stdout fails.
/// - Returns `CliError::Read` if reading from stdin fails.
/// - Returns `CliError::Parse` if parsing the input fails.
pub fn run() -> Result<(), CliError> {
    let input = input::read("Enter a non-negative number:")?;
    let parsed_input = input::parse::<u16>(&input)?;

    println!("You entered {parsed_input}.");

    let game = Game::default();

    game.print_board();

    Ok(())
}
