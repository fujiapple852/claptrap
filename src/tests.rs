#![cfg(test)]

use crate::output::Output;
use claptrap::Command;
use std::ffi::OsString;

macro_rules! case {
    ($path:expr, $name:expr) => {{
        (
            $name,
            include_str!(concat!("../tests/resources/", $path, "/", $name, ".toml")),
            include_str!(concat!("../tests/resources/", $path, "/", $name, ".args")),
        )
    }};
}

fn run(spec: &str, args: &str) -> Output {
    let app: Command = toml::from_str(spec).unwrap();
    let args: Vec<OsString> = if args.trim().is_empty() {
        Vec::new()
    } else {
        args.split_whitespace().map(OsString::from).collect()
    };
    crate::parse::parse(app, args)
}

mod basic {
    use crate::output::{CatCmd, ExitCode};
    use test_case::test_case;

    #[test_case(case!("basic", "no_args"))]
    #[test_case(case!("basic", "short_help"))]
    #[test_case(case!("basic", "long_help"))]
    #[test_case(case!("basic", "short_version"))]
    #[test_case(case!("basic", "variables"))]
    #[test_case(case!("basic", "unexpected_arg"))]
    #[test_case(case!("basic", "all_aliases"))]
    fn test_values((name, spec, args): (&str, &str, &str)) {
        insta::assert_snapshot!(name, super::run(spec, args));
    }

    #[test]
    fn cat_cmd_handles_eof_in_message() {
        let styled =
            clap::builder::StyledStr::from("this contains EOF in the text\nEOF\nand more\n");
        let cmd = CatCmd::new(styled, ExitCode::Error, clap::ColorChoice::Never);
        insta::assert_snapshot!(format!("{}", cmd));
    }
}

mod command {
    use test_case::test_case;

    #[test_case(case!("command", "name"))]
    #[test_case(case!("command", "arg"))]
    #[test_case(case!("command", "args"))]
    #[test_case(case!("command", "arg_required_else_help"))]
    #[test_case(case!("command", "group"))]
    #[test_case(case!("command", "groups"))]
    #[test_case(case!("command", "subcommand"))]
    #[test_case(case!("command", "subcommands"))]
    #[test_case(case!("command", "no_binary_name"))]
    #[test_case(case!("command", "ignore_errors"))]
    #[test_case(case!("command", "args_override_self"))]
    #[test_case(case!("command", "dont_delimit_trailing_values"))]
    #[test_case(case!("command", "color-1"))]
    #[test_case(case!("command", "color-2"))]
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
    #[test_case(case!("command", "disable_colored_help-1"))]
    #[test_case(case!("command", "disable_colored_help-2"))]
    #[test_case(case!("command", "help_expected"))]
    #[test_case(case!("command", "hide_possible_values"))]
    #[test_case(case!("command", "infer_long_args"))]
    #[test_case(case!("command", "infer_long_args-ambiguous"))]
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
    #[test_case(case!("command", "help_template"))]
    #[test_case(case!("command", "flatten_help"))]
    #[test_case(case!("command", "next_help_heading"))]
    #[test_case(case!("command", "next_display_order"))]
    #[test_case(case!("command", "allow_missing_positional-1"))]
    #[test_case(case!("command", "allow_missing_positional-with_default"))]
    #[test_case(case!("command", "allow_missing_positional-2"))]
    #[test_case(case!("command", "allow_missing_positional-3"))]
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
    #[test_case(case!("command", "visible_long_flag_aliases-1"))]
    #[test_case(case!("command", "visible_long_flag_aliases-2"))]
    #[test_case(case!("command", "display_order"))]
    #[test_case(case!("command", "hide"))]
    #[test_case(case!("command", "subcommand_required"))]
    #[test_case(case!("command", "allow_external_subcommands"))]
    #[test_case(case!("command", "external_subcommand_value_parser"))]
    #[test_case(case!("command", "args_conflicts_with_subcommands"))]
    #[test_case(case!("command", "subcommand_precedence_over_arg"))]
    #[test_case(case!("command", "subcommand_negates_reqs"))]
    #[test_case(case!("command", "multicall-error"))]
    #[test_case(case!("command", "multicall-busybox1"))]
    #[test_case(case!("command", "multicall-busybox2"))]
    #[test_case(case!("command", "subcommand_value_name"))]
    #[test_case(case!("command", "subcommand_help_heading"))]
    fn test_command((name, spec, args): (&str, &str, &str)) {
        insta::assert_snapshot!(name, super::run(spec, args));
    }

