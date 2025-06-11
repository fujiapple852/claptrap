use crate::command::Command;
use crate::output::{CatCmd, ExitCode, Output, Var};
use clap::ArgAction;
use std::ffi::OsString;

pub mod command;
pub mod output;

/// Parse the provided arguments and generate output.
///
/// This function does not perform any I/O operations.
pub fn parse(cmd: Command, args: Vec<OsString>) -> Output {
    let clap_app = clap::Command::from(cmd).no_binary_name(true);
    match clap_app.clone().try_get_matches_from(args) {
        Ok(matches) => Output::Variables(extract_matches(clap_app, matches)),
        Err(err) => match err.kind() {
            clap::error::ErrorKind::DisplayHelp | clap::error::ErrorKind::DisplayVersion => {
                Output::Cat(CatCmd::new(err.render(), ExitCode::Success))
            }
            clap::error::ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand => {
                Output::Cat(CatCmd::new(err.render(), ExitCode::Usage))
            }
            _ => Output::Cat(CatCmd::new(err.render(), ExitCode::Error)),
        },
    }
}

// TODO: subcommand and groups
fn extract_matches(cmd: clap::Command, matches: clap::ArgMatches) -> Vec<Var> {
    matches
        .ids()
        .filter_map(|id| {
            let arg = cmd.get_arguments().find(|a| a.get_id() == id)?;
            match arg.get_action() {
                ArgAction::SetTrue | ArgAction::SetFalse => Some(Var::Single(
                    id.to_string(),
                    matches.get_flag(id.as_str()).to_string(),
                )),
                ArgAction::Count => Some(Var::Single(
                    id.to_string(),
                    matches.get_count(id.as_str()).to_string(),
                )),
                ArgAction::Append => matches.get_many::<String>(id.as_str()).map(|values| {
                    Var::Many(
                        id.to_string(),
                        values.into_iter().map(|value| value.to_owned()).collect(),
                    )
                }),
                ArgAction::Set => {
                    if arg.is_many() {
                        matches.get_many::<String>(id.as_str()).map(|values| {
                            Var::Many(
                                id.to_string(),
                                values.into_iter().map(|value| value.to_owned()).collect(),
                            )
                        })
                    } else {
                        matches
                            .get_one::<String>(id.as_str())
                            .map(|value| Var::Single(id.to_string(), value.to_owned()))
                    }
                }
                _ => None,
            }
        })
        .collect()
}

/// Extension trait for `clap::Arg` to determine if it is many-valued.
trait IsManyEx {
    /// Returns true if the argument is many-valued.
    fn is_many(&self) -> bool;
}

impl IsManyEx for clap::Arg {
    fn is_many(&self) -> bool {
        self.get_num_args()
            .map(|r| r.max_values() > 1)
            .or(self.get_value_delimiter().map(|_| true))
            .unwrap_or(false)
    }
}
