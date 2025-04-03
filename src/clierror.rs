use std::{error::Error, fmt::Display};

use tracing::subscriber::SetGlobalDefaultError;

#[derive(Debug)]
pub struct CliError(pub String);

impl Error for CliError {}

impl From<chrono::ParseError> for CliError {
    fn from(error: chrono::ParseError) -> Self {
        CliError(format!("failed to parse date : {:?}", error))
    }
}

impl From<confy::ConfyError> for CliError {
    fn from(error: confy::ConfyError) -> Self {
        CliError(format!("failed to load config : {:?}", error))
    }
}

impl From<ctrlc::Error> for CliError {
    fn from(error: ctrlc::Error) -> Self {
        CliError(format!("failed to register ctrlc handler : {:?}", error))
    }
}

impl From<&str> for CliError {
    fn from(error: &str) -> Self {
        CliError(error.to_string())
    }
}

impl From<std::fmt::Error> for CliError {
    fn from(error: std::fmt::Error) -> Self {
        CliError(format!("failed to format string : {:?}", error))
    }
}

impl From<std::io::Error> for CliError {
    fn from(error: std::io::Error) -> Self {
        CliError(format!("failed to read file : {:?}", error))
    }
}

impl From<SetGlobalDefaultError> for CliError {
    fn from(error: SetGlobalDefaultError) -> Self {
        CliError(format!(
            "failed to set global default subscriber : {:?}",
            error
        ))
    }
}

impl Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
