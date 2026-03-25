use std::{
    io::{self, Write},
    str,
};

pub struct Input;

impl Input {
    fn read_line(prompt: &str) -> io::Result<String> {
        print!("{prompt} ");

        io::stdout().flush()?;

        let mut buffer = String::new();

        io::stdin().read_line(&mut buffer)?;

        Ok(buffer.trim().to_string())
    }

    /// Reads a line from the user and parsed it to `T`.
    ///
    /// # Errors
    /// Returns an error if reading from stdin fails or if parsing the input fails.
    pub fn read_parsed<T: str::FromStr>(prompt: &str) -> io::Result<T> {
        let input = Self::read_line(prompt)?;

        input
            .parse()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid input"))
    }
}
