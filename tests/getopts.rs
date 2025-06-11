use claptrap::getopts::command_from_optstring;
use claptrap::parse;
use std::ffi::OsString;

#[test]
fn test_basic_optstring() {
    let cmd = command_from_optstring("ab:c").unwrap();
    let args: Vec<OsString> = ["-a", "-b", "val"]
        .into_iter()
        .map(OsString::from)
        .collect();
    let output = parse(cmd, args);
    insta::assert_snapshot!(output);
}
