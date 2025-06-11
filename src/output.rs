use clap::builder::StyledStr;
use itertools::Itertools;

fn shell_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\''"))
}
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
            Self::Cat(cmd) => write!(f, "{cmd}"),
            Self::Variables(vars) => write!(f, "{}", vars.iter().format("\n")),
        }
    }
}

/// Represents a variable output by claptrap.
#[derive(Debug, Eq, PartialEq)]
pub enum Var {
    Single(Vec<String>, String, String),
    Many(Vec<String>, String, Vec<String>),
}

impl Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Single(prefix, name, value) => {
                let value = shell_quote(value);
                if prefix.is_empty() {
                    write!(f, "{PREFIX}_{name}={value}")
                } else {
                    write!(
                        f,
                        "{}_{}_{}={}",
                        PREFIX,
                        prefix.iter().format("_"),
                        name,
                        value
                    )
                }
            }
            Self::Many(prefix, name, values) => {
                let values = values.iter().map(|v| shell_quote(v)).join(" ");
                if prefix.is_empty() {
                    write!(f, "{PREFIX}_{name}=({values})")
                } else {
                    write!(
                        f,
                        "{}_{}_{}=({})",
                        PREFIX,
                        prefix.iter().format("_"),
                        name,
                        values
                    )
                }
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
    #[must_use]
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
