use serde::de::{IntoDeserializer, Visitor};
use serde::{Deserialize, Deserializer, de};
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct Styles {
    header: Option<Style>,
    error: Option<Style>,
    usage: Option<Style>,
    literal: Option<Style>,
    placeholder: Option<Style>,
    valid: Option<Style>,
    invalid: Option<Style>,
    context: Option<Style>,
    context_value: Option<Style>,
}

impl From<Styles> for clap::builder::Styles {
    fn from(styles: Styles) -> Self {
        let mut builder = Self::plain();
        if let Some(header) = styles.header {
            builder = builder.header(anstyle::Style::from(header));
        }
        if let Some(error) = styles.error {
            builder = builder.error(anstyle::Style::from(error));
        }
        if let Some(usage) = styles.usage {
            builder = builder.usage(anstyle::Style::from(usage));
        }
        if let Some(literal) = styles.literal {
            builder = builder.literal(anstyle::Style::from(literal));
        }
        if let Some(placeholder) = styles.placeholder {
            builder = builder.placeholder(anstyle::Style::from(placeholder));
        }
        if let Some(valid) = styles.valid {
            builder = builder.valid(anstyle::Style::from(valid));
        }
        if let Some(invalid) = styles.invalid {
            builder = builder.invalid(anstyle::Style::from(invalid));
        }
        if let Some(context) = styles.context {
            builder = builder.context(anstyle::Style::from(context));
        }
        if let Some(context_value) = styles.context_value {
            builder = builder.context_value(anstyle::Style::from(context_value));
        }
        builder
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
struct Style {
    fg: Option<Color>,
    bg: Option<Color>,
    underline: Option<Color>,
    effects: Option<Effects>,
}

impl From<Style> for anstyle::Style {
    fn from(style: Style) -> Self {
        Self::new()
            .fg_color(style.fg.map(Into::into))
            .bg_color(style.bg.map(Into::into))
            .underline_color(style.underline.map(Into::into))
            .effects(style.effects.map(Into::into).unwrap_or_default())
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
#[non_exhaustive]
enum Color {
    Ansi(AnsiColor),
    Ansi256(Ansi256Color),
    Rgb(RgbColor),
}

impl<'de> serde::Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ColorVisitor;

        impl Visitor<'_> for ColorVisitor {
            type Value = Color;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("an integer or a string")
            }

            fn visit_i64<E>(self, value: i64) -> Result<Color, E>
            where
                E: de::Error,
            {
                if value < 0 {
                    Err(E::custom("negative number is not valid for color"))
                } else {
                    #[expect(clippy::cast_sign_loss)]
                    Ok(Color::Ansi256(Ansi256Color(value as u8)))
                }
            }

            fn visit_u64<E>(self, value: u64) -> Result<Color, E>
            where
                E: de::Error,
            {
                if value > u64::from(u8::MAX) {
                    Err(E::custom("value out of range for u8"))
                } else {
                    Ok(Color::Ansi256(Ansi256Color(value as u8)))
                }
            }

            fn visit_str<E>(self, v: &str) -> Result<Color, E>
            where
                E: de::Error,
            {
                if v.starts_with('#') && v.len() == 7 {
                    let r = u8::from_str_radix(&v[1..3], 16).map_err(de::Error::custom)?;
                    let g = u8::from_str_radix(&v[3..5], 16).map_err(de::Error::custom)?;
                    let b = u8::from_str_radix(&v[5..7], 16).map_err(de::Error::custom)?;
                    Ok(Color::Rgb(RgbColor(r, g, b)))
                } else {
                    AnsiColor::deserialize(v.to_ascii_lowercase().into_deserializer())
                        .map(Color::Ansi)
                }
            }
        }
        deserializer.deserialize_any(ColorVisitor)
    }
}

