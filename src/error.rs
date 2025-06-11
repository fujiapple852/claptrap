use crate::{CatCmd, Output};
use clap::builder::StyledStr;
use claptrap::output::ExitCode;
use std::fmt::{Display, Formatter};

pub type Result<T> = std::result::Result<T, Error>;

/// Represents an error that can occur in claptrap.
///
/// This error type wraps an `Output` which is eval-safe.
#[derive(Debug)]
pub struct Error(Output);

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error(Output::Cat(CatCmd::new(
            StyledStr::from(format!("{err}\n")),
            ExitCode::Error,
        )))
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Error(Output::Cat(CatCmd::new(
            StyledStr::from(format!("{err}\n")),
            ExitCode::Error,
        )))
    }
}
