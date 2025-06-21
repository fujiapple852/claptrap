use claptrap::command::Command;
use claptrap::parse;
use std::ffi::OsString;

#[test]
fn test_bool() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            foo = { long = "foo", value-parser = ":bool:" }
        "#,
    )
    .unwrap();
    let input = "--foo true";
    let args: Vec<OsString> = input.split(' ').map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_boolish() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            foo = { long = "foo", value-parser = ":boolish:" }
        "#,
    )
    .unwrap();
    let input = "--foo yes";
    let args: Vec<OsString> = input.split(' ').map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_falsey() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            foo = { long = "foo", value-parser = ":falsey:" }
        "#,
    )
    .unwrap();
    let input = "--foo no";
    let args: Vec<OsString> = input.split(' ').map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_string() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            foo = { long = "foo", value-parser = ":string:" }
        "#,
    )
    .unwrap();
    let input = "--foo bar";
    let args: Vec<OsString> = input.split(' ').map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_u8() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            foo = { long = "foo", value-parser = ":u8:" }
        "#,
    )
    .unwrap();
    let input = format!("--foo {}", u8::MAX);
    let args: Vec<OsString> = input.split(' ').map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_u16() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            foo = { long = "foo", value-parser = ":u16:" }
        "#,
    )
    .unwrap();
    let input = format!("--foo {}", u16::MAX);
    let args: Vec<OsString> = input.split(' ').map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_u32() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            foo = { long = "foo", value-parser = ":u32:" }
        "#,
    )
    .unwrap();
    let input = format!("--foo {}", u32::MAX);
    let args: Vec<OsString> = input.split(' ').map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_u64() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            foo = { long = "foo", value-parser = ":u64:" }
        "#,
    )
    .unwrap();
    let input = format!("--foo {}", u64::MAX);
    let args: Vec<OsString> = input.split(' ').map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_u128() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            foo = { long = "foo", value-parser = ":u128:" }
        "#,
    )
    .unwrap();
    let input = format!("--foo {}", u128::MAX);
    let args: Vec<OsString> = input.split(' ').map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_usize() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            foo = { long = "foo", value-parser = ":usize:" }
        "#,
    )
    .unwrap();
    let input = format!("--foo {}", usize::MAX);
    let args: Vec<OsString> = input.split(' ').map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_i8() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            foo = { long = "foo", value-parser = ":i8:", allow-negative-numbers = true }
        "#,
    )
    .unwrap();
    let input = format!("--foo {}", i8::MIN);
    let args: Vec<OsString> = input.split(' ').map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_i16() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            foo = { long = "foo", value-parser = ":i16:", allow-negative-numbers = true }
        "#,
    )
    .unwrap();
    let input = format!("--foo {}", i16::MIN);
    let args: Vec<OsString> = input.split(' ').map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_i32() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            foo = { long = "foo", value-parser = ":i32:", allow-negative-numbers = true }
        "#,
    )
    .unwrap();
    let input = format!("--foo {}", i32::MIN);
    let args: Vec<OsString> = input.split(' ').map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_i64() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            foo = { long = "foo", value-parser = ":i64:", allow-negative-numbers = true }
        "#,
    )
    .unwrap();
    let input = format!("--foo {}", i64::MIN);
    let args: Vec<OsString> = input.split(' ').map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_i128() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            foo = { long = "foo", value-parser = ":i128:", allow-negative-numbers = true }
        "#,
    )
    .unwrap();
    let input = format!("--foo {}", i128::MIN);
    let args: Vec<OsString> = input.split(' ').map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_isize() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            foo = { long = "foo", value-parser = ":isize:", allow-negative-numbers = true }
        "#,
    )
    .unwrap();
    let input = format!("--foo {}", isize::MIN);
    let args: Vec<OsString> = input.split(' ').map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_f32() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            foo = { long = "foo", value-parser = ":f32:" }
        "#,
    )
    .unwrap();
    let input = "--foo 3.14";
    let args: Vec<OsString> = input.split(' ').map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_f64() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            foo = { long = "foo", value-parser = ":f64:" }
        "#,
    )
    .unwrap();
    let input = "--foo 123456.78901234567";
    let args: Vec<OsString> = input.split(' ').map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_possible_values() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            foo = { long = "foo", value-parser = ["always", "auto", "never"] }
        "#,
    )
    .unwrap();
    let input = "--foo auto";
    let args: Vec<OsString> = input.split(' ').map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}