    #[test]
    #[ignore = "test is sensitive to terminal width"]
    fn long_about() {
        let output = super::run(
            include_str!("../tests/resources/command/long_about.toml"),
            include_str!("../tests/resources/command/long_about.args"),
        );
        insta::assert_snapshot!("long_about", output);
    }

    #[test]
    #[ignore = "test is sensitive to terminal width"]
    fn after_long_help() {
        let output = super::run(
            include_str!("../tests/resources/command/after_long_help.toml"),
            include_str!("../tests/resources/command/after_long_help.args"),
        );
        insta::assert_snapshot!("after_long_help", output);
    }
}

mod arg {
    use test_case::test_case;

    #[test_case(case!("arg", "id"))]
    #[test_case(case!("arg", "action-append"))]
    #[test_case(case!("arg", "action-count"))]
    #[test_case(case!("arg", "action-help"))]
    #[test_case(case!("arg", "action-help_long"))]
    #[test_case(case!("arg", "action-help_short"))]
    #[test_case(case!("arg", "action-set"))]
    #[test_case(case!("arg", "action-set_false"))]
    #[test_case(case!("arg", "action-set_true"))]
    #[test_case(case!("arg", "action-version"))]
    #[test_case(case!("arg", "alias"))]
    #[test_case(case!("arg", "aliases"))]
    #[test_case(case!("arg", "allow_hyphen_values"))]
    #[test_case(case!("arg", "allow_negative_numbers"))]
    #[test_case(case!("arg", "conflicts_with"))]
    #[test_case(case!("arg", "conflicts_with_all"))]
    #[test_case(case!("arg", "default_missing_value-1"))]
    #[test_case(case!("arg", "default_missing_value-2"))]
    #[test_case(case!("arg", "default_missing_value-3"))]
    #[test_case(case!("arg", "default_missing_value_os"))]
    #[test_case(case!("arg", "default_missing_values"))]
    #[test_case(case!("arg", "default_missing_values_os"))]
    #[test_case(case!("arg", "default_missing_value-bool_literal1"))]
    #[test_case(case!("arg", "default_missing_value-bool_literal2"))]
    #[test_case(case!("arg", "default_missing_value-bool_literal3"))]
    #[test_case(case!("arg", "raw"))]
    #[test_case(case!("arg", "default_value-1"))]
    #[test_case(case!("arg", "default_value-2"))]
    #[test_case(case!("arg", "default_value_if-equals1"))]
    #[test_case(case!("arg", "default_value_if-equals2"))]
    #[test_case(case!("arg", "default_value_if-not_present"))]
    #[test_case(case!("arg", "default_value_if-present"))]
    #[test_case(case!("arg", "default_value_if-unset"))]
    #[test_case(case!("arg", "default_value_ifs-1"))]
    #[test_case(case!("arg", "default_value_ifs-2"))]
    #[test_case(case!("arg", "default_value_ifs-order"))]
    #[test_case(case!("arg", "default_values-1"))]
    #[test_case(case!("arg", "default_values-2"))]
    #[test_case(case!("arg", "display_order"))]
    #[test_case(case!("arg", "exclusive"))]
    #[test_case(case!("arg", "global"))]
    #[test_case(case!("arg", "group"))]
    #[test_case(case!("arg", "groups"))]
    #[test_case(case!("arg", "help"))]
    #[test_case(case!("arg", "help_heading"))]
    #[test_case(case!("arg", "hide"))]
    #[test_case(case!("arg", "hide_default_value"))]
    #[test_case(case!("arg", "hide_env"))]
    #[test_case(case!("arg", "hide_env_values"))]
    #[test_case(case!("arg", "hide_long_help-1"))]
    #[test_case(case!("arg", "hide_long_help-2"))]
    #[test_case(case!("arg", "hide_possible_values"))]
    #[test_case(case!("arg", "hide_short_help-1"))]
    #[test_case(case!("arg", "hide_short_help-2"))]
    #[test_case(case!("arg", "value_hint"))]
    #[test_case(case!("arg", "ignore_case"))]
    #[test_case(case!("arg", "ignore_case-multi"))]
    #[test_case(case!("arg", "index"))]
    #[test_case(case!("arg", "it_fails_when_action_set_false_twice"))]
    #[test_case(case!("arg", "it_fails_when_action_set_true_twice"))]
    #[test_case(case!("arg", "it_fails_when_action_set_twice"))]
    #[test_case(case!("arg", "it_fails_when_not_allow_hyphen_values"))]
    #[test_case(case!("arg", "last-1"))]
    #[test_case(case!("arg", "last-2"))]
    #[test_case(case!("arg", "long"))]
    #[test_case(case!("arg", "next_line_help"))]
    #[test_case(case!("arg", "num_args"))]
    #[test_case(case!("arg", "num_args-default_missing_value1"))]
    #[test_case(case!("arg", "num_args-default_missing_value2"))]
    #[test_case(case!("arg", "num_args-default_missing_value3"))]
    #[test_case(case!("arg", "num_args-multi_positional1"))]
    #[test_case(case!("arg", "num_args-multi_positional2"))]
    #[test_case(case!("arg", "num_args-multi_positional_solution"))]
    #[test_case(case!("arg", "num_args-tuples1"))]
    #[test_case(case!("arg", "num_args-tuples2"))]
    #[test_case(case!("arg", "overrides_with"))]
    #[test_case(case!("arg", "overrides_with_all"))]
    #[test_case(case!("arg", "require_equals-1"))]
    #[test_case(case!("arg", "require_equals-2"))]
    #[test_case(case!("arg", "required-1"))]
    #[test_case(case!("arg", "required-2"))]
    #[test_case(case!("arg", "required_if_eq-1"))]
    #[test_case(case!("arg", "required_if_eq-2"))]
    #[test_case(case!("arg", "required_if_eq_all-1"))]
    #[test_case(case!("arg", "required_if_eq_all-2"))]
    #[test_case(case!("arg", "required_if_eq_any-1"))]
    #[test_case(case!("arg", "required_if_eq_any-2"))]
    #[test_case(case!("arg", "required_if_eq-case_insensitive"))]
    #[test_case(case!("arg", "required_if_eq-case_sensitive"))]
    #[test_case(case!("arg", "required_unless_present-1"))]
    #[test_case(case!("arg", "required_unless_present-2"))]
    #[test_case(case!("arg", "required_unless_present_all-1"))]
    #[test_case(case!("arg", "required_unless_present_all-2"))]
    #[test_case(case!("arg", "required_unless_present_any-1"))]
    #[test_case(case!("arg", "required_unless_present_any-2"))]
    #[test_case(case!("arg", "requires-1"))]
    #[test_case(case!("arg", "requires-2"))]
    #[test_case(case!("arg", "requires_if-1"))]
    #[test_case(case!("arg", "requires_if-2"))]
    #[test_case(case!("arg", "requires_ifs-1"))]
    #[test_case(case!("arg", "requires_ifs-2"))]
    #[test_case(case!("arg", "requires_ifs-present1"))]
    #[test_case(case!("arg", "requires_ifs-present2"))]
    #[test_case(case!("arg", "short"))]
    #[test_case(case!("arg", "short_alias"))]
    #[test_case(case!("arg", "short_aliases"))]
    #[test_case(case!("arg", "short_help"))]
    #[test_case(case!("arg", "trailing_var_arg"))]
    #[test_case(case!("arg", "value_delimiter"))]
    #[test_case(case!("arg", "value_name"))]
    #[test_case(case!("arg", "value_names"))]
    #[test_case(case!("arg", "value_parser"))]
    #[test_case(case!("arg", "value_terminator"))]
    #[test_case(case!("arg", "visible_alias"))]
    #[test_case(case!("arg", "visible_aliases"))]
    #[test_case(case!("arg", "visible_short_alias"))]
    #[test_case(case!("arg", "visible_short_aliases"))]
    fn test_arg((name, spec, args): (&str, &str, &str)) {
        insta::assert_snapshot!(name, super::run(spec, args));
    }

