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
        let mut arg = Arg::new().short(ch);
        let mut action = ArgAction::SetTrue;
        if matches!(chars.peek(), Some(':')) {
            chars.next();
            action = ArgAction::Set;
            if matches!(chars.peek(), Some(':')) {
                chars.next();
                arg = arg.num_args(NumArgs::RangeToInclusive(..=1));
            } else {
                arg = arg.num_args(NumArgs::Exact(1));
            }
        }
        arg = arg.action(action);
        command = command.arg(ch.to_string(), arg);
    }
    Ok(command)
}
