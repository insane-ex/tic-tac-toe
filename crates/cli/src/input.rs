use std::{
    io::{self, Write},
    str::FromStr,
};

use super::CliError;

/// # Errors
///
/// - Returns `CliError::Flush` if flushing stdout fails.
/// - Returns `CliError::Read` if reading from stdin fails.
pub fn read(prompt: &str) -> Result<String, CliError> {
    print!("{prompt} ");

    io::stdout().flush().map_err(|_| CliError::Flush)?;

    let mut buffer = String::new();

    io::stdin()
        .read_line(&mut buffer)
        .map_err(|_| CliError::Read)?;

    Ok(buffer.trim().to_string())
}

/// # Errors
///
/// - Returns `CliError::Parse` if input parsing fails.
pub fn parse<T: FromStr>(input: &str) -> Result<T, CliError> {
    let parsed_input = input.trim().parse::<T>().map_err(|_| CliError::Parse)?;

    Ok(parsed_input)
}
