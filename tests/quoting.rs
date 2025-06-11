use claptrap::command::Command;
use claptrap::parse;
use std::ffi::OsString;

#[test]
fn single_value_with_quote() {
    let app: Command = toml::from_str(
        r#"name = "prog"
            [args]
            val = { long = "val" }
        "#,
    )
    .unwrap();
    let input = "--val foo'bar";
    let args: Vec<OsString> = input.split(' ').map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn many_values_special_chars() {
    let app: Command = toml::from_str(
        r#"name = "prog"
            [args]
            opt = { long = "opt", action = "append" }
        "#,
    )
    .unwrap();
    let input = "--opt val1 --opt val'2";
    let args: Vec<OsString> = input.split(' ').map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}
