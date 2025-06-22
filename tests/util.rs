use claptrap::command::Command;
use claptrap::parse;
use std::ffi::OsString;

/// Parses the provided command specification and executes it with the given
/// arguments.
///
/// # Panics
///
/// Panics if the TOML specification fails to deserialize.
pub fn run(spec: &str, args: &str) -> claptrap::output::Output {
    let app: Command = toml::from_str(spec).unwrap();
    let args: Vec<OsString> = if args.trim().is_empty() {
        Vec::new()
    } else {
        args.split_whitespace().map(OsString::from).collect()
    };
    parse(app, args)
}
