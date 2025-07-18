use crate::types::num_args::NumArgs;
use crate::types::values::ValueParser;
use serde::Deserialize;

/// Represents a command line argument configuration.
#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct Arg {
    #[serde(skip)]
    pub(crate) id: String,
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
    value_parser: Option<ValueParser>,
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
    default_value_if: Option<DefaultIf>,
    default_value_ifs: Option<Vec<DefaultIf>>,
    required_unless_present: Option<String>,
    required_unless_present_all: Option<Vec<String>>,
    required_unless_present_any: Option<Vec<String>>,
    required_if_eq: Option<IfEq>,
    required_if_eq_any: Option<Vec<IfEq>>,
    required_if_eq_all: Option<Vec<IfEq>>,
    requires_if: Option<RequiresIf>,
    requires_ifs: Option<Vec<RequiresIf>>,
    conflicts_with: Option<String>,
    conflicts_with_all: Option<Vec<String>>,
    overrides_with: Option<String>,
    overrides_with_all: Option<Vec<String>>,
}

impl Arg {
    #[must_use]
    pub(crate) fn get_id(&self) -> &str {
        self.id.as_ref()
    }
    #[must_use]
    pub(crate) fn value_parser(&self) -> Option<&ValueParser> {
        self.value_parser.as_ref()
    }
}

#[expect(clippy::cognitive_complexity, clippy::too_many_lines)]
impl From<Arg> for clap::Arg {
    fn from(value: Arg) -> Self {
        let mut arg = Self::new(value.id);
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
        if let Some(value_parser) = value.value_parser {
            arg = arg.value_parser(clap::builder::ValueParser::from(value_parser));
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
            arg = arg.value_hint(clap::ValueHint::from(value_hint));
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
        if let Some(default_if) = value.default_value_if {
            if let Some(default) = default_if.default {
                if let Some(val) = default_if.value {
                    arg = arg.default_value_if(
                        default_if.arg,
                        clap::builder::ArgPredicate::Equals(val.into()),
                        default,
                    );
                } else {
                    arg = arg.default_value_if(
                        default_if.arg,
                        clap::builder::ArgPredicate::IsPresent,
                        default,
                    );
                }
            } else if let Some(val) = default_if.value {
                arg = arg.default_value_if(
                    default_if.arg,
                    clap::builder::ArgPredicate::Equals(val.into()),
                    None,
                );
            } else {
                arg = arg.default_value_if(
                    default_if.arg,
                    clap::builder::ArgPredicate::IsPresent,
                    None,
                );
            }
        }
        if let Some(default_value_ifs) = value.default_value_ifs {
            let with_default = default_value_ifs
                .clone()
                .into_iter()
                .filter_map(|default_value_if| {
                    if let Some(default) = default_value_if.default {
                        if let Some(val) = default_value_if.value {
                            Some((
                                default_value_if.arg,
                                clap::builder::ArgPredicate::Equals(val.into()),
                                default,
                            ))
                        } else {
                            Some((
                                default_value_if.arg,
                                clap::builder::ArgPredicate::IsPresent,
                                default,
                            ))
                        }
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            arg = arg.default_value_ifs(with_default);
            let without_default = default_value_ifs
                .into_iter()
                .filter_map(|default_value_if| {
                    if default_value_if.default.is_none() {
                        if let Some(val) = default_value_if.value {
                            Some((
                                default_value_if.arg,
                                clap::builder::ArgPredicate::Equals(val.into()),
                                None,
                            ))
                        } else {
                            Some((
                                default_value_if.arg,
                                clap::builder::ArgPredicate::IsPresent,
                                None,
                            ))
                        }
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            arg = arg.default_value_ifs(without_default);
        }

        if let Some(required_unless_present) = value.required_unless_present {
            arg = arg.required_unless_present(required_unless_present);
        }
        if let Some(required_unless_present_all) = value.required_unless_present_all {
            arg = arg.required_unless_present_all(required_unless_present_all);
        }
        if let Some(required_unless_present_any) = value.required_unless_present_any {
            arg = arg.required_unless_present_any(required_unless_present_any);
        }
        if let Some(if_eq) = value.required_if_eq {
            arg = arg.required_if_eq(if_eq.arg, if_eq.value);
        }
        if let Some(if_eqs) = value.required_if_eq_any {
            arg = arg.required_if_eq_any(if_eqs.into_iter().map(|v| (v.arg, v.value)));
        }
        if let Some(if_eqs) = value.required_if_eq_all {
            arg = arg.required_if_eq_all(if_eqs.into_iter().map(|v| (v.arg, v.value)));
        }
        if let Some(req_if) = value.requires_if {
            match req_if.value {
                Some(val) => {
                    arg = arg
                        .requires_if(clap::builder::ArgPredicate::Equals(val.into()), req_if.arg);
                }
                None => arg = arg.requires_if(clap::builder::ArgPredicate::IsPresent, req_if.arg),
            }
        }
        if let Some(req_ifs) = value.requires_ifs {
            arg = arg.requires_ifs(req_ifs.into_iter().map(|req_if| match req_if.value {
                Some(val) => (clap::builder::ArgPredicate::Equals(val.into()), req_if.arg),
                None => (clap::builder::ArgPredicate::IsPresent, req_if.arg),
            }));
        }
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

#[derive(Debug, Clone, Default, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
enum ArgAction {
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
            ArgAction::Set => Self::Set,
            ArgAction::Append => Self::Append,
            ArgAction::Count => Self::Count,
            ArgAction::SetTrue => Self::SetTrue,
            ArgAction::SetFalse => Self::SetFalse,
            ArgAction::Help => Self::Help,
            ArgAction::HelpShort => Self::HelpShort,
            ArgAction::HelpLong => Self::HelpLong,
            ArgAction::Version => Self::Version,
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
enum ValueHint {
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
            ValueHint::Unknown => Self::Unknown,
            ValueHint::Other => Self::Other,
            ValueHint::AnyPath => Self::AnyPath,
            ValueHint::FilePath => Self::FilePath,
            ValueHint::DirPath => Self::DirPath,
            ValueHint::ExecutablePath => Self::ExecutablePath,
            ValueHint::CommandName => Self::CommandName,
            ValueHint::CommandString => Self::CommandString,
            ValueHint::CommandWithArguments => Self::CommandWithArguments,
            ValueHint::Username => Self::Username,
            ValueHint::Hostname => Self::Hostname,
            ValueHint::Url => Self::Url,
            ValueHint::EmailAddress => Self::EmailAddress,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
struct DefaultIf {
    arg: String,
    value: Option<String>,
    default: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
struct RequiresIf {
    arg: String,
    value: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
struct IfEq {
    arg: String,
    value: String,
}
