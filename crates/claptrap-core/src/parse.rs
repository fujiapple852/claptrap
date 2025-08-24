use crate::clap_ext::IsManyEx;
use crate::output::{CatCmd, ExitCode, Output, Var};
use crate::types::{Command, ValueParser};
use clap::parser::ValuesRef;
use clap::{ArgAction, ColorChoice};
use std::ffi::OsString;
use std::fmt::Display;

/// Parse the provided arguments and generate output.
///
/// This function does not perform any I/O operations.
#[must_use]
#[expect(clippy::needless_pass_by_value)]
pub fn parse<I, T>(cmd: Command, args: I) -> Output
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let clap_cmd = clap::Command::from(cmd.clone());
    let color = clap_cmd.get_color();
    let help_color = if clap_cmd.is_disable_colored_help_set() {
        ColorChoice::Never
    } else {
        color
    };
    match clap_cmd.clone().try_get_matches_from(args) {
        Ok(matches) => Output::Variables(
            extract_subcommand_path(&matches)
                .into_iter()
                .chain(extract_matches(&cmd, &clap_cmd, &matches, &[]))
                .collect(),
        ),
        Err(err) => match err.kind() {
            clap::error::ErrorKind::DisplayHelp | clap::error::ErrorKind::DisplayVersion => {
                Output::Cat(CatCmd::new(err.render(), ExitCode::Success, help_color))
            }
            clap::error::ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand => {
                Output::Cat(CatCmd::new(err.render(), ExitCode::Usage, help_color))
            }
            _ => Output::Cat(CatCmd::new(err.render(), ExitCode::Error, color)),
        },
    }
}

fn extract_subcommand_path(matches: &clap::ArgMatches) -> Option<Var> {
    let path = std::iter::successors(matches.subcommand(), |&(_, sub)| sub.subcommand())
        .map(|(name, _)| name.to_string())
        .collect::<Vec<_>>();
    (!path.is_empty()).then_some(Var::Subcommand(path))
}

fn extract_matches(
    cmd: &Command,
    clap_cmd: &clap::Command,
    matches: &clap::ArgMatches,
    prefix: &[String],
) -> Vec<Var> {
    let vars = matches.ids().filter_map(|id| {
        let arg = cmd.get_arguments().find(|a| a.get_id() == id.as_str())?;
        let clap_arg = clap_cmd.get_arguments().find(|a| a.get_id() == id)?;
        match clap_arg.get_action() {
            ArgAction::SetTrue | ArgAction::SetFalse => Some(Var::Single(
                prefix.to_vec(),
                id.to_string(),
                matches.get_flag(id.as_str()).to_string(),
            )),
            ArgAction::Count => Some(Var::Single(
                prefix.to_vec(),
                id.to_string(),
                matches.get_count(id.as_str()).to_string(),
            )),
            ArgAction::Append => arg
                .value_parser()
                .and_then(|v| get_many(v, matches, id.as_str()))
                .or_else(|| get_many_raw(matches, id.as_str()))
                .map(|vals| Var::Many(prefix.to_vec(), id.to_string(), vals)),
            ArgAction::Set => {
                if clap_arg.is_many() {
                    arg.value_parser()
                        .and_then(|v| get_many(v, matches, id.as_str()))
                        .or_else(|| get_many_raw(matches, id.as_str()))
                        .map(|vals| Var::Many(prefix.to_vec(), id.to_string(), vals))
                } else {
                    arg.value_parser()
                        .and_then(|v| get_one(v, matches, id.as_str()))
                        .or_else(|| get_one_raw(matches, id.as_str()))
                        .map(|val| Var::Single(prefix.to_vec(), id.to_string(), val))
                }
            }
            _ => None,
        }
    });

    let sub = matches.subcommand().into_iter().flat_map(|(name, sub_m)| {
        cmd.get_subcommands()
            .find(|sc| sc.get_name() == name)
            .into_iter()
            .filter_map(move |sub_cmd| {
                clap_cmd
                    .get_subcommands()
                    .find(|sc| sc.get_name() == name)
                    .map(|clap_sub| {
                        let mut sub_prefix = prefix.to_vec();
                        sub_prefix.push(name.to_string());
                        extract_matches(sub_cmd, clap_sub, sub_m, &sub_prefix)
                    })
            })
            .flatten()
    });

    vars.chain(sub).collect()
}

