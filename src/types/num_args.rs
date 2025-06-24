use serde::de::{self, Deserializer, Visitor};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::{Range, RangeFrom, RangeInclusive, RangeTo, RangeToInclusive};

/// Represents the number of arguments a command can take.
///
/// This enum is used to specify the number of arguments a command can accept in a
/// configuration file, such as a TOML file. It supports various formats including
/// exact counts, ranges, and special keywords like "empty", "single", and "optional".
///
/// It can be deserialized from integers, strings, or specific keywords, allowing for
/// flexible command argument definitions.
///
/// # Examples
/// ```toml
/// num-args = "empty"  # Represents 0 arguments
/// num-args = "single" # Represents exactly 1 argument
/// num-args = "optional" # Represents 0 or 1 arguments
/// num-args = 3 # Represents exactly 3 arguments
/// num-args = "3" # Represents exactly 3 arguments (as a string)
/// num-args = ".." # Represents an unbounded number of arguments
/// num-args = "1..3" # Represents a range of 1 to 2 arguments (inclusive lower, exclusive upper)
/// num-args = "1.." # Represents at least 1 argument (unbounded upper)
/// num-args = "..3" # Represents at most 2 arguments (exclusive lower, inclusive upper)
/// num-args = "1..=3" # Represents a range of 1 to 3 arguments (inclusive both ends)
/// num-args = "..=3" # Represents at most 3 arguments (inclusive upper)
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum NumArgs {
    /// Represents a specific number of arguments.
    ///
    /// I.e. `3`
    Exact(usize),
    /// An unbounded range (`..`).
    ///
    /// I.e. `..`
    RangeFull,
    /// A (half-open) range bounded inclusively below and exclusively above
    /// (`start..end`).
    ///
    /// I.e. `1..3`, `0..5`, etc.
    Range(Range<usize>),
    /// A range only bounded inclusively below (`start..`).
    ///
    /// I.e. `1..`, `2..`, etc.
    RangeFrom(RangeFrom<usize>),
    /// A range only bounded inclusively above (`..end`).
    ///
    /// I.e. `..3`, `..5`, etc.
    RangeTo(RangeTo<usize>),
    /// A range bounded inclusively below and above (`start..=end`).
    ///
    /// I.e. `1..=3`, `0..=5`, etc.
    RangeInclusive(RangeInclusive<usize>),
    /// A range only bounded inclusively above (`..=end`).
    ///
    /// I.e. `..=3`, `..=5`, etc.
    RangeToInclusive(RangeToInclusive<usize>),
}

impl Display for NumArgs {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exact(n) => write!(f, "{n}"),
            Self::RangeFull => write!(f, ".."),
            Self::Range(range) => write!(f, "{}..{}", range.start, range.end),
            Self::RangeFrom(range) => write!(f, "{}..", range.start),
            Self::RangeTo(range) => write!(f, "..{}", range.end),
            Self::RangeInclusive(range) => write!(f, "{}..={}", range.start(), range.end()),
            Self::RangeToInclusive(range) => write!(f, "..={}", range.end),
        }
    }
}

impl<'de> serde::Deserialize<'de> for NumArgs {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NumArgsVisitor;

        impl Visitor<'_> for NumArgsVisitor {
            type Value = NumArgs;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("an integer or a string range")
            }

            fn visit_i64<E>(self, value: i64) -> Result<NumArgs, E>
            where
                E: de::Error,
            {
                if value < 0 {
                    Err(E::custom("negative number of arguments is not allowed"))
                } else {
                    #[expect(clippy::cast_sign_loss)]
                    Ok(NumArgs::Exact(value as usize))
                }
            }

            fn visit_u64<E>(self, value: u64) -> Result<NumArgs, E>
            where
                E: de::Error,
            {
                Ok(NumArgs::Exact(value as usize))
            }

            fn visit_str<E>(self, v: &str) -> Result<NumArgs, E>
            where
                E: de::Error,
            {
                parse_num_args(v).map_err(E::custom)
            }
        }

        deserializer.deserialize_any(NumArgsVisitor)
    }
}

fn parse_num_args(value: &str) -> Result<NumArgs, String> {
    fn parse(value: &str) -> Result<NumArgs, String> {
        if value.contains("..")
            && !value.contains("..=")
            && !value.ends_with("..")
            && !value.starts_with("..")
        {
            let parts: Vec<_> = value.splitn(2, "..").collect();
            let start = parts[0]
                .parse::<usize>()
                .map_err(|_| "invalid lower bound".to_string())?;
            let end = parts[1]
                .parse::<usize>()
                .map_err(|_| "invalid upper bound".to_string())?;
            if start >= end {
                return Err("lower bound must be < upper bound".into());
            }
            Ok(NumArgs::Range(start..end))
        } else if value.ends_with("..") && value.len() > 2 {
            let start = value[..value.len() - 2]
                .parse::<usize>()
                .map_err(|_| "invalid lower bound".to_string())?;
            Ok(NumArgs::RangeFrom(start..))
        } else if value.starts_with("..") && !value.starts_with("..=") {
            let end = value[2..]
                .parse::<usize>()
                .map_err(|_| "invalid upper bound".to_string())?;
            Ok(NumArgs::RangeTo(..end))
        } else if value.contains("..=") && !value.starts_with("..=") {
            let parts: Vec<_> = value.splitn(2, "..=").collect();
            let start = parts[0]
                .parse::<usize>()
                .map_err(|_| "invalid lower bound".to_string())?;
            let end = parts[1]
                .parse::<usize>()
                .map_err(|_| "invalid upper bound".to_string())?;
            if start > end {
                return Err("lower bound must be <= upper bound".into());
            }
            Ok(NumArgs::RangeInclusive(start..=end))
        } else if let Some(stripped) = value.strip_prefix("..=") {
            let end = stripped
                .parse::<usize>()
                .map_err(|_| "invalid upper bound".to_string())?;
            Ok(NumArgs::RangeToInclusive(..=end))
        } else {
            value
                .parse::<usize>()
                .map(NumArgs::Exact)
                .map_err(|_| "unrecognised num_args format".into())
        }
    }
    match value {
        "empty" => Ok(NumArgs::Exact(0)),
        "single" => Ok(NumArgs::Exact(1)),
        "optional" => Ok(NumArgs::RangeToInclusive(..=1)),
        ".." => Ok(NumArgs::RangeFull),
        other => parse(other),
    }
}

