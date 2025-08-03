use crate::cli::OutputFormat;
use clap::builder::StyledStr;
use clap::ColorChoice;
use itertools::Itertools;

/// The prefix for variables.
pub const PREFIX: &str = "claptrap";

/// The prefix for the subcommand variable.
pub const SUBCOMMAND_PREFIX: &str = "subcommand";

/// The separator for subcommand paths in the variable value.
pub const SUBCOMMAND_VALUE_SEPARATOR: &str = "::";

/// The separator for subcommand paths in the variable name.
pub const SUBCOMMAND_PATH_SEPARATOR: &str = "_";

/// Represents the output of a claptrap command.
#[derive(Debug, Eq, PartialEq)]
pub enum Output {
    Cat(CatCmd),
    Variables(Vec<Var>),
}

impl Output {
    #[must_use]
    pub fn render(&self, format: OutputFormat) -> String {
        match format {
            OutputFormat::Posix => match self {
                Self::Cat(cmd) => cmd.render_posix(),
                Self::Variables(vars) => vars.iter().map(Var::render_posix).join("\n"),
            },
            OutputFormat::PowerShell => match self {
                Self::Cat(cmd) => cmd.render_powershell(),
                Self::Variables(vars) => vars.iter().map(Var::render_powershell).join("\n"),
            },
        }
    }
}

pub use cat::{CatCmd, ExitCode};
pub use var::Var;

mod var {
    use super::{PREFIX, SUBCOMMAND_PATH_SEPARATOR, SUBCOMMAND_PREFIX, SUBCOMMAND_VALUE_SEPARATOR};
    use itertools::Itertools;

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

    impl Var {
        #[must_use]
        pub(super) fn render_posix(&self) -> String {
            fn quote(value: &str) -> String {
                format!("'{}'", value.replace('\'', "'\\''"))
            }
            match self {
                Self::Subcommand(path) => {
                    let path = quote(&path.iter().join(SUBCOMMAND_VALUE_SEPARATOR));
                    format!("{PREFIX}__{SUBCOMMAND_PREFIX}={path}")
                }
                Self::Single(path, name, value) => {
                    let value = quote(value);
                    if path.is_empty() {
                        format!("{PREFIX}_{name}={value}")
                    } else {
                        format!(
                            "{}_{}_{}={}",
                            PREFIX,
                            path.iter().format(SUBCOMMAND_PATH_SEPARATOR),
                            name,
                            value
                        )
                    }
                }
                Self::Many(path, name, values) => {
                    let values = values.iter().map(|v| quote(v)).join(" ");
                    if path.is_empty() {
                        format!("{PREFIX}_{name}=({values})")
                    } else {
                        format!(
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

        #[must_use]
        pub(super) fn render_powershell(&self) -> String {
            fn quote(value: &str) -> String {
                format!("'{}'", value.replace('\'', "''"))
            }
            match self {
                Self::Subcommand(path) => {
                    let path = path.iter().join(SUBCOMMAND_VALUE_SEPARATOR);
                    let path = quote(&path);
                    format!("$env:{PREFIX}__{SUBCOMMAND_PREFIX} = {path}")
                }
                Self::Single(p, name, value) => {
                    let value = quote(value);
                    if p.is_empty() {
                        format!("$env:{PREFIX}_{name} = {value}")
                    } else {
                        format!(
                            "$env:{}_{}_{} = {}",
                            PREFIX,
                            p.iter().format(SUBCOMMAND_PATH_SEPARATOR),
                            name,
                            value
                        )
                    }
                }
                Self::Many(p, name, values) => {
                    let values = values.iter().map(|v| quote(v)).format(", ");
                    if p.is_empty() {
                        format!("$env:{PREFIX}_{name} = @({values})")
                    } else {
                        format!(
                            "$env:{}_{}_{} = @({})",
                            PREFIX,
                            p.iter().format(SUBCOMMAND_PATH_SEPARATOR),
                            name,
                            values
                        )
                    }
                }
            }
        }
    }
}

mod cat {
    use super::{ColorChoice, StyledStr};
    use crc32fast::hash;
    use std::fmt::Display;

    /// A `cat` command invocation.
    #[derive(Debug, Eq, PartialEq)]
    pub struct CatCmd {
        pub data: StyledStr,
        pub exit_code: ExitCode,
        pub color: ColorChoice,
    }

    impl CatCmd {
        #[must_use]
        pub fn new(cmd: StyledStr, exit_code: ExitCode, color: ColorChoice) -> Self {
            Self {
                data: cmd,
                exit_code,
                color,
            }
        }

        #[cfg(test)]
        #[must_use]
        pub fn render(&self) -> String {
            self.render_posix()
        }

        #[must_use]
        pub(super) fn render_posix(&self) -> String {
            let msg = match self.color {
                ColorChoice::Never => format!("{}", self.data),
                _ => format!("{}", self.data.ansi()),
            };
            let mut delimiter = format!("EOF_{:08x}", hash(msg.as_bytes()));
            while msg.contains(&delimiter) {
                delimiter.push('_');
            }
            format!(
                "command cat <<'{}'\n{}{}\nexit {}",
                delimiter, msg, delimiter, self.exit_code
            )
        }
        #[must_use]
        pub(super) fn render_powershell(&self) -> String {
            let msg = match self.color {
                ColorChoice::Never => format!("{}", self.data),
                _ => format!("{}", self.data.ansi()),
            };
            let msg = Self::ps_quote_double(&msg);
            format!("Write-Output {msg}\nexit {}", self.exit_code)
        }

        fn ps_quote_double(value: &str) -> String {
            format!(
                "@\"\n{}\n\"@",
                value
                    .replace('`', "``")
                    .replace('"', "`\"")
                    .replace('$', "`$")
            )
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
}
