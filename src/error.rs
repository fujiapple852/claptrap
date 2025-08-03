use crate::cli::OutputFormat;
use crate::output::ExitCode;
use crate::{CatCmd, Output};
use clap::builder::StyledStr;
use clap::ColorChoice;

pub type Result<T> = std::result::Result<T, Error>;

/// Represents an error that can occur in claptrap.
///
/// This error type wraps an `Output` which is eval-safe.
///
/// This does not impl the `std::error::Error` trait, as it cannot be rendered with the `Display` trait.
/// Instead, it provides a method to render the error in a specific output format.
#[derive(Debug)]
pub struct Error(Output);

impl Error {
    #[must_use]
    pub fn render(&self, format: OutputFormat) -> String {
        self.0.render(format)
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