    #[test_case(case!("arg", "env"))]
    #[test_case(case!("arg", "env-default_value"))]
    #[test_case(case!("arg", "env-option"))]
    fn test_arg_env((name, spec, args): (&str, &str, &str)) {
        unsafe {
            std::env::set_var("MY_FLAG", "env");
        }
        insta::assert_snapshot!(name, super::run(spec, args));
    }

    #[test_case(case!("arg", "env-falsey"))]
    fn test_arg_env_falsey((name, spec, args): (&str, &str, &str)) {
        unsafe {
            std::env::set_var("TRUE_FLAG", "true");
            std::env::set_var("FALSE_FLAG", "0");
        }
        insta::assert_snapshot!(name, super::run(spec, args));
    }

    #[test_case(case!("arg", "env-multi"))]
    fn test_arg_env_multi((name, spec, args): (&str, &str, &str)) {
        unsafe {
            std::env::set_var("MY_FLAG_MULTI", "env1,env2");
        }
        insta::assert_snapshot!(name, super::run(spec, args));
    }

    #[test]
    #[ignore = "test is sensitive to terminal width"]
    fn long_help() {
        let output = super::run(
            include_str!("../tests/resources/arg/long_help.toml"),
            include_str!("../tests/resources/arg/long_help.args"),
        );
        insta::assert_snapshot!("long_help", output);
    }
}