impl From<Color> for anstyle::Color {
    fn from(color: Color) -> Self {
        match color {
            Color::Ansi(c) => Self::Ansi(anstyle::AnsiColor::from(c)),
            Color::Ansi256(c) => Self::Ansi256(anstyle::Ansi256Color::from(c)),
            Color::Rgb(c) => Self::Rgb(anstyle::RgbColor::from(c)),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
enum AnsiColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl From<AnsiColor> for anstyle::AnsiColor {
    fn from(color: AnsiColor) -> Self {
        match color {
            AnsiColor::Black => Self::Black,
            AnsiColor::Red => Self::Red,
            AnsiColor::Green => Self::Green,
            AnsiColor::Yellow => Self::Yellow,
            AnsiColor::Blue => Self::Blue,
            AnsiColor::Magenta => Self::Magenta,
            AnsiColor::Cyan => Self::Cyan,
            AnsiColor::White => Self::White,
            AnsiColor::BrightBlack => Self::BrightBlack,
            AnsiColor::BrightRed => Self::BrightRed,
            AnsiColor::BrightGreen => Self::BrightGreen,
            AnsiColor::BrightYellow => Self::BrightYellow,
            AnsiColor::BrightBlue => Self::BrightBlue,
            AnsiColor::BrightMagenta => Self::BrightMagenta,
            AnsiColor::BrightCyan => Self::BrightCyan,
            AnsiColor::BrightWhite => Self::BrightWhite,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
struct Ansi256Color(u8);

impl From<Ansi256Color> for anstyle::Ansi256Color {
    fn from(color: Ansi256Color) -> Self {
        Self(color.0)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
struct RgbColor(u8, u8, u8);

impl From<RgbColor> for anstyle::RgbColor {
    fn from(color: RgbColor) -> Self {
        Self(color.0, color.1, color.2)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
#[non_exhaustive]
struct Effects(Vec<Effect>);

impl From<Effects> for anstyle::Effects {
    fn from(effects: Effects) -> Self {
        effects
            .0
            .into_iter()
            .fold(Self::new(), |mut effects, effect| {
                match effect {
                    Effect::Bold => effects = effects.insert(Self::BOLD),
                    Effect::Dimmed => effects = effects.insert(Self::DIMMED),
                    Effect::Italic => effects = effects.insert(Self::ITALIC),
                    Effect::Underline => effects = effects.insert(Self::UNDERLINE),
                    Effect::DoubleUnderline => effects = effects.insert(Self::DOUBLE_UNDERLINE),
                    Effect::CurlyUnderline => effects = effects.insert(Self::CURLY_UNDERLINE),
                    Effect::DottedUnderline => effects = effects.insert(Self::DOTTED_UNDERLINE),
                    Effect::DashedUnderline => effects = effects.insert(Self::DASHED_UNDERLINE),
                    Effect::Blink => effects = effects.insert(Self::BLINK),
                    Effect::Invert => effects = effects.insert(Self::INVERT),
                    Effect::Hidden => effects = effects.insert(Self::HIDDEN),
                    Effect::Strikethrough => effects = effects.insert(Self::STRIKETHROUGH),
                }
                effects
            })
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
enum Effect {
    Bold,
    Dimmed,
    Italic,
    Underline,
    DoubleUnderline,
    CurlyUnderline,
    DottedUnderline,
    DashedUnderline,
    Blink,
    Invert,
    Hidden,
    Strikethrough,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::style::Effect;
    use test_case::test_case;

    #[test_case(serde_json::json!("black"), Ok(Color::Ansi(AnsiColor::Black)); "black")]
    #[test_case(serde_json::json!("red"), Ok(Color::Ansi(AnsiColor::Red)); "red")]
    #[test_case(serde_json::json!("green"), Ok(Color::Ansi(AnsiColor::Green)); "green")]
    #[test_case(serde_json::json!("yellow"), Ok(Color::Ansi(AnsiColor::Yellow)); "yellow")]
    #[test_case(serde_json::json!("blue"), Ok(Color::Ansi(AnsiColor::Blue)); "blue")]
    #[test_case(serde_json::json!("magenta"), Ok(Color::Ansi(AnsiColor::Magenta)); "magenta")]
    #[test_case(serde_json::json!("cyan"), Ok(Color::Ansi(AnsiColor::Cyan)); "cyan")]
    #[test_case(serde_json::json!("white"), Ok(Color::Ansi(AnsiColor::White)); "white")]
    #[test_case(serde_json::json!("bright-black"), Ok(Color::Ansi(AnsiColor::BrightBlack)); "bright_black")]
    #[test_case(serde_json::json!("bright-red"), Ok(Color::Ansi(AnsiColor::BrightRed)); "bright_red")]
    #[test_case(serde_json::json!("bright-green"), Ok(Color::Ansi(AnsiColor::BrightGreen)); "bright_green")]
    #[test_case(serde_json::json!("bright-yellow"), Ok(Color::Ansi(AnsiColor::BrightYellow)); "bright_yellow")]
    #[test_case(serde_json::json!("bright-blue"), Ok(Color::Ansi(AnsiColor::BrightBlue)); "bright_blue")]
    #[test_case(serde_json::json!("bright-magenta"), Ok(Color::Ansi(AnsiColor::BrightMagenta)); "bright_magenta")]
    #[test_case(serde_json::json!("bright-cyan"), Ok(Color::Ansi(AnsiColor::BrightCyan)); "bright_cyan")]
    #[test_case(serde_json::json!("bright-white"), Ok(Color::Ansi(AnsiColor::BrightWhite)); "bright_white")]
    #[test_case(serde_json::json!("bLaCk"), Ok(Color::Ansi(AnsiColor::Black)); "mixed_case")]
    #[test_case(serde_json::json!("#000000"), Ok(Color::Rgb(RgbColor(0, 0, 0))); "black_rgb")]
    #[test_case(serde_json::json!("#ff0000"), Ok(Color::Rgb(RgbColor(255, 0, 0))); "red_rgb")]
    #[test_case(serde_json::json!("#00ff00"), Ok(Color::Rgb(RgbColor(0, 255, 0))); "green_rgb")]
    #[test_case(serde_json::json!("#ffff00"), Ok(Color::Rgb(RgbColor(255, 255, 0))); "yellow_rgb")]
    #[test_case(serde_json::json!("#0000ff"), Ok(Color::Rgb(RgbColor(0, 0, 255))); "blue_rgb")]
    #[test_case(serde_json::json!("#ff00ff"), Ok(Color::Rgb(RgbColor(255, 0, 255))); "magenta_rgb")]
    #[test_case(serde_json::json!("#00ffff"), Ok(Color::Rgb(RgbColor(0, 255, 255))); "cyan_rgb")]
    #[test_case(serde_json::json!("#ffffff"), Ok(Color::Rgb(RgbColor(255, 255, 255))); "white_rgb")]
    #[test_case(serde_json::json!(0), Ok(Color::Ansi256(Ansi256Color(0))); "0")]
    #[test_case(serde_json::json!(255), Ok(Color::Ansi256(Ansi256Color(255))); "255")]
    #[test_case(serde_json::json!(256), Err(serde::de::Error::custom("value out of range for u8")); "256")]
    #[test_case(serde_json::json!(-1), Err(serde::de::Error::custom("negative number is not valid for color")); "negative")]
    #[test_case(serde_json::json!("invalid"), Err(serde::de::Error::custom("unknown variant `invalid`, expected one of `black`, `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `white`, `bright-black`, `bright-red`, `bright-green`, `bright-yellow`, `bright-blue`, `bright-magenta`, `bright-cyan`, `bright-white`")); "invalid_color")]
    #[test_case(serde_json::json!("#xxaabb"), Err(serde::de::Error::custom("invalid digit found in string")); "invalid_hex_color")]
    fn test_color(value: serde_json::Value, expected: Result<Color, serde_json::Error>) {
        let actual: serde_json::Result<Color> = serde_json::from_value(value);
        match (actual, expected) {
            (Ok(actual), Ok(expected)) => assert_eq!(actual, expected),
            (Err(e), Err(expected)) => assert_eq!(e.to_string(), expected.to_string()),
            _ => panic!("Unexpected result"),
        }
    }

    #[test_case(serde_json::json!("bold"), Ok(Effect::Bold); "bold")]
    #[test_case(serde_json::json!("dimmed"), Ok(Effect::Dimmed); "dimmed")]
    #[test_case(serde_json::json!("italic"), Ok(Effect::Italic); "italic")]
    #[test_case(serde_json::json!("underline"), Ok(Effect::Underline); "underline")]
    #[test_case(serde_json::json!("double-underline"), Ok(Effect::DoubleUnderline); "double_underline")]
    #[test_case(serde_json::json!("curly-underline"), Ok(Effect::CurlyUnderline); "curly_underline")]
    #[test_case(serde_json::json!("dotted-underline"), Ok(Effect::DottedUnderline); "dotted_underline")]
    #[test_case(serde_json::json!("dashed-underline"), Ok(Effect::DashedUnderline); "dashed_underline")]
    #[test_case(serde_json::json!("blink"), Ok(Effect::Blink); "blink")]
    #[test_case(serde_json::json!("invert"), Ok(Effect::Invert); "invert")]
    #[test_case(serde_json::json!("hidden"), Ok(Effect::Hidden); "hidden")]
    #[test_case(serde_json::json!("strikethrough"), Ok(Effect::Strikethrough); "strikethrough")]
    #[test_case(serde_json::json!("invalid"), Err(serde::de::Error::custom("unknown variant `invalid`, expected one of `bold`, `dimmed`, `italic`, `underline`, `double-underline`, `curly-underline`, `dotted-underline`, `dashed-underline`, `blink`, `invert`, `hidden`, `strikethrough`")); "invalid_effect")]
    fn test_effect(value: serde_json::Value, expected: Result<Effect, serde_json::Error>) {
        let actual: serde_json::Result<Effect> = serde_json::from_value(value);
        match (actual, expected) {
            (Ok(actual), Ok(expected)) => assert_eq!(actual, expected),
            (Err(e), Err(expected)) => assert_eq!(e.to_string(), expected.to_string()),
            _ => panic!("Unexpected result"),
        }
    }
}
