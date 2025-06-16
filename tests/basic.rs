use claptrap::command::Command;
use claptrap::parse;
use claptrap::output::{CatCmd, ExitCode};
use std::ffi::OsString;

#[test]
fn it_outputs_usage_and_exit_2_on_no_args() {
    let spec = include_str!("resources/config/toml/myapp.toml");
    let app: Command = toml::from_str(spec).unwrap();
    let args: Vec<OsString> = vec![];
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn it_outputs_usage_and_exit_0_on_short_help() {
    let spec = include_str!("resources/config/toml/myapp.toml");
    let app: Command = toml::from_str(spec).unwrap();
    let args: Vec<OsString> = vec!["-h".into()];
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn it_outputs_usage_and_exit_0_on_long_help() {
    let spec = include_str!("resources/config/toml/myapp.toml");
    let app: Command = toml::from_str(spec).unwrap();
    let args: Vec<OsString> = vec!["--help".into()];
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn it_outputs_version_and_exit_0_on_short_version() {
    let spec = include_str!("resources/config/toml/myapp.toml");
    let app: Command = toml::from_str(spec).unwrap();
    let args: Vec<OsString> = vec!["-V".into()];
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn it_outputs_variables() {
    let spec = include_str!("resources/config/toml/myapp.toml");
    let app: Command = toml::from_str(spec).unwrap();
    let args: Vec<OsString> = vec!["--mode".into(), "stream".into(), "-p".into(), "udp".into()];
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn it_outputs_error_and_exit_1_on_unexpected_arg() {
    let spec = include_str!("resources/config/toml/myapp.toml");
    let app: Command = toml::from_str(spec).unwrap();
    let args: Vec<OsString> = vec!["--invalid".into()];
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn cat_cmd_handles_eof_in_message() {
    let styled = clap::builder::StyledStr::from("this contains EOF in the text\nEOF\nand more\n");
    let cmd = CatCmd::new(styled, ExitCode::Error);
    insta::assert_snapshot!(format!("{}", cmd));
}
