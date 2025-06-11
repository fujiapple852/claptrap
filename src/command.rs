use indexmap::IndexMap;
use serde::Deserialize;
use std::str::FromStr;

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub struct Command {
    name: String,
    args: IndexMap<String, Arg>,
    ignore_errors: Option<bool>,
    args_override_self: Option<bool>,
    dont_delimit_trailing_values: Option<bool>,
    color: Option<ColorChoice>,
    // TODO styles
    term_width: Option<usize>,
    max_term_width: Option<usize>,
    disable_version_flag: Option<bool>,
    propagate_version: Option<bool>,
    next_line_help: Option<bool>,
    disable_help_flag: Option<bool>,
    disable_help_subcommand: Option<bool>,
    disable_colored_help: Option<bool>,
    help_expected: Option<bool>,
    hide_possible_values: Option<bool>,
    infer_long_args: Option<bool>,
    infer_subcommands: Option<bool>,
    bin_name: Option<String>,
    display_name: Option<String>,
    author: Option<String>,
    about: Option<String>,
    long_about: Option<String>,
    after_help: Option<String>,
    after_long_help: Option<String>,
    before_help: Option<String>,
    before_long_help: Option<String>,
    version: Option<String>,
    long_version: Option<String>,
    override_usage: Option<String>,
    override_help: Option<String>,
    help_template: Option<String>,
    flatten_help: Option<bool>,
    next_help_heading: Option<String>,
    next_display_order: Option<usize>,
    arg_required_else_help: Option<bool>, // TODO default to true (opinionated!)
    allow_missing_positional: Option<bool>,
    short_flag: Option<char>,
    long_flag: Option<String>,
    alias: Option<String>,
    short_flag_alias: Option<char>,
    long_flag_alias: Option<String>,
    aliases: Option<Vec<String>>,
    short_flag_aliases: Option<Vec<char>>,
    long_flag_aliases: Option<Vec<String>>,
    visible_alias: Option<String>,
    visible_short_flag_alias: Option<char>,
    visible_long_flag_alias: Option<String>,
    visible_aliases: Option<Vec<String>>,
    visible_short_flag_aliases: Option<Vec<char>>,
    visible_long_flag_aliases: Option<Vec<String>>,
    display_order: Option<usize>,
    hide: Option<bool>,
    subcommand_required: Option<bool>,
    allow_external_subcommands: Option<bool>,
    // TODO external_subcommand_value_parser: Option<String>,
    args_conflicts_with_subcommands: Option<bool>,
    subcommand_precedence_over_arg: Option<bool>,
    subcommand_negates_reqs: Option<bool>,
    multicall: Option<bool>,
    subcommand_value_name: Option<String>,
    subcommand_help_heading: Option<String>,
}

