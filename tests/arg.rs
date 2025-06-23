#[path = "util.rs"]
mod util;
use test_case::test_case;

#[test_case(case!("arg", "action_append"))]
#[test_case(case!("arg", "action_count"))]
#[test_case(case!("arg", "action_help"))]
#[test_case(case!("arg", "action_help_long"))]
#[test_case(case!("arg", "action_help_short"))]
#[test_case(case!("arg", "action_set"))]
#[test_case(case!("arg", "action_set_false"))]
#[test_case(case!("arg", "action_set_true"))]
#[test_case(case!("arg", "action_version"))]
#[test_case(case!("arg", "alias"))]
#[test_case(case!("arg", "aliases"))]
#[test_case(case!("arg", "allow_hyphen_values"))]
#[test_case(case!("arg", "allow_negative_numbers"))]
#[test_case(case!("arg", "conflicts_with"))]
#[test_case(case!("arg", "conflicts_with_all"))]
#[test_case(case!("arg", "default_missing_value-1"))]
#[test_case(case!("arg", "default_missing_value-2"))]
#[test_case(case!("arg", "default_missing_value-3"))]
#[test_case(case!("arg", "default_missing_value_bool_literal-1"))]
#[test_case(case!("arg", "default_missing_value_bool_literal-2"))]
#[test_case(case!("arg", "default_missing_value_bool_literal-3"))]
#[test_case(case!("arg", "default_value-1"))]
#[test_case(case!("arg", "default_value-2"))]
#[test_case(case!("arg", "default_value_if_equals-1"))]
#[test_case(case!("arg", "default_value_if_equals-2"))]
#[test_case(case!("arg", "default_value_if_not_present"))]
#[test_case(case!("arg", "default_value_if_present"))]
#[test_case(case!("arg", "default_value_if_unset"))]
#[test_case(case!("arg", "default_value_ifs-1"))]
#[test_case(case!("arg", "default_value_ifs-2"))]
#[test_case(case!("arg", "default_value_ifs_order"))]
#[test_case(case!("arg", "default_values-1"))]
#[test_case(case!("arg", "default_values-2"))]
#[test_case(case!("arg", "display_order"))]
#[test_case(case!("arg", "exclusive"))]
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
#[test_case(case!("arg", "ignore_case"))]
#[test_case(case!("arg", "ignore_case_multi"))]
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
#[test_case(case!("arg", "num_args_default_missing_value-1"))]
#[test_case(case!("arg", "num_args_default_missing_value-2"))]
#[test_case(case!("arg", "num_args_default_missing_value-3"))]
#[test_case(case!("arg", "num_args_multi_positional-1"))]
#[test_case(case!("arg", "num_args_multi_positional-2"))]
#[test_case(case!("arg", "num_args_multi_positional_solution"))]
#[test_case(case!("arg", "num_args_tuples-1"))]
#[test_case(case!("arg", "num_args_tuples-2"))]
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
#[test_case(case!("arg", "required_if_eq_case_insensitive"))]
#[test_case(case!("arg", "required_if_eq_case_sensitive"))]
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
#[test_case(case!("arg", "requires_ifs_present-1"))]
#[test_case(case!("arg", "requires_ifs_present-2"))]
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
    insta::assert_snapshot!(name, util::run(spec, args));
}

#[test_case(case!("arg", "env"))]
#[test_case(case!("arg", "env_default_value"))]
#[test_case(case!("arg", "env_option"))]
fn test_arg_env((name, spec, args): (&str, &str, &str)) {
    unsafe {
        std::env::set_var("MY_FLAG", "env");
    }
    insta::assert_snapshot!(name, util::run(spec, args));
}

#[test_case(case!("arg", "env_falsey"))]
fn test_arg_env_falsey((name, spec, args): (&str, &str, &str)) {
    unsafe {
        std::env::set_var("TRUE_FLAG", "true");
        std::env::set_var("FALSE_FLAG", "0");
    }
    insta::assert_snapshot!(name, util::run(spec, args));
}

#[test_case(case!("arg", "env_multi"))]
fn test_arg_env_multi((name, spec, args): (&str, &str, &str)) {
    unsafe {
        std::env::set_var("MY_FLAG_MULTI", "env1,env2");
    }
    insta::assert_snapshot!(name, util::run(spec, args));
}

#[test]
#[ignore = "test is sensitive to terminal width"]
fn long_help() {
    let output = util::run(
        include_str!("resources/arg/long_help.toml"),
        include_str!("resources/arg/long_help.args"),
    );
    insta::assert_snapshot!("long_help", output);
}
