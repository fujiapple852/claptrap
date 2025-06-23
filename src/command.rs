use crate::arg::Arg;
use crate::arg_group::{ArgGroup, NamedArgGroup};
use crate::style::Styles;
use crate::values::ValueParser;
use indexmap::IndexMap;
use serde::{Deserialize, Deserializer};

/// Represents a command configuration for a CLI application.
#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub struct Command {
    name: String,
    #[serde(default, deserialize_with = "deserialize_args")]
    args: Option<IndexMap<String, Arg>>,
    #[serde(default)]
    subcommands: Vec<Command>,
    groups: Option<IndexMap<String, ArgGroup>>,
    ignore_errors: Option<bool>,
    args_override_self: Option<bool>,
    dont_delimit_trailing_values: Option<bool>,
    color: Option<ColorChoice>,
    styles: Option<Styles>,
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
    arg_required_else_help: Option<bool>,
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
    external_subcommand_value_parser: Option<ValueParser>,
    args_conflicts_with_subcommands: Option<bool>,
    subcommand_precedence_over_arg: Option<bool>,
    subcommand_negates_reqs: Option<bool>,
    multicall: Option<bool>,
    subcommand_value_name: Option<String>,
    subcommand_help_heading: Option<String>,
}

impl Command {
    #[must_use]
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_subcommands(&self) -> impl Iterator<Item = &Self> {
        self.subcommands.iter()
    }
}

fn deserialize_args<'de, D>(de: D) -> Result<Option<IndexMap<String, Arg>>, D::Error>
where
    D: Deserializer<'de>,
{
    let mut args = Option::<IndexMap<String, Arg>>::deserialize(de)?;
    if let Some(map) = &mut args {
        map.iter_mut().for_each(|(id, arg)| arg.id.clone_from(id));
    }
    Ok(args)
}

impl Command {
    pub fn get_arguments(&self) -> impl Iterator<Item = &Arg> {
        self.args
            .as_ref()
            .into_iter()
            .flat_map(|args| args.values())
    }
}

#[expect(clippy::cognitive_complexity, clippy::too_many_lines)]
impl From<Command> for clap::Command {
    fn from(cmd: Command) -> Self {
        let mut command = Self::new(cmd.name);
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
        if let Some(styles) = cmd.styles {
            command = command.styles(clap::builder::Styles::from(styles));
        }
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
        if let Some(external_subcommand_value_parser) = cmd.external_subcommand_value_parser {
            command = command.external_subcommand_value_parser(clap::builder::ValueParser::from(
                external_subcommand_value_parser,
            ));
        }
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
        command = command.args(if let Some(args) = cmd.args {
            args.into_values().map(clap::Arg::from).collect::<Vec<_>>()
        } else {
            Vec::new()
        });
        if let Some(groups) = cmd.groups {
            command = command.groups(
                groups
                    .into_iter()
                    .map(|(name, group)| clap::ArgGroup::from(NamedArgGroup::new(name, group))),
            );
        }
        command = command.subcommands(cmd.subcommands);
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
            ColorChoice::Auto => Self::Auto,
            ColorChoice::Always => Self::Always,
            ColorChoice::Never => Self::Never,
        }
    }
}
