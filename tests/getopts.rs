use claptrap::getopts::parse_getopts;
use claptrap::parse;
use std::ffi::OsString;

#[test]
fn test_parse_getopts() {
    let app = parse_getopts("ab:c::").unwrap();
    let args: Vec<OsString> = vec!["-a", "-b", "file", "-c", "opt"].into_iter().map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}