impl From<Command> for clap::Command {
    fn from(cmd: Command) -> Self {
        let mut command = clap::Command::new(cmd.name);
        command = command.args(
            cmd.args
                .into_iter()
                .map(|(name, arg)| clap::Arg::from(NamedArg::new(name, arg)))
                .collect::<Vec<_>>(),
        );
        // TODO: group
        // TODO: groups
        // TODO: subcommand
        // TODO: subcommands
        // TODO: error
        if let Some(ignore_errors) = cmd.ignore_errors {
            command = command.ignore_errors(ignore_errors);
        }
        if let Some(args_override_self) = cmd.args_override_self {
            command = command.args_override_self(args_override_self);
        }
        if let Some(dont_delimit_trailing_values) = cmd.dont_delimit_trailing_values {
            command = command.dont_delimit_trailing_values(dont_delimit_trailing_values);
        }
        if let Some(color) = cmd.color {
            command = command.color(clap::ColorChoice::from(color));
        }
        // TODO: styles
        if let Some(term_width) = cmd.term_width {
            command = command.term_width(term_width);
        }
        if let Some(max_term_width) = cmd.max_term_width {
            command = command.max_term_width(max_term_width);
        }
        if let Some(disable_version_flag) = cmd.disable_version_flag {
            command = command.disable_version_flag(disable_version_flag);
        }
        if let Some(propagate_version) = cmd.propagate_version {
            command = command.propagate_version(propagate_version);
        }
        if let Some(next_line_help) = cmd.next_line_help {
            command = command.next_line_help(next_line_help);
        }
        if let Some(disable_help_flag) = cmd.disable_help_flag {
            command = command.disable_help_flag(disable_help_flag);
        }
        if let Some(disable_help_subcommand) = cmd.disable_help_subcommand {
            command = command.disable_help_subcommand(disable_help_subcommand);
        }
        if let Some(disable_colored_help) = cmd.disable_colored_help {
            command = command.disable_colored_help(disable_colored_help);
        }
        if let Some(help_expected) = cmd.help_expected {
            command = command.help_expected(help_expected);
        }
        if let Some(hide_possible_values) = cmd.hide_possible_values {
            command = command.hide_possible_values(hide_possible_values);
        }
        if let Some(infer_long_args) = cmd.infer_long_args {
            command = command.infer_long_args(infer_long_args);
        }
        if let Some(infer_subcommands) = cmd.infer_subcommands {
            command = command.infer_subcommands(infer_subcommands);
        }
        if let Some(bin_name) = cmd.bin_name {
            command = command.bin_name(bin_name);
        }
        if let Some(display_name) = cmd.display_name {
            command = command.display_name(display_name);
        }
        if let Some(author) = cmd.author {
            command = command.author(author);
        }
        if let Some(about) = cmd.about {
            command = command.about(about);
        }
        if let Some(long_about) = cmd.long_about {
            command = command.long_about(long_about);
        }
        if let Some(after_help) = cmd.after_help {
            command = command.after_help(after_help);
        }
        if let Some(after_long_help) = cmd.after_long_help {
            command = command.after_long_help(after_long_help);
        }
        if let Some(before_help) = cmd.before_help {
            command = command.before_help(before_help);
        }
        if let Some(before_long_help) = cmd.before_long_help {
            command = command.before_long_help(before_long_help);
        }
        if let Some(version) = cmd.version {
            command = command.version(version);
        }
        if let Some(long_version) = cmd.long_version {
            command = command.long_version(long_version);
        }
        if let Some(override_usage) = cmd.override_usage {
            command = command.override_usage(override_usage);
        }
        if let Some(override_help) = cmd.override_help {
            command = command.override_help(override_help);
        }
        if let Some(help_template) = cmd.help_template {
            command = command.help_template(help_template);
        }
        if let Some(flatten_help) = cmd.flatten_help {
            command = command.flatten_help(flatten_help);
        }
        if let Some(next_help_heading) = cmd.next_help_heading {
            command = command.next_help_heading(next_help_heading);
        }
        if let Some(next_display_order) = cmd.next_display_order {
            command = command.next_display_order(next_display_order);
        }
        if let Some(arg_required_else_help) = cmd.arg_required_else_help {
            command = command.arg_required_else_help(arg_required_else_help);
        }
        if let Some(allow_missing_positional) = cmd.allow_missing_positional {
            command = command.allow_missing_positional(allow_missing_positional);
        }
        if let Some(short_flag) = cmd.short_flag {
            command = command.short_flag(short_flag);
        }
        if let Some(long_flag) = cmd.long_flag {
            command = command.long_flag(long_flag);
        }
        if let Some(alias) = cmd.alias {
            command = command.alias(alias);
        }
        if let Some(short_flag_alias) = cmd.short_flag_alias {
            command = command.short_flag_alias(short_flag_alias);
        }
        if let Some(long_flag_alias) = cmd.long_flag_alias {
            command = command.long_flag_alias(long_flag_alias);
        }
        if let Some(aliases) = cmd.aliases {
            command = command.aliases(aliases);
        }
        if let Some(short_flag_aliases) = cmd.short_flag_aliases {
            command = command.short_flag_aliases(short_flag_aliases);
        }
        if let Some(long_flag_aliases) = cmd.long_flag_aliases {
            command = command.long_flag_aliases(long_flag_aliases);
        }
        if let Some(visible_alias) = cmd.visible_alias {
            command = command.visible_alias(visible_alias);
        }
        if let Some(visible_short_flag_alias) = cmd.visible_short_flag_alias {
            command = command.visible_short_flag_alias(visible_short_flag_alias);
        }
        if let Some(visible_long_flag_alias) = cmd.visible_long_flag_alias {
            command = command.visible_long_flag_alias(visible_long_flag_alias);
        }
        if let Some(visible_aliases) = cmd.visible_aliases {
            command = command.visible_aliases(visible_aliases);
        }
        if let Some(visible_short_flag_aliases) = cmd.visible_short_flag_aliases {
            command = command.visible_short_flag_aliases(visible_short_flag_aliases);
        }
        if let Some(visible_long_flag_aliases) = cmd.visible_long_flag_aliases {
            command = command.visible_long_flag_aliases(visible_long_flag_aliases);
        }
        if let Some(display_order) = cmd.display_order {
            command = command.display_order(display_order);
        }
        if let Some(hide) = cmd.hide {
            command = command.hide(hide);
        }
        if let Some(subcommand_required) = cmd.subcommand_required {
            command = command.subcommand_required(subcommand_required);
        }
        if let Some(allow_external_subcommands) = cmd.allow_external_subcommands {
            command = command.allow_external_subcommands(allow_external_subcommands);
        }
        // TODO: external_subcommand_value_parser
        if let Some(args_conflicts_with_subcommands) = cmd.args_conflicts_with_subcommands {
            command = command.args_conflicts_with_subcommands(args_conflicts_with_subcommands);
        }
        if let Some(subcommand_precedence_over_arg) = cmd.subcommand_precedence_over_arg {
            command = command.subcommand_precedence_over_arg(subcommand_precedence_over_arg);
        }
        if let Some(subcommand_negates_reqs) = cmd.subcommand_negates_reqs {
            command = command.subcommand_negates_reqs(subcommand_negates_reqs);
        }
        if let Some(multicall) = cmd.multicall {
            command = command.multicall(multicall);
        }
        if let Some(subcommand_value_name) = cmd.subcommand_value_name {
            command = command.subcommand_value_name(subcommand_value_name);
        }
        if let Some(subcommand_help_heading) = cmd.subcommand_help_heading {
            command = command.subcommand_help_heading(subcommand_help_heading);
        }

        command
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum ColorChoice {
    Auto,
    Always,
    Never,
}

impl From<ColorChoice> for clap::ColorChoice {
    fn from(color_choice: ColorChoice) -> Self {
        match color_choice {
            ColorChoice::Auto => clap::ColorChoice::Auto,
            ColorChoice::Always => clap::ColorChoice::Always,
            ColorChoice::Never => clap::ColorChoice::Never,
        }
    }
}

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
    num_args: Option<usize>,
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
    default_value: Option<String>,            // TODO is really OsString
    default_values: Option<Vec<String>>,      // TODO is really Vec<OsString>
    default_missing_value: Option<String>,    // TODO is really OsString
    default_missing_value_os: Option<String>, // TODO is really OsString
    default_missing_values: Option<Vec<String>>, // TODO is really Vec<OsString>
    default_missing_values_os: Option<Vec<String>>, // TODO is really Vec<OsString>
    env: Option<String>,                      // TODO is really OsString
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
    /// Non-standard option
    min_args: Option<usize>,
    /// Non-standard option
    max_args: Option<usize>,
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

        match (value.num_args, value.min_args, value.max_args) {
            (Some(_), Some(_), None) | (Some(_), None, Some(_)) | (Some(_), Some(_), Some(_)) => {
                // TODO how to handle this custom error
                panic!(
                    "num_args, min_args, and max_args are mutually exclusive. Use num_args to set both min and max, or use min_args and max_args to set them separately."
                );
            }
            (Some(num_args), _, _) => {
                arg = arg.num_args(clap::builder::ValueRange::new(num_args));
            }
            (None, Some(min), Some(max)) => {
                arg = arg.num_args(clap::builder::ValueRange::new(min..=max));
            }
            (None, Some(min), None) => {
                arg = arg.num_args(clap::builder::ValueRange::new(min..));
            }
            (None, None, Some(max)) => {
                arg = arg.num_args(clap::builder::ValueRange::new(..=max));
            }
            _ => {}
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
