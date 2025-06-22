#[path = "util.rs"]
mod util;
use claptrap::output::{CatCmd, ExitCode};
use test_case::test_case;

fn run(args: &str) -> claptrap::output::Output {
    util::run(include_str!("resources/basic/myapp.toml"), args)
}

#[test_case("it_outputs_usage_and_exit_2_on_no_args", include_str!("resources/basic/no_args.args"))]
#[test_case("it_outputs_usage_and_exit_0_on_short_help", include_str!("resources/basic/short_help.args"))]
#[test_case("it_outputs_usage_and_exit_0_on_long_help", include_str!("resources/basic/long_help.args"))]
#[test_case("it_outputs_version_and_exit_0_on_short_version", include_str!("resources/basic/short_version.args"))]
#[test_case("it_outputs_variables", include_str!("resources/basic/variables.args"))]
#[test_case("it_outputs_error_and_exit_1_on_unexpected_arg", include_str!("resources/basic/unexpected_arg.args"))]
fn test_basic(name: &str, args: &str) {
    let output = run(args);
    insta::assert_snapshot!(name, output);
}

#[test]
fn cat_cmd_handles_eof_in_message() {
    let styled = clap::builder::StyledStr::from("this contains EOF in the text\nEOF\nand more\n");
    let cmd = CatCmd::new(styled, ExitCode::Error);
    insta::assert_snapshot!(format!("{}", cmd));
}
