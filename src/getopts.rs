use crate::arg::{Arg, ArgAction};
use crate::command::Command;
use crate::num_args::NumArgs;

/// Parse a classic getopts specification string and build a [`Command`].
///
/// The returned `Command` will have a default name of `"prog"` and one
/// argument per option found in the getopts string.
pub fn parse_getopts(spec: &str) -> anyhow::Result<Command> {
    let mut chars = spec.chars().peekable();
    if chars.peek() == Some(&':') {
        // skip leading ':' which toggles silent error mode in `getopts`
        chars.next();
    }

    let mut command = Command::new("prog");
    while let Some(ch) = chars.next() {
        if ch == ':' || ch.is_whitespace() {
            return Err(anyhow::anyhow!("invalid option character: {ch}"));
        }

        let mut arg = Arg::new().short(ch);
        let mut action = ArgAction::SetTrue;

        let mut colon_count = 0;
        while matches!(chars.peek(), Some(':')) {
            colon_count += 1;
            chars.next();
        }

        match colon_count {
            0 => {}
            1 => {
                action = ArgAction::Set;
                arg = arg.num_args(NumArgs::Exact(1));
            }
            2 => {
                action = ArgAction::Set;
                arg = arg.num_args(NumArgs::RangeToInclusive(..=1));
            }
            _ => {
                return Err(anyhow::anyhow!(
                    "too many ':' after option {ch}"
                ));
            }
        }

        arg = arg.action(action);
        command = command.arg(ch.to_string(), arg);
    }

    Ok(command)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty() {
        let cmd = parse_getopts("").unwrap();
        assert_eq!(cmd, Command::new("prog"));
    }

    #[test]
    fn parse_leading_colon_only() {
        let cmd = parse_getopts(":").unwrap();
        assert_eq!(cmd, Command::new("prog"));
    }

    #[test]
    fn parse_basic_spec() {
        let cmd = parse_getopts("ab:c::").unwrap();

        let expected = Command::new("prog")
            .arg(
                "a",
                Arg::new()
                    .short('a')
                    .action(ArgAction::SetTrue),
            )
            .arg(
                "b",
                Arg::new()
                    .short('b')
                    .num_args(NumArgs::Exact(1))
                    .action(ArgAction::Set),
            )
            .arg(
                "c",
                Arg::new()
                    .short('c')
                    .num_args(NumArgs::RangeToInclusive(..=1))
                    .action(ArgAction::Set),
            );

        assert_eq!(cmd, expected);
    }

    #[test]
    fn error_triple_colon() {
        let err = parse_getopts("a:::").unwrap_err();
        assert!(err.to_string().contains("too many ':'"));
    }

    #[test]
    fn error_stray_colon() {
        let err = parse_getopts("::").unwrap_err();
        assert!(err.to_string().contains("invalid option character"));
    }

    #[test]
    fn error_whitespace() {
        let err = parse_getopts("a b").unwrap_err();
        assert!(err.to_string().contains("invalid option character"));
    }
}
