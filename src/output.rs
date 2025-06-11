use clap::builder::StyledStr;
use itertools::Itertools;
use std::fmt::Display;

// The prefix for variables output by claptrap
const PREFIX: &str = "claptrap";

/// Represents the output of a claptrap command.
#[derive(Debug, Eq, PartialEq)]
pub enum Output {
    Cat(CatCmd),
    Variables(Vec<Var>),
}

impl Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Output::Cat(cmd) => write!(f, "{cmd}"),
            Output::Variables(vars) => write!(f, "{}", vars.iter().format("\n")),
        }
    }
}

/// Represents a variable output by claptrap.
#[derive(Debug, Eq, PartialEq)]
pub enum Var {
    Single(String, String),
    Many(String, Vec<String>),
}

impl Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Var::Single(name, value) => write!(f, "{}_{}={}", PREFIX, name, value),
            Var::Many(name, values) => {
                write!(f, "{}_{}=({})", PREFIX, name, values.iter().join(" "))
            }
        }
    }
}

/// Exit code for the `CatCmd`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExitCode {
    Success = 0,
    Error = 1,
    Usage = 2,
    Panic = 3,
}

impl Display for ExitCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

/// A `cat` command invocation.
#[derive(Debug, Eq, PartialEq)]
pub struct CatCmd {
    pub data: StyledStr,
    pub exit_code: ExitCode,
}

impl CatCmd {
    pub fn new(cmd: StyledStr, exit_code: ExitCode) -> Self {
        Self {
            data: cmd,
            exit_code,
        }
    }
}

impl Display for CatCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "command cat <<'EOF'\n{}EOF\nexit {}",
            self.data.ansi(),
            self.exit_code
        )
    }
}
