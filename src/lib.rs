#![forbid(unsafe_code)]

use crate::command::Command;
use crate::output::{CatCmd, ExitCode, Output, Var};
use clap::ArgAction;
use std::ffi::OsString;

pub mod arg;
pub mod arg_group;
pub mod command;
pub mod num_args;
pub mod output;
pub mod style;
pub mod values;

/// Parse the provided arguments and generate output.
///
/// This function does not perform any I/O operations.
#[must_use]
pub fn parse(cmd: Command, args: Vec<OsString>) -> Output {
    let clap_app = clap::Command::from(cmd).no_binary_name(true);
    match clap_app.clone().try_get_matches_from(args) {
        Ok(matches) => Output::Variables(extract_matches(&clap_app, &matches, &mut vec![])),
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

fn extract_matches(
    cmd: &clap::Command,
    matches: &clap::ArgMatches,
    prefix: &mut Vec<String>,
) -> Vec<Var> {
    let local_prefix = prefix.clone();
    let iter_args = matches.ids().filter_map(|id| {
        let arg = cmd.get_arguments().find(|a| a.get_id() == id)?;
        match arg.get_action() {
            ArgAction::SetTrue | ArgAction::SetFalse => Some(Var::Single(
                local_prefix.clone(),
                id.to_string(),
                matches.get_flag(id.as_str()).to_string(),
            )),
            ArgAction::Count => Some(Var::Single(
                local_prefix.clone(),
                id.to_string(),
                matches.get_count(id.as_str()).to_string(),
            )),
            ArgAction::Append => matches.get_raw(id.as_str()).map(|values| {
                Var::Many(
                    local_prefix.clone(),
                    id.to_string(),
                    values.map(|v| v.to_string_lossy().into_owned()).collect(),
                )
            }),
            ArgAction::Set => {
                if arg.is_many() {
                    matches.get_raw(id.as_str()).map(|values| {
                        Var::Many(
                            local_prefix.clone(),
                            id.to_string(),
                            values.map(|v| v.to_string_lossy().into_owned()).collect(),
                        )
                    })
                } else {
                    matches
                        .get_raw(id.as_str())
                        .and_then(|mut values| values.next())
                        .map(|value| {
                            Var::Single(
                                local_prefix.clone(),
                                id.to_string(),
                                value.to_string_lossy().into_owned(),
                            )
                        })
                }
            }
            _ => None,
        }
    });
    let iter_sub = matches
        .subcommand()
        .into_iter()
        .flat_map(|(name, sub_matches)| {
            cmd.get_subcommands()
                .find(|sc| sc.get_name() == name)
                .map(|cmd| {
                    prefix.push(name.to_string());
                    let it = extract_matches(cmd, sub_matches, prefix).into_iter();
                    prefix.pop();
                    it
                })
                .unwrap_or_else(|| vec![].into_iter())
        });
    iter_args.chain(iter_sub).collect()
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
            .or_else(|| self.get_value_delimiter().map(|_| true))
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::{Arg, ArgAction, Command, value_parser};

    #[test]
    fn it_extracts_numeric_values() {
        let cmd = Command::new("prog").arg(
            Arg::new("count")
                .long("count")
                .action(ArgAction::Set)
                .value_parser(value_parser!(u8)),
        );
        let matches = cmd
            .clone()
            .try_get_matches_from(["prog", "--count", "42"])
            .unwrap();
        let vars = extract_matches(&cmd, &matches, &mut Vec::new());
        assert_eq!(
            vars,
            vec![Var::Single(vec![], "count".to_string(), "42".to_string())]
        );
    }
}
