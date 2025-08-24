use serde::de::value::BorrowedStrDeserializer;
use serde::Deserialize;

#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[non_exhaustive]
pub enum ValueParser {
    Bool,
    Boolish,
    Falsey,
    String,
    U8,
    U16,
    U32,
    U64,
    U128,
    Usize,
    I8,
    I16,
    I32,
    I64,
    I128,
    Isize,
    F32,
    F64,
    PossibleValues(Vec<PossibleValue>),
}

impl<'de> serde::Deserialize<'de> for ValueParser {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        fn parse_typed_value_parser(value: &str) -> Option<TypedValueParser> {
            value
                .strip_prefix(':')
                .and_then(|s| s.strip_suffix(':'))
                .and_then(|inner| {
                    TypedValueParser::deserialize(
                        BorrowedStrDeserializer::<serde::de::value::Error>::new(inner),
                    )
                    .ok()
                })
        }
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
                    Some(tv) => Ok(ValueParser::from(tv)),
                    None => Ok(ValueParser::PossibleValues(vec![PossibleValue::from(
                        value.to_string(),
                    )])),
                }
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element::<PossibleValue>()? {
                    values.push(value);
                }
                match values.as_slice() {
                    [PossibleValue::Value(pv)] => match parse_typed_value_parser(pv) {
                        Some(tv) => Ok(ValueParser::from(tv)),
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
            ValueParser::U8 => clap::value_parser!(u8).into(),
            ValueParser::U16 => clap::value_parser!(u16).into(),
            ValueParser::U32 => clap::value_parser!(u32).into(),
            ValueParser::U64 => clap::value_parser!(u64).into(),
            ValueParser::U128 => clap::value_parser!(u128).into(),
            ValueParser::Usize => clap::value_parser!(usize).into(),
            ValueParser::I8 => clap::value_parser!(i8).into(),
            ValueParser::I16 => clap::value_parser!(i16).into(),
            ValueParser::I32 => clap::value_parser!(i32).into(),
            ValueParser::I64 => clap::value_parser!(i64).into(),
            ValueParser::I128 => clap::value_parser!(i128).into(),
            ValueParser::Isize => clap::value_parser!(isize).into(),
            ValueParser::F32 => clap::value_parser!(f32).into(),
            ValueParser::F64 => clap::value_parser!(f64).into(),
            ValueParser::PossibleValues(values) => {
                let pvs = values
                    .into_iter()
                    .map(clap::builder::PossibleValue::from)
                    .collect::<Vec<_>>();
                Self::new(clap::builder::PossibleValuesParser::new(pvs))
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
#[non_exhaustive]
pub enum PossibleValue {
    Value(String),
    #[serde(rename_all = "kebab-case")]
    Detailed {
        name: String,
        help: Option<String>,
        alias: Option<String>,
        aliases: Option<Vec<String>>,
        hide: Option<bool>,
    },
}

impl From<PossibleValue> for clap::builder::PossibleValue {
    fn from(value: PossibleValue) -> Self {
        match value {
            PossibleValue::Value(v) => Self::new(v),
            PossibleValue::Detailed {
                name,
                help,
                alias,
                aliases,
                hide,
            } => {
                let mut pv = Self::new(name);
                if let Some(help) = help {
                    pv = pv.help(help);
                }
                if let Some(alias) = alias {
                    pv = pv.alias(alias);
                }
                if let Some(aliases) = aliases {
                    pv = pv.aliases(aliases);
                }
                if let Some(hide) = hide {
                    pv = pv.hide(hide);
                }
                pv
            }
        }
    }
}

impl From<String> for PossibleValue {
    fn from(value: String) -> Self {
        Self::Value(value)
    }
}

impl From<&str> for PossibleValue {
    fn from(value: &str) -> Self {
        Self::Value(value.to_string())
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
enum TypedValueParser {
    Bool,
    Boolish,
    Falsey,
    String,
    U8,
    U16,
    U32,
    U64,
    U128,
    Usize,
    I8,
    I16,
    I32,
    I64,
    I128,
    Isize,
    F32,
    F64,
}

impl From<TypedValueParser> for ValueParser {
    fn from(tv: TypedValueParser) -> Self {
        match tv {
            TypedValueParser::Bool => Self::Bool,
            TypedValueParser::Boolish => Self::Boolish,
            TypedValueParser::Falsey => Self::Falsey,
            TypedValueParser::String => Self::String,
            TypedValueParser::U8 => Self::U8,
            TypedValueParser::U16 => Self::U16,
            TypedValueParser::U32 => Self::U32,
            TypedValueParser::U64 => Self::U64,
            TypedValueParser::U128 => Self::U128,
            TypedValueParser::Usize => Self::Usize,
            TypedValueParser::I8 => Self::I8,
            TypedValueParser::I16 => Self::I16,
            TypedValueParser::I32 => Self::I32,
            TypedValueParser::I64 => Self::I64,
            TypedValueParser::I128 => Self::I128,
            TypedValueParser::Isize => Self::Isize,
            TypedValueParser::F32 => Self::F32,
            TypedValueParser::F64 => Self::F64,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    #[test_case(serde_json::json!(":bool:"), Ok(ValueParser::Bool); "bool magic")]
    #[test_case(serde_json::json!(":boolish:"), Ok(ValueParser::Boolish); "boolish magic")]
    #[test_case(serde_json::json!(":falsey:"), Ok(ValueParser::Falsey); "falsey magic")]
    #[test_case(serde_json::json!(":string:"), Ok(ValueParser::String); "string magic")]
    #[test_case(serde_json::json!(":u8:"), Ok(ValueParser::U8); "u8 magic")]
    #[test_case(serde_json::json!(":u16:"), Ok(ValueParser::U16); "u16 magic")]
    #[test_case(serde_json::json!(":u32:"), Ok(ValueParser::U32); "u32 magic")]
    #[test_case(serde_json::json!(":u64:"), Ok(ValueParser::U64); "u64 magic")]
    #[test_case(serde_json::json!(":u128:"), Ok(ValueParser::U128); "u128 magic")]
    #[test_case(serde_json::json!(":usize:"), Ok(ValueParser::Usize); "usize magic")]
    #[test_case(serde_json::json!(":i8:"), Ok(ValueParser::I8); "i8 magic")]
    #[test_case(serde_json::json!(":i16:"), Ok(ValueParser::I16); "i16 magic")]
    #[test_case(serde_json::json!(":i32:"), Ok(ValueParser::I32); "i32 magic")]
    #[test_case(serde_json::json!(":i64:"), Ok(ValueParser::I64); "i64 magic")]
    #[test_case(serde_json::json!(":i128:"), Ok(ValueParser::I128); "i128 magic")]
    #[test_case(serde_json::json!(":isize:"), Ok(ValueParser::Isize); "isize magic")]
    #[test_case(serde_json::json!(":f32:"), Ok(ValueParser::F32); "f32 magic")]
    #[test_case(serde_json::json!(":f64:"), Ok(ValueParser::F64); "f64 magic")]
    #[test_case(serde_json::json!([":bool:"]), Ok(ValueParser::Bool); "bool magic list")]
    #[test_case(serde_json::json!([":boolish:"]), Ok(ValueParser::Boolish); "boolish magic list")]
    #[test_case(serde_json::json!([":falsey:"]), Ok(ValueParser::Falsey); "falsey magic list")]
    #[test_case(serde_json::json!([":string:"]), Ok(ValueParser::String); "string magic list")]
    #[test_case(
        serde_json::json!(["foo", "bar"]),
        Ok(ValueParser::PossibleValues(vec![PossibleValue::from("foo"), PossibleValue::from("bar")]));
        "possible values"
    )]
    #[test_case(
        serde_json::json!([":bool:"]),
        Ok(ValueParser::Bool);
        "known single magic possible value"
    )]
    #[test_case(
        serde_json::json!([":unknown:"]),
        Ok(ValueParser::PossibleValues(vec![PossibleValue::from(":unknown:")]));
        "unknown single magic possible value"
    )]
    #[test_case(
        serde_json::json!([":bool:", ":falsey:"]),
        Ok(ValueParser::PossibleValues(vec![PossibleValue::from(":bool:"), PossibleValue::from(":falsey:")]));
        "known multiple magic values"
    )]
    #[test_case(
        serde_json::json!([":falsey"]),
        Ok(ValueParser::PossibleValues(vec![PossibleValue::from(":falsey")]));
        "prefix possible value"
    )]
    #[test_case(
        serde_json::json!(["falsey:"]),
        Ok(ValueParser::PossibleValues(vec![PossibleValue::from("falsey:")]));
        "suffix possible value"
    )]
    #[test_case(
        serde_json::json!([{"name":"fast", "help":"speed"}]),
        Ok(ValueParser::PossibleValues(vec![PossibleValue::Detailed { name: "fast".into(), help: Some("speed".into()), alias: None, aliases: None, hide: None }]));
        "possible value struct with help"
    )]
    #[test_case(serde_json::json!(false), Err(serde::de::Error::custom("invalid type: boolean `false`, expected a value parser type")); "not a sequence")]
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
