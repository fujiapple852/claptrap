#[path = "util.rs"]
mod util;
use claptrap::output::{CatCmd, ExitCode};
use test_case::test_case;

#[test_case(case!("basic", "no_args"))]
#[test_case(case!("basic", "short_help"))]
#[test_case(case!("basic", "long_help"))]
#[test_case(case!("basic", "short_version"))]
#[test_case(case!("basic", "variables"))]
#[test_case(case!("basic", "unexpected_arg"))]
fn test_values((name, spec, args): (&str, &str, &str)) {
    insta::assert_snapshot!(name, util::run(spec, args));
}

#[test]
fn cat_cmd_handles_eof_in_message() {
    let styled = clap::builder::StyledStr::from("this contains EOF in the text\nEOF\nand more\n");
    let cmd = CatCmd::new(styled, ExitCode::Error);
    insta::assert_snapshot!(format!("{}", cmd));
}
