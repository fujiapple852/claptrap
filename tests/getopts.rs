use claptrap::getopts::parse_getopts;

#[test]
fn parse_empty() {
    let cmd = parse_getopts("").unwrap();
    insta::assert_debug_snapshot!(cmd);
}

#[test]
fn parse_leading_colon_only() {
    let cmd = parse_getopts(":").unwrap();
    insta::assert_debug_snapshot!(cmd);
}

#[test]
fn parse_basic_spec() {
    let cmd = parse_getopts("ab:c::").unwrap();
    insta::assert_debug_snapshot!(cmd);
}

#[test]
fn error_triple_colon() {
    let err = parse_getopts("a:::").unwrap_err();
    insta::assert_snapshot!(err.to_string());
}

#[test]
fn error_stray_colon() {
    let err = parse_getopts("::").unwrap_err();
    insta::assert_snapshot!(err.to_string());
}

#[test]
fn error_whitespace() {
    let err = parse_getopts("a b").unwrap_err();
    insta::assert_snapshot!(err.to_string());
}
