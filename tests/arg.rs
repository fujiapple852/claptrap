#[path = "util.rs"]
mod util;
use test_case::test_case;

fn run(name: &str, spec: &str, input: &str) -> claptrap::output::Output {
    unsafe {
        match name {
            "env" | "env_option" | "env_default_value" => std::env::set_var("MY_FLAG", "env"),
            "env_falsey" => {
                std::env::set_var("TRUE_FLAG", "true");
                std::env::set_var("FALSE_FLAG", "0");
            }
            "env_multi" => std::env::set_var("MY_FLAG_MULTI", "env1,env2"),
            _ => {}
        }
    }
    util::run(spec, input)
}
#[test_case(include_str!("resources/arg/action_append.toml"), include_str!("resources/arg/action_append.args"), "action_append")]
#[test_case(include_str!("resources/arg/action_count.toml"), include_str!("resources/arg/action_count.args"), "action_count")]
#[test_case(include_str!("resources/arg/action_help.toml"), include_str!("resources/arg/action_help.args"), "action_help")]
#[test_case(include_str!("resources/arg/action_help_long.toml"), include_str!("resources/arg/action_help_long.args"), "action_help_long")]
#[test_case(include_str!("resources/arg/action_help_short.toml"), include_str!("resources/arg/action_help_short.args"), "action_help_short")]
#[test_case(include_str!("resources/arg/action_set.toml"), include_str!("resources/arg/action_set.args"), "action_set")]
#[test_case(include_str!("resources/arg/action_set_false.toml"), include_str!("resources/arg/action_set_false.args"), "action_set_false")]
#[test_case(include_str!("resources/arg/action_set_true.toml"), include_str!("resources/arg/action_set_true.args"), "action_set_true")]
#[test_case(include_str!("resources/arg/action_version.toml"), include_str!("resources/arg/action_version.args"), "action_version")]
#[test_case(include_str!("resources/arg/alias.toml"), include_str!("resources/arg/alias.args"), "alias")]
#[test_case(include_str!("resources/arg/aliases.toml"), include_str!("resources/arg/aliases.args"), "aliases")]
#[test_case(include_str!("resources/arg/allow_hyphen_values.toml"), include_str!("resources/arg/allow_hyphen_values.args"), "allow_hyphen_values")]
#[test_case(include_str!("resources/arg/allow_negative_numbers.toml"), include_str!("resources/arg/allow_negative_numbers.args"), "allow_negative_numbers")]
#[test_case(include_str!("resources/arg/conflicts_with.toml"), include_str!("resources/arg/conflicts_with.args"), "conflicts_with")]
#[test_case(include_str!("resources/arg/conflicts_with_all.toml"), include_str!("resources/arg/conflicts_with_all.args"), "conflicts_with_all")]
#[test_case(include_str!("resources/arg/default_missing_value.toml"), include_str!("resources/arg/default_missing_value-2.args"), "default_missing_value-2")]
#[test_case(include_str!("resources/arg/default_missing_value.toml"), include_str!("resources/arg/default_missing_value-3.args"), "default_missing_value-3")]
#[test_case(include_str!("resources/arg/default_missing_value.toml"), include_str!("resources/arg/default_missing_value.args"), "default_missing_value")]
#[test_case(include_str!("resources/arg/default_missing_value_bool_literal.toml"), include_str!("resources/arg/default_missing_value_bool_literal-2.args"), "default_missing_value_bool_literal-2")]
#[test_case(include_str!("resources/arg/default_missing_value_bool_literal.toml"), include_str!("resources/arg/default_missing_value_bool_literal-3.args"), "default_missing_value_bool_literal-3")]
#[test_case(include_str!("resources/arg/default_missing_value_bool_literal.toml"), include_str!("resources/arg/default_missing_value_bool_literal.args"), "default_missing_value_bool_literal")]
#[test_case(include_str!("resources/arg/default_value.toml"), include_str!("resources/arg/default_value-2.args"), "default_value-2")]
#[test_case(include_str!("resources/arg/default_value.toml"), include_str!("resources/arg/default_value.args"), "default_value")]
#[test_case(include_str!("resources/arg/default_value_if_equals.toml"), include_str!("resources/arg/default_value_if_equals-2.args"), "default_value_if_equals-2")]
#[test_case(include_str!("resources/arg/default_value_if_equals.toml"), include_str!("resources/arg/default_value_if_equals.args"), "default_value_if_equals")]
#[test_case(include_str!("resources/arg/default_value_if_not_present.toml"), include_str!("resources/arg/default_value_if_not_present.args"), "default_value_if_not_present")]
#[test_case(include_str!("resources/arg/default_value_if_present.toml"), include_str!("resources/arg/default_value_if_present.args"), "default_value_if_present")]
#[test_case(include_str!("resources/arg/default_value_if_unset.toml"), include_str!("resources/arg/default_value_if_unset.args"), "default_value_if_unset")]
#[test_case(include_str!("resources/arg/default_value_ifs.toml"), include_str!("resources/arg/default_value_ifs-2.args"), "default_value_ifs-2")]
#[test_case(include_str!("resources/arg/default_value_ifs.toml"), include_str!("resources/arg/default_value_ifs.args"), "default_value_ifs")]
#[test_case(include_str!("resources/arg/default_value_ifs_order.toml"), include_str!("resources/arg/default_value_ifs_order.args"), "default_value_ifs_order")]
#[test_case(include_str!("resources/arg/default_values.toml"), include_str!("resources/arg/default_values-2.args"), "default_values-2")]
#[test_case(include_str!("resources/arg/default_values.toml"), include_str!("resources/arg/default_values.args"), "default_values")]
#[test_case(include_str!("resources/arg/display_order.toml"), include_str!("resources/arg/display_order.args"), "display_order")]
#[test_case(include_str!("resources/arg/env.toml"), include_str!("resources/arg/env.args"), "env")]
#[test_case(include_str!("resources/arg/env_default_value.toml"), include_str!("resources/arg/env_default_value.args"), "env_default_value")]
#[test_case(include_str!("resources/arg/env_falsey.toml"), include_str!("resources/arg/env_falsey.args"), "env_falsey")]
#[test_case(include_str!("resources/arg/env_multi.toml"), include_str!("resources/arg/env_multi.args"), "env_multi")]
#[test_case(include_str!("resources/arg/env_option.toml"), include_str!("resources/arg/env_option.args"), "env_option")]
#[test_case(include_str!("resources/arg/exclusive.toml"), include_str!("resources/arg/exclusive.args"), "exclusive")]
#[test_case(include_str!("resources/arg/group.toml"), include_str!("resources/arg/group.args"), "group")]
#[test_case(include_str!("resources/arg/groups.toml"), include_str!("resources/arg/groups.args"), "groups")]
#[test_case(include_str!("resources/arg/help.toml"), include_str!("resources/arg/help.args"), "help")]
#[test_case(include_str!("resources/arg/help_heading.toml"), include_str!("resources/arg/help_heading.args"), "help_heading")]
#[test_case(include_str!("resources/arg/hide.toml"), include_str!("resources/arg/hide.args"), "hide")]
#[test_case(include_str!("resources/arg/hide_default_value.toml"), include_str!("resources/arg/hide_default_value.args"), "hide_default_value")]
#[test_case(include_str!("resources/arg/hide_env.toml"), include_str!("resources/arg/hide_env.args"), "hide_env")]
#[test_case(include_str!("resources/arg/hide_env_values.toml"), include_str!("resources/arg/hide_env_values.args"), "hide_env_values")]
#[test_case(include_str!("resources/arg/hide_long_help.toml"), include_str!("resources/arg/hide_long_help-2.args"), "hide_long_help-2")]
#[test_case(include_str!("resources/arg/hide_long_help.toml"), include_str!("resources/arg/hide_long_help.args"), "hide_long_help")]
#[test_case(include_str!("resources/arg/hide_possible_values.toml"), include_str!("resources/arg/hide_possible_values.args"), "hide_possible_values")]
#[test_case(include_str!("resources/arg/hide_short_help.toml"), include_str!("resources/arg/hide_short_help-2.args"), "hide_short_help-2")]
#[test_case(include_str!("resources/arg/hide_short_help.toml"), include_str!("resources/arg/hide_short_help.args"), "hide_short_help")]
#[test_case(include_str!("resources/arg/ignore_case.toml"), include_str!("resources/arg/ignore_case.args"), "ignore_case")]
#[test_case(include_str!("resources/arg/ignore_case_multi.toml"), include_str!("resources/arg/ignore_case_multi.args"), "ignore_case_multi")]
#[test_case(include_str!("resources/arg/index.toml"), include_str!("resources/arg/index.args"), "index")]
#[test_case(include_str!("resources/arg/it_fails_when_action_set_false_twice.toml"), include_str!("resources/arg/it_fails_when_action_set_false_twice.args"), "it_fails_when_action_set_false_twice")]
#[test_case(include_str!("resources/arg/it_fails_when_action_set_true_twice.toml"), include_str!("resources/arg/it_fails_when_action_set_true_twice.args"), "it_fails_when_action_set_true_twice")]
#[test_case(include_str!("resources/arg/it_fails_when_action_set_twice.toml"), include_str!("resources/arg/it_fails_when_action_set_twice.args"), "it_fails_when_action_set_twice")]
#[test_case(include_str!("resources/arg/it_fails_when_not_allow_hyphen_values.toml"), include_str!("resources/arg/it_fails_when_not_allow_hyphen_values.args"), "it_fails_when_not_allow_hyphen_values")]
#[test_case(include_str!("resources/arg/last.toml"), include_str!("resources/arg/last-2.args"), "last-2")]
#[test_case(include_str!("resources/arg/last.toml"), include_str!("resources/arg/last.args"), "last")]
#[test_case(include_str!("resources/arg/long.toml"), include_str!("resources/arg/long.args"), "long")]
#[test_case(include_str!("resources/arg/next_line_help.toml"), include_str!("resources/arg/next_line_help.args"), "next_line_help")]
#[test_case(include_str!("resources/arg/num_args.toml"), include_str!("resources/arg/num_args.args"), "num_args")]
#[test_case(include_str!("resources/arg/num_args_default_missing_value.toml"), include_str!("resources/arg/num_args_default_missing_value-2.args"), "num_args_default_missing_value-2")]
#[test_case(include_str!("resources/arg/num_args_default_missing_value.toml"), include_str!("resources/arg/num_args_default_missing_value-3.args"), "num_args_default_missing_value-3")]
#[test_case(include_str!("resources/arg/num_args_default_missing_value.toml"), include_str!("resources/arg/num_args_default_missing_value.args"), "num_args_default_missing_value")]
#[test_case(include_str!("resources/arg/num_args_multi_positional.toml"), include_str!("resources/arg/num_args_multi_positional-2.args"), "num_args_multi_positional-2")]
#[test_case(include_str!("resources/arg/num_args_multi_positional.toml"), include_str!("resources/arg/num_args_multi_positional.args"), "num_args_multi_positional")]
#[test_case(include_str!("resources/arg/num_args_multi_positional_solution.toml"), include_str!("resources/arg/num_args_multi_positional_solution.args"), "num_args_multi_positional_solution")]
#[test_case(include_str!("resources/arg/num_args_tuples.toml"), include_str!("resources/arg/num_args_tuples-2.args"), "num_args_tuples-2")]
#[test_case(include_str!("resources/arg/num_args_tuples.toml"), include_str!("resources/arg/num_args_tuples.args"), "num_args_tuples")]
#[test_case(include_str!("resources/arg/overrides_with.toml"), include_str!("resources/arg/overrides_with.args"), "overrides_with")]
#[test_case(include_str!("resources/arg/overrides_with_all.toml"), include_str!("resources/arg/overrides_with_all.args"), "overrides_with_all")]
#[test_case(include_str!("resources/arg/require_equals.toml"), include_str!("resources/arg/require_equals-2.args"), "require_equals-2")]
#[test_case(include_str!("resources/arg/require_equals.toml"), include_str!("resources/arg/require_equals.args"), "require_equals")]
#[test_case(include_str!("resources/arg/required.toml"), include_str!("resources/arg/required-2.args"), "required-2")]
#[test_case(include_str!("resources/arg/required.toml"), include_str!("resources/arg/required.args"), "required")]
#[test_case(include_str!("resources/arg/required_if_eq.toml"), include_str!("resources/arg/required_if_eq-2.args"), "required_if_eq-2")]
#[test_case(include_str!("resources/arg/required_if_eq.toml"), include_str!("resources/arg/required_if_eq.args"), "required_if_eq")]
#[test_case(include_str!("resources/arg/required_if_eq_all.toml"), include_str!("resources/arg/required_if_eq_all-2.args"), "required_if_eq_all-2")]
#[test_case(include_str!("resources/arg/required_if_eq_all.toml"), include_str!("resources/arg/required_if_eq_all.args"), "required_if_eq_all")]
#[test_case(include_str!("resources/arg/required_if_eq_any.toml"), include_str!("resources/arg/required_if_eq_any-2.args"), "required_if_eq_any-2")]
#[test_case(include_str!("resources/arg/required_if_eq_any.toml"), include_str!("resources/arg/required_if_eq_any.args"), "required_if_eq_any")]
#[test_case(include_str!("resources/arg/required_if_eq_case_insensitive.toml"), include_str!("resources/arg/required_if_eq_case_insensitive.args"), "required_if_eq_case_insensitive")]
#[test_case(include_str!("resources/arg/required_if_eq_case_sensitive.toml"), include_str!("resources/arg/required_if_eq_case_sensitive.args"), "required_if_eq_case_sensitive")]
#[test_case(include_str!("resources/arg/required_unless_present.toml"), include_str!("resources/arg/required_unless_present-2.args"), "required_unless_present-2")]
#[test_case(include_str!("resources/arg/required_unless_present.toml"), include_str!("resources/arg/required_unless_present.args"), "required_unless_present")]
#[test_case(include_str!("resources/arg/required_unless_present_all.toml"), include_str!("resources/arg/required_unless_present_all-2.args"), "required_unless_present_all-2")]
#[test_case(include_str!("resources/arg/required_unless_present_all.toml"), include_str!("resources/arg/required_unless_present_all.args"), "required_unless_present_all")]
#[test_case(include_str!("resources/arg/required_unless_present_any.toml"), include_str!("resources/arg/required_unless_present_any-2.args"), "required_unless_present_any-2")]
#[test_case(include_str!("resources/arg/required_unless_present_any.toml"), include_str!("resources/arg/required_unless_present_any.args"), "required_unless_present_any")]
#[test_case(include_str!("resources/arg/requires.toml"), include_str!("resources/arg/requires-2.args"), "requires-2")]
#[test_case(include_str!("resources/arg/requires.toml"), include_str!("resources/arg/requires.args"), "requires")]
#[test_case(include_str!("resources/arg/requires_if.toml"), include_str!("resources/arg/requires_if-2.args"), "requires_if-2")]
#[test_case(include_str!("resources/arg/requires_if.toml"), include_str!("resources/arg/requires_if.args"), "requires_if")]
#[test_case(include_str!("resources/arg/requires_ifs.toml"), include_str!("resources/arg/requires_ifs-2.args"), "requires_ifs-2")]
#[test_case(include_str!("resources/arg/requires_ifs.toml"), include_str!("resources/arg/requires_ifs.args"), "requires_ifs")]
#[test_case(include_str!("resources/arg/requires_ifs_present.toml"), include_str!("resources/arg/requires_ifs_present-2.args"), "requires_ifs_present-2")]
#[test_case(include_str!("resources/arg/requires_ifs_present.toml"), include_str!("resources/arg/requires_ifs_present.args"), "requires_ifs_present")]
#[test_case(include_str!("resources/arg/short.toml"), include_str!("resources/arg/short.args"), "short")]
#[test_case(include_str!("resources/arg/short_alias.toml"), include_str!("resources/arg/short_alias.args"), "short_alias")]
#[test_case(include_str!("resources/arg/short_aliases.toml"), include_str!("resources/arg/short_aliases.args"), "short_aliases")]
#[test_case(include_str!("resources/arg/short_help.toml"), include_str!("resources/arg/short_help.args"), "short_help")]
#[test_case(include_str!("resources/arg/trailing_var_arg.toml"), include_str!("resources/arg/trailing_var_arg.args"), "trailing_var_arg")]
#[test_case(include_str!("resources/arg/value_delimiter.toml"), include_str!("resources/arg/value_delimiter.args"), "value_delimiter")]
#[test_case(include_str!("resources/arg/value_name.toml"), include_str!("resources/arg/value_name.args"), "value_name")]
#[test_case(include_str!("resources/arg/value_names.toml"), include_str!("resources/arg/value_names.args"), "value_names")]
#[test_case(include_str!("resources/arg/value_parser.toml"), include_str!("resources/arg/value_parser.args"), "value_parser")]
#[test_case(include_str!("resources/arg/value_terminator.toml"), include_str!("resources/arg/value_terminator.args"), "value_terminator")]
#[test_case(include_str!("resources/arg/visible_alias.toml"), include_str!("resources/arg/visible_alias.args"), "visible_alias")]
#[test_case(include_str!("resources/arg/visible_aliases.toml"), include_str!("resources/arg/visible_aliases.args"), "visible_aliases")]
#[test_case(include_str!("resources/arg/visible_short_alias.toml"), include_str!("resources/arg/visible_short_alias.args"), "visible_short_alias")]
#[test_case(include_str!("resources/arg/visible_short_aliases.toml"), include_str!("resources/arg/visible_short_aliases.args"), "visible_short_aliases")]
fn test_args(spec: &str, args: &str, name: &str) {
    let output = run(name, spec, args);
    insta::assert_snapshot!(name, output);
}

#[test]
#[ignore = "test is sensitive to terminal width"]
fn long_help() {
    let output = run(
        "long_help",
        include_str!("resources/arg/long_help.toml"),
        include_str!("resources/arg/long_help.args"),
    );
    insta::assert_snapshot!("long_help", output);
}
