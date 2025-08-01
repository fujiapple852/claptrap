use crate::output::ExitCode;
use crate::{CatCmd, Output};
use clap::builder::StyledStr;
use clap::ColorChoice;
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

impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Self {
        Self(Output::Cat(CatCmd::new(
            StyledStr::from(format!("{err}\n")),
            ExitCode::Error,
            ColorChoice::Auto,
        )))
    }
}
