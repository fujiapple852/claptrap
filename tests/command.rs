#[path = "util.rs"]
mod util;
use test_case::test_case;

#[test_case(case!("command", "no_args"))]
#[test_case(case!("command", "arg"))]
#[test_case(case!("command", "arg_required_else_help"))]
#[test_case(case!("command", "group"))]
#[test_case(case!("command", "groups"))]
#[test_case(case!("command", "subcommand"))]
#[test_case(case!("command", "subcommand_many"))]
#[test_case(case!("command", "ignore_error"))]
#[test_case(case!("command", "args_override_self"))]
#[test_case(case!("command", "dont_delimit_trailing_values"))]
#[test_case(case!("command", "color"))]
#[test_case(case!("command", "styles"))]
#[test_case(case!("command", "term_width"))]
#[test_case(case!("command", "max_term_width"))]
#[test_case(case!("command", "disable_version_flag-1"))]
#[test_case(case!("command", "disable_version_flag-2"))]
#[test_case(case!("command", "propagate_version"))]
#[test_case(case!("command", "next_line_help"))]
#[test_case(case!("command", "disable_help_flag-1"))]
#[test_case(case!("command", "disable_help_flag-2"))]
#[test_case(case!("command", "disable_help_subcommand"))]
#[test_case(case!("command", "disable_colored_help"))]
#[test_case(case!("command", "help_expected"))]
#[test_case(case!("command", "hide_possible_values"))]
#[test_case(case!("command", "infer_long_args"))]
#[test_case(case!("command", "infer_long_args_ambiguous"))]
#[test_case(case!("command", "infer_subcommands"))]
#[test_case(case!("command", "bin_name"))]
#[test_case(case!("command", "display_name"))]
#[test_case(case!("command", "author"))]
#[test_case(case!("command", "about"))]
#[test_case(case!("command", "after_help"))]
#[test_case(case!("command", "before_help"))]
#[test_case(case!("command", "before_long_help"))]
#[test_case(case!("command", "version"))]
#[test_case(case!("command", "long_version"))]
#[test_case(case!("command", "override_usage"))]
#[test_case(case!("command", "override_help"))]
#[test_case(case!("command", "template"))]
#[test_case(case!("command", "flatten_help"))]
#[test_case(case!("command", "next_help_heading"))]
#[test_case(case!("command", "next_display_order"))]
#[test_case(case!("command", "allow_missing_positional"))]
#[test_case(case!("command", "allow_missing_positional_with_default"))]
#[test_case(case!("command", "allow_missing_positional_style_2-1"))]
#[test_case(case!("command", "allow_missing_positional_style_2-2"))]
#[test_case(case!("command", "short_flag"))]
#[test_case(case!("command", "long_flag"))]
#[test_case(case!("command", "alias"))]
#[test_case(case!("command", "short_flag_alias"))]
#[test_case(case!("command", "long_flag_alias"))]
#[test_case(case!("command", "aliases"))]
#[test_case(case!("command", "short_flag_aliases"))]
#[test_case(case!("command", "long_flag_aliases"))]
#[test_case(case!("command", "visible_alias"))]
#[test_case(case!("command", "visible_short_flag_alias"))]
#[test_case(case!("command", "visible_long_flag_alias"))]
#[test_case(case!("command", "visible_aliases"))]
#[test_case(case!("command", "visible_short_flag_aliases"))]
#[test_case(case!("command", "visible_long_flag_aliases"))]
#[test_case(case!("command", "display_order"))]
#[test_case(case!("command", "hide"))]
#[test_case(case!("command", "subcommand_required"))]
#[test_case(case!("command", "allow_external_subcommands"))]
#[test_case(case!("command", "external_subcommand_value_parser"))]
#[test_case(case!("command", "args_conflicts_with_subcommands"))]
#[test_case(case!("command", "subcommand_precedence_over_arg"))]
#[test_case(case!("command", "subcommand_value_name"))]
#[test_case(case!("command", "subcommand_help_heading"))]
fn test_command((name, spec, args): (&str, &str, &str)) {
    insta::assert_snapshot!(name, util::run(spec, args));
}

#[test]
#[ignore = "test is sensitive to terminal width"]
fn long_about() {
    let output = util::run(
        include_str!("resources/command/long_about.toml"),
        include_str!("resources/command/long_about.args"),
    );
    insta::assert_snapshot!("long_about", output);
}

#[test]
#[ignore = "test is sensitive to terminal width"]
fn after_long_help() {
    let output = util::run(
        include_str!("resources/command/after_long_help.toml"),
        include_str!("resources/command/after_long_help.args"),
    );
    insta::assert_snapshot!("after_long_help", output);
}
