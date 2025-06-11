use crate::num_args::NumArgs;
use serde::Deserialize;
use std::str::FromStr;

/// Represents a named argument with its associated `Arg` configuration.
///
/// This struct is used to hold the name of the argument and its configuration,
/// which is represented by the `Arg` struct.
#[derive(Debug)]
pub struct NamedArg {
    pub name: String,
    pub arg: Arg,
}

impl NamedArg {
    pub fn new(name: String, arg: Arg) -> Self {
        Self { name, arg }
    }
}

/// Represents a command line argument configuration.
#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub struct Arg {
    id: Option<String>,
    short: Option<char>,
    long: Option<String>,
    alias: Option<String>,
    short_alias: Option<char>,
    aliases: Option<Vec<String>>,
    short_aliases: Option<Vec<char>>,
    visible_alias: Option<String>,
    visible_short_alias: Option<char>,
    visible_aliases: Option<Vec<String>>,
    visible_short_aliases: Option<Vec<char>>,
    index: Option<usize>,
    trailing_var_arg: Option<bool>,
    last: Option<bool>,
    required: Option<bool>,
    requires: Option<String>,
    exclusive: Option<bool>,
    global: Option<bool>,
    action: Option<ArgAction>,
    value_parser: Option<Vec<String>>,
    num_args: Option<NumArgs>,
    value_name: Option<String>,
    value_names: Option<Vec<String>>,
    value_hint: Option<ValueHint>,
    ignore_case: Option<bool>,
    allow_hyphen_values: Option<bool>,
    allow_negative_numbers: Option<bool>,
    require_equals: Option<bool>,
    value_delimiter: Option<char>,
    value_terminator: Option<String>,
    raw: Option<bool>,
    default_value: Option<String>,
    default_values: Option<Vec<String>>,
    default_missing_value: Option<String>,
    default_missing_value_os: Option<String>,
    default_missing_values: Option<Vec<String>>,
    default_missing_values_os: Option<Vec<String>>,
    env: Option<String>,
    help: Option<String>,
    long_help: Option<String>,
    display_order: Option<usize>,
    help_heading: Option<String>,
    next_line_help: Option<bool>,
    hide: Option<bool>,
    hide_possible_values: Option<bool>,
    hide_default_value: Option<bool>,
    hide_env: Option<bool>,
    hide_env_values: Option<bool>,
    hide_short_help: Option<bool>,
    hide_long_help: Option<bool>,
    group: Option<String>,
    groups: Option<Vec<String>>,
    // default_value_if // TODO
    // default_value_ifs // TODO
    required_unless_present: Option<String>,
    required_unless_present_all: Option<Vec<String>>,
    required_unless_present_any: Option<Vec<String>>,
    // required_if_eq // TODO
    // required_if_eq_any // TODO
    // required_if_eq_all // TODO
    // requires_if // TODO
    // requires_ifs // TODO
    conflicts_with: Option<String>,
    conflicts_with_all: Option<Vec<String>>,
    overrides_with: Option<String>,
    overrides_with_all: Option<Vec<String>>,
    /// Non-standard option
    typed_value_parser: Option<String>,
}

