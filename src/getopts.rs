use crate::command::Command;
use anyhow::Result;

/// Build a `Command` specification from a classic getopts option string.
///
/// Supports single-character options. A trailing `:` after a character
/// indicates the option requires an argument. A double `::` marks the
/// argument as optional.
pub fn command_from_optstring(opt: &str) -> Result<Command> {
    let mut spec = String::from("name = \"app\"\n[args]\n");
    let mut chars = opt.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == ':' {
            // Leading ':' or stray ':' are ignored
            continue;
        }
        let mut action = "set-true";
        let mut num_args: Option<&str> = None;
        if let Some(':') = chars.peek() {
            chars.next();
            action = "set";
            if let Some(':') = chars.peek() {
                chars.next();
                num_args = Some("optional");
            }
        }
        spec.push_str(&format!(
            "\"{}\" = {{ short = '{}', action = \"{}\"",
            ch, ch, action
        ));
        if let Some(na) = num_args {
            spec.push_str(&format!(", num-args = \"{}\"", na));
        }
        spec.push_str(" }\n");
    }
    Ok(toml::from_str(&spec)?)
}
