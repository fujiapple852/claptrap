use serde::Deserialize;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ValueParser {
    Bool,
    Boolish,
    Falsey,
    String,
    PossibleValues(Vec<String>),
}

impl<'de> Deserialize<'de> for ValueParser {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ValueParserVisitor;

        impl<'de> serde::de::Visitor<'de> for ValueParserVisitor {
            type Value = ValueParser;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a value parser type")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match parse_typed_value_parser(value) {
                    Some(TypedValueParser::Bool) => Ok(ValueParser::Bool),
                    Some(TypedValueParser::Boolish) => Ok(ValueParser::Boolish),
                    Some(TypedValueParser::Falsey) => Ok(ValueParser::Falsey),
                    Some(TypedValueParser::String) => Ok(ValueParser::String),
                    None => Ok(ValueParser::PossibleValues(vec![value.to_string()])),
                }
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element::<String>()? {
                    values.push(value);
                }
                match values.as_slice() {
                    [magic] => match parse_typed_value_parser(magic) {
                        Some(TypedValueParser::Bool) => Ok(ValueParser::Bool),
                        Some(TypedValueParser::Boolish) => Ok(ValueParser::Boolish),
                        Some(TypedValueParser::Falsey) => Ok(ValueParser::Falsey),
                        Some(TypedValueParser::String) => Ok(ValueParser::String),
                        None => Ok(ValueParser::PossibleValues(values)),
                    },
                    _ => Ok(ValueParser::PossibleValues(values)),
                }
            }
        }
        deserializer.deserialize_any(ValueParserVisitor)
    }
}

impl From<ValueParser> for clap::builder::ValueParser {
    fn from(value_parser: ValueParser) -> Self {
        match value_parser {
            ValueParser::Bool => Self::new(clap::builder::BoolValueParser::new()),
            ValueParser::Boolish => Self::new(clap::builder::BoolishValueParser::new()),
            ValueParser::Falsey => Self::new(clap::builder::FalseyValueParser::new()),
            ValueParser::String => Self::new(clap::builder::StringValueParser::new()),
            ValueParser::PossibleValues(values) => {
                Self::new(clap::builder::PossibleValuesParser::new(values))
            }
        }
    }
}

#[derive(Debug, Clone, strum::EnumString)]
#[strum(serialize_all = "kebab-case")]
enum TypedValueParser {
    Bool,
    Boolish,
    Falsey,
    String,
}

fn parse_typed_value_parser(value: &str) -> Option<TypedValueParser> {
    value
        .strip_prefix(':')
        .and_then(|s| s.strip_suffix(':'))
        .and_then(|inner| TypedValueParser::from_str(inner).ok())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::de::Error;
    use test_case::test_case;

    #[test_case(serde_json::json!(":bool:"), Ok(ValueParser::Bool); "bool magic")]
    #[test_case(serde_json::json!(":boolish:"), Ok(ValueParser::Boolish); "boolish magic")]
    #[test_case(serde_json::json!(":falsey:"), Ok(ValueParser::Falsey); "falsey magic")]
    #[test_case(serde_json::json!(":string:"), Ok(ValueParser::String); "string magic")]
    #[test_case(serde_json::json!([":bool:"]), Ok(ValueParser::Bool); "bool magic list")]
    #[test_case(serde_json::json!([":boolish:"]), Ok(ValueParser::Boolish); "boolish magic list")]
    #[test_case(serde_json::json!([":falsey:"]), Ok(ValueParser::Falsey); "falsey magic list")]
    #[test_case(serde_json::json!([":string:"]), Ok(ValueParser::String); "string magic list")]
    #[test_case(serde_json::json!(["foo", "bar"]), Ok(ValueParser::PossibleValues(vec!["foo".to_string(), "bar".to_string()])); "possible values")]
    #[test_case(serde_json::json!([":unknown:"]), Ok(ValueParser::PossibleValues(vec![":unknown:".to_string()])); "unknown single magic possible value")]
    #[test_case(serde_json::json!([":bool:", ":falsey:"]), Ok(ValueParser::PossibleValues(vec![":bool:".to_string(), ":falsey:".to_string()])); "known multiple magic values")]
    #[test_case(serde_json::json!([":falsey"]), Ok(ValueParser::PossibleValues(vec![":falsey".to_string()])); "prefix possible value")]
    #[test_case(serde_json::json!(["falsey:"]), Ok(ValueParser::PossibleValues(vec!["falsey:".to_string()])); "suffix possible value")]
    #[test_case(serde_json::json!(false), Err(serde::de::value::Error::custom("invalid type: boolean `false`, expected a value parser type")); "not a sequence")]
    fn test_value_parser(
        value: serde_json::Value,
        expected: Result<ValueParser, serde::de::value::Error>,
    ) {
        let actual: serde_json::Result<ValueParser> = serde_json::from_value(value);
        match (actual, expected) {
            (Ok(actual), Ok(expected)) => assert_eq!(actual, expected),
            (Err(e), Err(expected)) => assert_eq!(e.to_string(), expected.to_string()),
            _ => panic!("Unexpected result"),
        }
    }
}