impl From<NamedArg> for clap::Arg {
    fn from(named_arg: NamedArg) -> Self {
        let value = named_arg.arg;
        let mut arg = clap::Arg::new(named_arg.name);
        if let Some(id) = value.id {
            arg = arg.id(id);
        }
        // TODO: test if the yaml contains a string
        if let Some(short) = value.short {
            arg = arg.short(short);
        }
        if let Some(long) = value.long {
            arg = arg.long(long);
        }
        if let Some(alias) = value.alias {
            arg = arg.alias(alias);
        }
        if let Some(short_alias) = value.short_alias {
            arg = arg.short_alias(short_alias);
        }
        if let Some(aliases) = value.aliases {
            arg = arg.aliases(aliases);
        }
        if let Some(short_aliases) = value.short_aliases {
            arg = arg.short_aliases(short_aliases);
        }
        if let Some(visible_alias) = value.visible_alias {
            arg = arg.visible_alias(visible_alias);
        }
        if let Some(visible_short_alias) = value.visible_short_alias {
            arg = arg.visible_short_alias(visible_short_alias);
        }
        if let Some(visible_aliases) = value.visible_aliases {
            arg = arg.visible_aliases(visible_aliases);
        }
        if let Some(visible_short_aliases) = value.visible_short_aliases {
            arg = arg.visible_short_aliases(visible_short_aliases);
        }
        // TODO can panic, check clap docs and add tests
        if let Some(index) = value.index {
            arg = arg.index(index);
        }
        if let Some(trailing_var_arg) = value.trailing_var_arg {
            arg = arg.trailing_var_arg(trailing_var_arg);
        }
        if let Some(last) = value.last {
            arg = arg.last(last);
        }
        if let Some(required) = value.required {
            arg = arg.required(required);
        }
        if let Some(requires) = value.requires {
            arg = arg.requires(requires);
        }
        if let Some(exclusive) = value.exclusive {
            arg = arg.exclusive(exclusive);
        }
        if let Some(global) = value.global {
            arg = arg.global(global);
        }
        if let Some(action) = value.action {
            arg = arg.action(clap::ArgAction::from(action));
        }

        match (value.value_parser, value.typed_value_parser) {
            (Some(_), Some(_)) => {
                panic!(
                    "value_parser and typed_value_parser are mutually exclusive. Use one or the other."
                );
            }
            (Some(value_parser), None) => {
                arg = arg.value_parser(value_parser);
            }
            (None, Some(typed_value_parser)) => {
                let value_parser = match TypedValueParser::from_str(&typed_value_parser).unwrap() {
                    TypedValueParser::Bool => {
                        clap::builder::ValueParser::new(clap::builder::BoolValueParser::new())
                    }
                    TypedValueParser::Boolish => {
                        clap::builder::ValueParser::new(clap::builder::BoolishValueParser::new())
                    }
                    TypedValueParser::Falsey => {
                        clap::builder::ValueParser::new(clap::builder::FalseyValueParser::new())
                    }
                };
                arg = arg.value_parser(value_parser);
            }
            _ => {}
        }
        if let Some(num_args) = value.num_args {
            arg = arg.num_args(clap::builder::ValueRange::new(
                clap::builder::ValueRange::from(num_args),
            ));
        }
        if let Some(value_name) = value.value_name {
            arg = arg.value_name(value_name);
        }
        if let Some(value_names) = value.value_names {
            arg = arg.value_names(value_names);
        }
        if let Some(value_hint) = value.value_hint {
            arg = arg.value_hint(clap::ValueHint::from(value_hint))
        }
        if let Some(ignore_case) = value.ignore_case {
            arg = arg.ignore_case(ignore_case);
        }
        if let Some(allow_hyphen_values) = value.allow_hyphen_values {
            arg = arg.allow_hyphen_values(allow_hyphen_values);
        }
        if let Some(allow_negative_numbers) = value.allow_negative_numbers {
            arg = arg.allow_negative_numbers(allow_negative_numbers);
        }
        if let Some(require_equals) = value.require_equals {
            arg = arg.require_equals(require_equals);
        }
        if let Some(value_delimiter) = value.value_delimiter {
            arg = arg.value_delimiter(value_delimiter);
        }
        if let Some(value_terminator) = value.value_terminator {
            arg = arg.value_terminator(value_terminator);
        }
        if let Some(raw) = value.raw {
            arg = arg.raw(raw);
        }
        if let Some(default_value) = value.default_value {
            arg = arg.default_value(default_value);
        }
        if let Some(default_values) = value.default_values {
            arg = arg.default_values(default_values);
        }
        if let Some(default_missing_value) = value.default_missing_value {
            arg = arg.default_missing_value(default_missing_value);
        }
        if let Some(default_missing_value_os) = value.default_missing_value_os {
            arg = arg.default_missing_value_os(default_missing_value_os);
        }
        if let Some(default_missing_values) = value.default_missing_values {
            arg = arg.default_missing_values(default_missing_values);
        }
        if let Some(default_missing_values_os) = value.default_missing_values_os {
            arg = arg.default_missing_values_os(default_missing_values_os);
        }
        if let Some(env) = value.env {
            arg = arg.env(env);
        }
        if let Some(help) = value.help {
            arg = arg.help(help);
        }
        if let Some(long_help) = value.long_help {
            arg = arg.long_help(long_help);
        }
        if let Some(display_order) = value.display_order {
            arg = arg.display_order(display_order);
        }
        if let Some(help_heading) = value.help_heading {
            arg = arg.help_heading(help_heading);
        }
        if let Some(next_line_help) = value.next_line_help {
            arg = arg.next_line_help(next_line_help);
        }
        if let Some(hide) = value.hide {
            arg = arg.hide(hide);
        }
        if let Some(hide_possible_values) = value.hide_possible_values {
            arg = arg.hide_possible_values(hide_possible_values);
        }
        if let Some(hide_default_value) = value.hide_default_value {
            arg = arg.hide_default_value(hide_default_value);
        }
        if let Some(hide_env) = value.hide_env {
            arg = arg.hide_env(hide_env);
        }
        if let Some(hide_env_values) = value.hide_env_values {
            arg = arg.hide_env_values(hide_env_values);
        }
        if let Some(hide_short_help) = value.hide_short_help {
            arg = arg.hide_short_help(hide_short_help);
        }
        if let Some(hide_long_help) = value.hide_long_help {
            arg = arg.hide_long_help(hide_long_help);
        }
        if let Some(group) = value.group {
            arg = arg.group(group);
        }
        if let Some(groups) = value.groups {
            arg = arg.groups(groups);
        }
        // TODO: default_value_if
        // TODO: default_value_ifs
        if let Some(required_unless_present) = value.required_unless_present {
            arg = arg.required_unless_present(required_unless_present);
        }
        if let Some(required_unless_present_all) = value.required_unless_present_all {
            arg = arg.required_unless_present_all(required_unless_present_all);
        }
        if let Some(required_unless_present_any) = value.required_unless_present_any {
            arg = arg.required_unless_present_any(required_unless_present_any);
        }
        // TODO: required_if_eq
        // TODO: required_if_eq_any
        // TODO: required_if_eq_all
        // TODO: requires_if
        // TODO: requires_ifs
        if let Some(conflicts_with) = value.conflicts_with {
            arg = arg.conflicts_with(conflicts_with);
        }
        if let Some(conflicts_with_all) = value.conflicts_with_all {
            arg = arg.conflicts_with_all(conflicts_with_all);
        }
        if let Some(overrides_with) = value.overrides_with {
            arg = arg.overrides_with(overrides_with);
        }
        if let Some(overrides_with_all) = value.overrides_with_all {
            arg = arg.overrides_with_all(overrides_with_all);
        }
        arg
    }
}

