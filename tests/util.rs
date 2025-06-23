use claptrap::command::Command;
use claptrap::parse;
use std::ffi::OsString;

#[macro_export]
macro_rules! case {
    ($path:expr, $name:expr) => {{
        (
            $name,
            include_str!(concat!("resources/", $path, "/", $name, ".toml")),
            include_str!(concat!("resources/", $path, "/", $name, ".args")),
        )
    }};
}

pub fn run(spec: &str, args: &str) -> claptrap::output::Output {
    let app: Command = toml::from_str(spec).unwrap();
    let args: Vec<OsString> = if args.trim().is_empty() {
        Vec::new()
    } else {
        args.split_whitespace().map(OsString::from).collect()
    };
    parse(app, args)
}
