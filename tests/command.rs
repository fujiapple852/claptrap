#[path = "util.rs"]
mod util;
use test_case::test_case;

fn run(spec: &str, input: &str) -> claptrap::output::Output {
    util::run(spec, input)
}

#[test_case("no_args", include_str!("resources/command/no_args.toml"), include_str!("resources/command/no_args.args"); "no_args")]
#[test_case("arg", include_str!("resources/command/arg.toml"), include_str!("resources/command/arg.args"); "arg")]
#[test_case("arg_required_else_help", include_str!("resources/command/arg_required_else_help.toml"), include_str!("resources/command/arg_required_else_help.args"); "arg_required_else_help")]
#[test_case("group", include_str!("resources/command/group.toml"), include_str!("resources/command/group.args"); "group")]
#[test_case("groups", include_str!("resources/command/groups.toml"), include_str!("resources/command/groups.args"); "groups")]
#[test_case("subcommand", include_str!("resources/command/subcommand.toml"), include_str!("resources/command/subcommand.args"); "subcommand")]
#[test_case("subcommand_many", include_str!("resources/command/subcommand_many.toml"), include_str!("resources/command/subcommand_many.args"); "subcommand_many")]
#[test_case("ignore_error", include_str!("resources/command/ignore_error.toml"), include_str!("resources/command/ignore_error.args"); "ignore_error")]
#[test_case("args_override_self", include_str!("resources/command/args_override_self.toml"), include_str!("resources/command/args_override_self.args"); "args_override_self")]
#[test_case("dont_delimit_trailing_values", include_str!("resources/command/dont_delimit_trailing_values.toml"), include_str!("resources/command/dont_delimit_trailing_values.args"); "dont_delimit_trailing_values")]
#[test_case("color", include_str!("resources/command/color.toml"), include_str!("resources/command/color.args"); "color")]
#[test_case("styles", include_str!("resources/command/styles.toml"), include_str!("resources/command/styles.args"); "styles")]
#[test_case("term_width", include_str!("resources/command/term_width.toml"), include_str!("resources/command/term_width.args"); "term_width")]
#[test_case("max_term_width", include_str!("resources/command/max_term_width.toml"), include_str!("resources/command/max_term_width.args"); "max_term_width")]
#[test_case("diable_version_flag", include_str!("resources/command/diable_version_flag.toml"), include_str!("resources/command/diable_version_flag-1.args"); "diable_version_flag")]
#[test_case("diable_version_flag-2", include_str!("resources/command/diable_version_flag.toml"), include_str!("resources/command/diable_version_flag-2.args"); "diable_version_flag-2")]
#[test_case("propagate_version", include_str!("resources/command/propagate_version.toml"), include_str!("resources/command/propagate_version.args"); "propagate_version")]
#[test_case("next_line_help", include_str!("resources/command/next_line_help.toml"), include_str!("resources/command/next_line_help.args"); "next_line_help")]
#[test_case("diable_help_flag", include_str!("resources/command/diable_help_flag.toml"), include_str!("resources/command/diable_help_flag-1.args"); "diable_help_flag")]
#[test_case("diable_help_flag-2", include_str!("resources/command/diable_help_flag.toml"), include_str!("resources/command/diable_help_flag-2.args"); "diable_help_flag-2")]
#[test_case("disable_help_subcommand", include_str!("resources/command/disable_help_subcommand.toml"), include_str!("resources/command/disable_help_subcommand.args"); "disable_help_subcommand")]
#[test_case("disable_colored_help", include_str!("resources/command/disable_colored_help.toml"), include_str!("resources/command/disable_colored_help.args"); "disable_colored_help")]
#[test_case("help_expected", include_str!("resources/command/help_expected.toml"), include_str!("resources/command/help_expected.args"); "help_expected")]
#[test_case("hide_possible_values", include_str!("resources/command/hide_possible_values.toml"), include_str!("resources/command/hide_possible_values.args"); "hide_possible_values")]
#[test_case("infer_long_args", include_str!("resources/command/infer_long_args.toml"), include_str!("resources/command/infer_long_args.args"); "infer_long_args")]
#[test_case("infer_long_args_ambiguous", include_str!("resources/command/infer_long_args_ambiguous.toml"), include_str!("resources/command/infer_long_args_ambiguous.args"); "infer_long_args_ambiguous")]
#[test_case("infer_subcommands", include_str!("resources/command/infer_subcommands.toml"), include_str!("resources/command/infer_subcommands.args"); "infer_subcommands")]
#[test_case("bin_name", include_str!("resources/command/bin_name.toml"), include_str!("resources/command/bin_name.args"); "bin_name")]
#[test_case("display_name", include_str!("resources/command/display_name.toml"), include_str!("resources/command/display_name.args"); "display_name")]
#[test_case("author", include_str!("resources/command/author.toml"), include_str!("resources/command/author.args"); "author")]
#[test_case("about", include_str!("resources/command/about.toml"), include_str!("resources/command/about.args"); "about")]
#[test_case("after_help", include_str!("resources/command/after_help.toml"), include_str!("resources/command/after_help.args"); "after_help")]
#[test_case("before_help", include_str!("resources/command/before_help.toml"), include_str!("resources/command/before_help.args"); "before_help")]
#[test_case("before_long_help", include_str!("resources/command/before_long_help.toml"), include_str!("resources/command/before_long_help.args"); "before_long_help")]
#[test_case("version", include_str!("resources/command/version.toml"), include_str!("resources/command/version.args"); "version")]
#[test_case("long_version", include_str!("resources/command/long_version.toml"), include_str!("resources/command/long_version.args"); "long_version")]
#[test_case("override_usage", include_str!("resources/command/override_usage.toml"), include_str!("resources/command/override_usage.args"); "override_usage")]
#[test_case("override_help", include_str!("resources/command/override_help.toml"), include_str!("resources/command/override_help.args"); "override_help")]
#[test_case("template", include_str!("resources/command/template.toml"), include_str!("resources/command/template.args"); "template")]
#[test_case("flatten_help", include_str!("resources/command/flatten_help.toml"), include_str!("resources/command/flatten_help.args"); "flatten_help")]
#[test_case("next_help_heading", include_str!("resources/command/next_help_heading.toml"), include_str!("resources/command/next_help_heading.args"); "next_help_heading")]
#[test_case("next_display_order", include_str!("resources/command/next_display_order.toml"), include_str!("resources/command/next_display_order.args"); "next_display_order")]
#[test_case("allow_missing_positional", include_str!("resources/command/allow_missing_positional.toml"), include_str!("resources/command/allow_missing_positional.args"); "allow_missing_positional")]
#[test_case("allow_missing_positional_with_default", include_str!("resources/command/allow_missing_positional_with_default.toml"), include_str!("resources/command/allow_missing_positional_with_default.args"); "allow_missing_positional_with_default")]
#[test_case("allow_missing_positional_style_2", include_str!("resources/command/allow_missing_positional_style_2.toml"), include_str!("resources/command/allow_missing_positional_style_2-1.args"); "allow_missing_positional_style_2")]
#[test_case("allow_missing_positional_style_2-2", include_str!("resources/command/allow_missing_positional_style_2.toml"), include_str!("resources/command/allow_missing_positional_style_2-2.args"); "allow_missing_positional_style_2-2")]
#[test_case("short_flag", include_str!("resources/command/short_flag.toml"), include_str!("resources/command/short_flag.args"); "short_flag")]
#[test_case("long_flag", include_str!("resources/command/long_flag.toml"), include_str!("resources/command/long_flag.args"); "long_flag")]
#[test_case("alias", include_str!("resources/command/alias.toml"), include_str!("resources/command/alias.args"); "alias")]
#[test_case("short_flag_alias", include_str!("resources/command/short_flag_alias.toml"), include_str!("resources/command/short_flag_alias.args"); "short_flag_alias")]
#[test_case("long_flag_alias", include_str!("resources/command/long_flag_alias.toml"), include_str!("resources/command/long_flag_alias.args"); "long_flag_alias")]
#[test_case("aliases", include_str!("resources/command/aliases.toml"), include_str!("resources/command/aliases.args"); "aliases")]
#[test_case("short_flag_aliases", include_str!("resources/command/short_flag_aliases.toml"), include_str!("resources/command/short_flag_aliases.args"); "short_flag_aliases")]
#[test_case("long_flag_aliases", include_str!("resources/command/long_flag_aliases.toml"), include_str!("resources/command/long_flag_aliases.args"); "long_flag_aliases")]
#[test_case("visible_alias", include_str!("resources/command/visible_alias.toml"), include_str!("resources/command/visible_alias.args"); "visible_alias")]
#[test_case("visible_short_flag_alias", include_str!("resources/command/visible_short_flag_alias.toml"), include_str!("resources/command/visible_short_flag_alias.args"); "visible_short_flag_alias")]
#[test_case("visible_long_flag_alias", include_str!("resources/command/visible_long_flag_alias.toml"), include_str!("resources/command/visible_long_flag_alias.args"); "visible_long_flag_alias")]
#[test_case("visible_aliases", include_str!("resources/command/visible_aliases.toml"), include_str!("resources/command/visible_aliases.args"); "visible_aliases")]
#[test_case("visible_short_flag_aliases", include_str!("resources/command/visible_short_flag_aliases.toml"), include_str!("resources/command/visible_short_flag_aliases.args"); "visible_short_flag_aliases")]
#[test_case("visible_long_flag_aliases", include_str!("resources/command/visible_long_flag_aliases.toml"), include_str!("resources/command/visible_long_flag_aliases.args"); "visible_long_flag_aliases")]
#[test_case("display_order", include_str!("resources/command/display_order.toml"), include_str!("resources/command/display_order.args"); "display_order")]
#[test_case("hide", include_str!("resources/command/hide.toml"), include_str!("resources/command/hide.args"); "hide")]
#[test_case("subcommand_required", include_str!("resources/command/subcommand_required.toml"), include_str!("resources/command/subcommand_required.args"); "subcommand_required")]
#[test_case("allow_external_subcommands", include_str!("resources/command/allow_external_subcommands.toml"), include_str!("resources/command/allow_external_subcommands.args"); "allow_external_subcommands")]
#[test_case("external_subcommand_value_parser", include_str!("resources/command/external_subcommand_value_parser.toml"), include_str!("resources/command/external_subcommand_value_parser.args"); "external_subcommand_value_parser")]
#[test_case("args_conflicts_with_subcommands", include_str!("resources/command/args_conflicts_with_subcommands.toml"), include_str!("resources/command/args_conflicts_with_subcommands.args"); "args_conflicts_with_subcommands")]
#[test_case("subcommand_precedence_over_arg", include_str!("resources/command/subcommand_precedence_over_arg.toml"), include_str!("resources/command/subcommand_precedence_over_arg.args"); "subcommand_precedence_over_arg")]
#[test_case("subcommand_value_name", include_str!("resources/command/subcommand_value_name.toml"), include_str!("resources/command/subcommand_value_name.args"); "subcommand_value_name")]
#[test_case("subcommand_help_heading", include_str!("resources/command/subcommand_help_heading.toml"), include_str!("resources/command/subcommand_help_heading.args"); "subcommand_help_heading")]
fn test_commands(name: &str, spec: &str, args: &str) {
    let output = run(spec, args);
    insta::assert_snapshot!(name, output);
}

#[test]
#[ignore = "test is sensitive to terminal width"]
fn long_about() {
    let output = run(
        include_str!("resources/command/long_about.toml"),
        include_str!("resources/command/long_about.args"),
    );
    insta::assert_snapshot!("long_about", output);
}

#[test]
#[ignore = "test is sensitive to terminal width"]
fn after_long_help() {
    let output = run(
        include_str!("resources/command/after_long_help.toml"),
        include_str!("resources/command/after_long_help.args"),
    );
    insta::assert_snapshot!("after_long_help", output);
}