mod arg_group {
    use test_case::test_case;

    #[test_case(case!("arg_group", "id"))]
    #[test_case(case!("arg_group", "args"))]
    #[test_case(case!("arg_group", "multiple"))]
    #[test_case(case!("arg_group", "required"))]
    #[test_case(case!("arg_group", "requires"))]
    #[test_case(case!("arg_group", "requires_all"))]
    #[test_case(case!("arg_group", "conflicts_with"))]
    #[test_case(case!("arg_group", "conflicts_with_all"))]
    fn test_arg_group((name, spec, args): (&str, &str, &str)) {
        insta::assert_snapshot!(name, super::run(spec, args));
    }
}

mod value_parser {
    use test_case::test_case;

    #[test_case(case!("value_parser", "bool"))]
    #[test_case(case!("value_parser", "boolish"))]
    #[test_case(case!("value_parser", "falsey"))]
    #[test_case(case!("value_parser", "string"))]
    #[test_case(case!("value_parser", "u8"))]
    #[test_case(case!("value_parser", "u16"))]
    #[test_case(case!("value_parser", "u32"))]
    #[test_case(case!("value_parser", "u64"))]
    #[test_case(case!("value_parser", "u128"))]
    #[test_case(case!("value_parser", "usize"))]
    #[test_case(case!("value_parser", "i8"))]
    #[test_case(case!("value_parser", "i16"))]
    #[test_case(case!("value_parser", "i32"))]
    #[test_case(case!("value_parser", "i64"))]
    #[test_case(case!("value_parser", "i128"))]
    #[test_case(case!("value_parser", "isize"))]
    #[test_case(case!("value_parser", "f32"))]
    #[test_case(case!("value_parser", "f64"))]
    #[test_case(case!("value_parser", "possible_values"))]
    fn test_values((name, spec, args): (&str, &str, &str)) {
        insta::assert_snapshot!(name, super::run(spec, args));
    }
}

mod quoting {
    use test_case::test_case;

    #[test_case(case!("quoting", "single_value_with_quote"))]
    #[test_case(case!("quoting", "many_values_special_chars"))]
    fn test_quoting((name, spec, args): (&str, &str, &str)) {
        insta::assert_snapshot!(name, super::run(spec, args));
    }
}
