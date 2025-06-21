use clap::builder::StyledStr;
use crc32fast::hash;
use itertools::Itertools;
use std::fmt::Display;

// The prefix for variables
const PREFIX: &str = "claptrap";

// The prefix for the subcommand variable
const SUBCOMMAND_PREFIX: &str = "subcommand";

// The separator for subcommand paths in the variable value
const SUBCOMMAND_VALUE_SEPARATOR: &str = "::";

// The separator for subcommand paths in the variable name
const SUBCOMMAND_PATH_SEPARATOR: &str = "_";

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
    /// The path of a subcommand.
    Subcommand(Vec<String>),
    /// A single variable with a prefix, name, and value.
    Single(Vec<String>, String, String),
    /// A variable with a prefix, name, and multiple values.
    Many(Vec<String>, String, Vec<String>),
}

impl Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Subcommand(path) => {
                let path = shell_quote(&path.iter().join(SUBCOMMAND_VALUE_SEPARATOR));
                write!(f, "{PREFIX}__{SUBCOMMAND_PREFIX}={path}")
            }
            Self::Single(path, name, value) => {
                let value = shell_quote(value);
                if path.is_empty() {
                    write!(f, "{PREFIX}_{name}={value}")
                } else {
                    write!(
                        f,
                        "{}_{}_{}={}",
                        PREFIX,
                        path.iter().format(SUBCOMMAND_PATH_SEPARATOR),
                        name,
                        value
                    )
                }
            }
            Self::Many(path, name, values) => {
                let values = values.iter().map(|v| shell_quote(v)).join(" ");
                if path.is_empty() {
                    write!(f, "{PREFIX}_{name}=({values})")
                } else {
                    write!(
                        f,
                        "{}_{}_{}=({})",
                        PREFIX,
                        path.iter().format(SUBCOMMAND_PATH_SEPARATOR),
                        name,
                        values
                    )
                }
            }
        }
    }
}

fn shell_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\''"))
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
        let msg = format!("{}", self.data.ansi());
        let mut delimiter = format!("EOF_{:08x}", hash(msg.as_bytes()));
        while msg.contains(&delimiter) {
            delimiter.push('_');
        }
        write!(
            f,
            "command cat <<'{}'\n{}{}\nexit {}",
            delimiter, msg, delimiter, self.exit_code
        )
    }
}