#[derive(Debug, Deserialize, Default, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum ArgAction {
    #[default]
    Set,
    Append,
    Count,
    SetTrue,
    SetFalse,
    Help,
    HelpShort,
    HelpLong,
    Version,
}

impl From<ArgAction> for clap::ArgAction {
    fn from(action: ArgAction) -> Self {
        match action {
            ArgAction::Set => clap::ArgAction::Set,
            ArgAction::Append => clap::ArgAction::Append,
            ArgAction::Count => clap::ArgAction::Count,
            ArgAction::SetTrue => clap::ArgAction::SetTrue,
            ArgAction::SetFalse => clap::ArgAction::SetFalse,
            ArgAction::Help => clap::ArgAction::Help,
            ArgAction::HelpShort => clap::ArgAction::HelpShort,
            ArgAction::HelpLong => clap::ArgAction::HelpLong,
            ArgAction::Version => clap::ArgAction::Version,
        }
    }
}

#[derive(Debug, Deserialize, Default, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum ValueHint {
    #[default]
    Unknown,
    Other,
    AnyPath,
    FilePath,
    DirPath,
    ExecutablePath,
    CommandName,
    CommandString,
    CommandWithArguments,
    Username,
    Hostname,
    Url,
    EmailAddress,
}

impl From<ValueHint> for clap::ValueHint {
    fn from(value_hint: ValueHint) -> Self {
        match value_hint {
            ValueHint::Unknown => clap::ValueHint::Unknown,
            ValueHint::Other => clap::ValueHint::Other,
            ValueHint::AnyPath => clap::ValueHint::AnyPath,
            ValueHint::FilePath => clap::ValueHint::FilePath,
            ValueHint::DirPath => clap::ValueHint::DirPath,
            ValueHint::ExecutablePath => clap::ValueHint::ExecutablePath,
            ValueHint::CommandName => clap::ValueHint::CommandName,
            ValueHint::CommandString => clap::ValueHint::CommandString,
            ValueHint::CommandWithArguments => clap::ValueHint::CommandWithArguments,
            ValueHint::Username => clap::ValueHint::Username,
            ValueHint::Hostname => clap::ValueHint::Hostname,
            ValueHint::Url => clap::ValueHint::Url,
            ValueHint::EmailAddress => clap::ValueHint::EmailAddress,
        }
    }
}

#[derive(Debug, Clone, strum::EnumString)]
#[strum(serialize_all = "kebab-case")]
enum TypedValueParser {
    Bool,
    Boolish,
    Falsey,
}
