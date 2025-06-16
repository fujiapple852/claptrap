use claptrap::command::Command;
use claptrap::parse;
use std::ffi::OsString;

#[test]
fn test_args() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            [groups]
            req_flags = { args = ["flag", "color"] }
            [args]
            flag = { short = 'f', action = "set-true" }
            color = { short = 'c', action = "set-true" }
        "#,
    )
    .unwrap();
    let input = "-f";
    let args: Vec<OsString> = input.split(' ').map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_multiple() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            [groups]
            req_flags = { args = ["flag", "color"], multiple = true }
            [args]
            flag = { short = 'f', action = "set-true" }
            color = { short = 'c', action = "set-true" }
        "#,
    )
    .unwrap();
    let input = "-f -c";
    let args: Vec<OsString> = input.split(' ').map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_required() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            [groups]
            req_flags = { args = ["flag", "color"], required = true }
            [args]
            flag = { short = 'f', action = "set-true" }
            color = { short = 'c', action = "set-true" }
        "#,
    )
    .unwrap();
    let args: Vec<OsString> = vec![];
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_requires() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            [groups]
            req_flags = { args = ["flag", "color"], requires = "debug" }
            [args]
            flag = { short = 'f', action = "set-true" }
            color = { short = 'c', action = "set-true" }
            debug = { short = 'd', action = "set-true" }
        "#,
    )
    .unwrap();
    let input = "-c";
    let args: Vec<OsString> = input.split(' ').map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_requires_all() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            [groups]
            req_flags = { args = ["flag", "color"], requires-all = ["debug", "verb"] }
            [args]
            flag = { short = 'f', action = "set-true" }
            color = { short = 'c', action = "set-true" }
            debug = { short = 'd', action = "set-true" }
            verb = { short = 'v', action = "set-true" }
        "#,
    )
    .unwrap();
    let input = "-c -d";
    let args: Vec<OsString> = input.split(' ').map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_conflicts_with() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            [groups]
            req_flags = { args = ["flag", "color"], conflicts-with = "debug" }
            [args]
            flag = { short = 'f', action = "set-true" }
            color = { short = 'c', action = "set-true" }
            debug = { short = 'd', action = "set-true" }
        "#,
    )
    .unwrap();
    let input = "-c -d";
    let args: Vec<OsString> = input.split(' ').map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_conflicts_with_all() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            [groups]
            req_flags = { args = ["flag", "color"], conflicts-with-all = ["debug", "verb"] }
            [args]
            flag = { short = 'f', action = "set-true" }
            color = { short = 'c', action = "set-true" }
            debug = { short = 'd', action = "set-true" }
            verb = { short = 'v', action = "set-true" }
        "#,
    )
    .unwrap();
    let input = "-c -v";
    let args: Vec<OsString> = input.split(' ').map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}
