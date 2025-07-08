#![forbid(unsafe_code)]

pub mod output;
pub mod parse;
pub mod types;

/// Extension trait for `clap::Arg` to determine if it is many-valued.
trait IsManyEx {
    /// Returns true if the argument is many-valued.
    fn is_many(&self) -> bool;
}

impl IsManyEx for clap::Arg {
    fn is_many(&self) -> bool {
        self.get_num_args()
            .map(|r| r.max_values() > 1)
            .or_else(|| self.get_value_delimiter().map(|_| true))
            .unwrap_or(false)
    }
}