fn get_one(value_parser: &ValueParser, matches: &clap::ArgMatches, id: &str) -> Option<String> {
    match value_parser {
        ValueParser::Falsey | ValueParser::Boolish | ValueParser::Bool => {
            matches.get_one::<bool>(id).map(ToString::to_string)
        }
        ValueParser::String | ValueParser::PossibleValues(_) => {
            matches.get_one::<String>(id).map(ToString::to_string)
        }
        ValueParser::U8 => matches.get_one::<u8>(id).map(ToString::to_string),
        ValueParser::U16 => matches.get_one::<u16>(id).map(ToString::to_string),
        ValueParser::U32 => matches.get_one::<u32>(id).map(ToString::to_string),
        ValueParser::U64 => matches.get_one::<u64>(id).map(ToString::to_string),
        ValueParser::U128 => matches.get_one::<u128>(id).map(ToString::to_string),
        ValueParser::Usize => matches.get_one::<usize>(id).map(ToString::to_string),
        ValueParser::I8 => matches.get_one::<i8>(id).map(ToString::to_string),
        ValueParser::I16 => matches.get_one::<i16>(id).map(ToString::to_string),
        ValueParser::I32 => matches.get_one::<i32>(id).map(ToString::to_string),
        ValueParser::I64 => matches.get_one::<i64>(id).map(ToString::to_string),
        ValueParser::I128 => matches.get_one::<i128>(id).map(ToString::to_string),
        ValueParser::Isize => matches.get_one::<isize>(id).map(ToString::to_string),
        ValueParser::F32 => matches.get_one::<f32>(id).map(ToString::to_string),
        ValueParser::F64 => matches.get_one::<f64>(id).map(ToString::to_string),
    }
}

fn get_many(
    value_parser: &ValueParser,
    matches: &clap::ArgMatches,
    id: &str,
) -> Option<Vec<String>> {
    fn to_string<'a, T: Display + 'a>(values: ValuesRef<'a, T>) -> Vec<String> {
        values.map(ToString::to_string).collect()
    }
    match value_parser {
        ValueParser::Falsey | ValueParser::Boolish | ValueParser::Bool => {
            matches.get_many::<bool>(id).map(to_string)
        }
        ValueParser::String | ValueParser::PossibleValues(_) => {
            matches.get_many::<String>(id).map(to_string)
        }
        ValueParser::U8 => matches.get_many::<u8>(id).map(to_string),
        ValueParser::U16 => matches.get_many::<u16>(id).map(to_string),
        ValueParser::U32 => matches.get_many::<u32>(id).map(to_string),
        ValueParser::U64 => matches.get_many::<u64>(id).map(to_string),
        ValueParser::U128 => matches.get_many::<u128>(id).map(to_string),
        ValueParser::Usize => matches.get_many::<usize>(id).map(to_string),
        ValueParser::I8 => matches.get_many::<i8>(id).map(to_string),
        ValueParser::I16 => matches.get_many::<i16>(id).map(to_string),
        ValueParser::I32 => matches.get_many::<i32>(id).map(to_string),
        ValueParser::I64 => matches.get_many::<i64>(id).map(to_string),
        ValueParser::I128 => matches.get_many::<i128>(id).map(to_string),
        ValueParser::Isize => matches.get_many::<isize>(id).map(to_string),
        ValueParser::F32 => matches.get_many::<f32>(id).map(to_string),
        ValueParser::F64 => matches.get_many::<f64>(id).map(to_string),
    }
}

fn get_one_raw(matches: &clap::ArgMatches, id: &str) -> Option<String> {
    matches
        .get_raw(id)
        .and_then(|mut values| values.next())
        .map(|value| value.to_string_lossy().into_owned())
}

fn get_many_raw(matches: &clap::ArgMatches, id: &str) -> Option<Vec<String>> {
    matches
        .get_raw(id)
        .map(|values| values.map(|v| v.to_string_lossy().into_owned()).collect())
}