impl From<NumArgs> for clap::builder::ValueRange {
    fn from(n: NumArgs) -> Self {
        match n {
            NumArgs::RangeFull => Self::new(..),
            NumArgs::Exact(n) => Self::new(n),
            NumArgs::Range(range) => Self::new(range),
            NumArgs::RangeFrom(range) => Self::new(range),
            NumArgs::RangeTo(range) => Self::new(range),
            NumArgs::RangeInclusive(range) => Self::new(range),
            NumArgs::RangeToInclusive(range) => Self::new(range),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use serde::de::Error;

    #[test_case::test_case(r#"num-args = "empty""#, Ok(NumArgs::Exact(0)); "empty")]
    #[test_case::test_case(r#"num-args = "single""#, Ok(NumArgs::Exact(1)); "single")]
    #[test_case::test_case(r#"num-args = "optional""#, Ok(NumArgs::RangeToInclusive(..=1)); "optional")]
    #[test_case::test_case(r"num-args = 3", Ok(NumArgs::Exact(3)); "exact integer")]
    #[test_case::test_case(r#"num-args = "3""#, Ok(NumArgs::Exact(3)); "exact string")]
    #[test_case::test_case(r#"num-args = "..""#, Ok(NumArgs::RangeFull); "full")]
    #[test_case::test_case(r#"num-args = "1..3""#, Ok(NumArgs::Range(1..3)); "range from non-zero")]
    #[test_case::test_case(r#"num-args = "0..3""#, Ok(NumArgs::Range(0..3)); "range from zero")]
    #[test_case::test_case(r#"num-args = "1..""#, Ok(NumArgs::RangeFrom(1..)); "range from non-zero open")]
    #[test_case::test_case(r#"num-args = "0..""#, Ok(NumArgs::RangeFrom(0..)); "range from zero open")]
    #[test_case::test_case(r#"num-args = "..3""#, Ok(NumArgs::RangeTo(..3)); "range to non-zero")]
    #[test_case::test_case(r#"num-args = "..0""#, Ok(NumArgs::RangeTo(..0)); "range to zero")]
    #[test_case::test_case(r#"num-args = "1..=3""#, Ok(NumArgs::RangeInclusive(1..=3)); "inclusive range from non-zero")]
    #[test_case::test_case(r#"num-args = "0..=3""#, Ok(NumArgs::RangeInclusive(0..=3)); "inclusive range from zero")]
    #[test_case::test_case(r#"num-args = "..=3""#, Ok(NumArgs::RangeToInclusive(..=3)); "to inclusive range to non-zero")]
    #[test_case::test_case(r#"num-args = "..=0""#, Ok(NumArgs::RangeToInclusive(..=0)); "to inclusive range to zero")]
    #[test_case::test_case(r#"num-args = "2..1""#, Err(toml::de::Error::custom("lower bound must be < upper bound")); "invalid range")]
    #[test_case::test_case(r#"num-args = "1..=0""#, Err(toml::de::Error::custom("lower bound must be <= upper bound")); "invalid inclusive range")]
    #[test_case::test_case(r#"num-args = "foo""#, Err(toml::de::Error::custom("unrecognised num_args format")); "unknown keyword")]
    #[test_case::test_case(r#"num-args = "4..2""#, Err(toml::de::Error::custom("lower bound must be < upper bound")); "range start >= end")]
    #[test_case::test_case(r#"num-args = "3..=1""#, Err(toml::de::Error::custom("lower bound must be <= upper bound")); "inclusive start > end")]
    #[test_case::test_case(r"num-args = -1", Err(toml::de::Error::custom("negative number of arguments is not allowed")); "negative integer")]
    fn test_deserialize(input: &str, expected: Result<NumArgs, toml::de::Error>) {
        #[derive(Deserialize)]
        #[serde(rename_all = "kebab-case")]
        struct Test {
            num_args: NumArgs,
        }
        let actual = toml::from_str::<Test>(input).map(|test| test.num_args);
        match (actual, expected) {
            (Ok(num_args1), Ok(num_args2)) => assert_eq!(num_args1, num_args2),
            (Ok(num_args), Err(_)) => panic!("expected Err(_), got {num_args}"),
            (Err(err), Ok(num_args)) => panic!("expected Ok({num_args}), got Err({err}"),
            (Err(err1), Err(err2)) => assert_eq!(err1.message(), err2.message()),
        }
    }
}
