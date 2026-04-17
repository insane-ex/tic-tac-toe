use std::{error, fmt};

#[derive(Debug)]
pub enum CliError {
    Flush,
    Read,
    Parse,
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Flush => "I/O error: failed to flush stdout",
            Self::Read => "I/O error: failed to read from stdin",
            Self::Parse => "Parse error: invalid input",
        })
    }
}

impl error::Error for CliError {}
