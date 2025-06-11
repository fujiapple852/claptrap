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

/// Parse the provided arguments and generate output.
///
/// This function does not perform any I/O operations.
#[must_use]
pub fn parse(cmd: Command, args: Vec<OsString>) -> Output {
    let clap_app = clap::Command::from(cmd).no_binary_name(true);
    match clap_app.clone().try_get_matches_from(args) {
        Ok(matches) => Output::Variables(extract_matches(&clap_app, &matches, &mut vec![])),
        Err(err) => {
            let err = err.format(&mut clap_app.clone());
            let color_choice = if clap_app.is_disable_colored_help_set() {
                clap::ColorChoice::Never
            } else {
                clap_app.get_color()
            };
            let mut rendered = err.render();
            if matches!(color_choice, clap::ColorChoice::Never) {
                rendered = clap::builder::StyledStr::from(rendered.to_string());
            }
            match err.kind() {
                clap::error::ErrorKind::DisplayHelp | clap::error::ErrorKind::DisplayVersion => {
                    Output::Cat(CatCmd::new(rendered, ExitCode::Success))
                }
                clap::error::ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand => {
                    Output::Cat(CatCmd::new(rendered, ExitCode::Usage))
                }
                _ => Output::Cat(CatCmd::new(rendered, ExitCode::Error)),
            }
        }
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
            ArgAction::Append => matches.get_many::<String>(id.as_str()).map(|values| {
                Var::Many(
                    local_prefix.clone(),
                    id.to_string(),
                    values.into_iter().map(ToOwned::to_owned).collect(),
                )
            }),
            ArgAction::Set => {
                if arg.is_many() {
                    matches.get_many::<String>(id.as_str()).map(|values| {
                        Var::Many(
                            local_prefix.clone(),
                            id.to_string(),
                            values.into_iter().map(ToOwned::to_owned).collect(),
                        )
                    })
                } else {
                    matches.get_one::<String>(id.as_str()).map(|value| {
                        Var::Single(local_prefix.clone(), id.to_string(), value.to_owned())
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
