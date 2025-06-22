#[path = "util.rs"]
mod util;
use test_case::test_case;

fn run(spec: &str, input: &str) -> claptrap::output::Output {
    util::run(spec, input)
}

#[test_case("bool", include_str!("resources/values/bool.toml"), include_str!("resources/values/bool.args"); "bool")]
#[test_case("boolish", include_str!("resources/values/boolish.toml"), include_str!("resources/values/boolish.args"); "boolish")]
#[test_case("falsey", include_str!("resources/values/falsey.toml"), include_str!("resources/values/falsey.args"); "falsey")]
#[test_case("string", include_str!("resources/values/string.toml"), include_str!("resources/values/string.args"); "string")]
#[test_case("u8", include_str!("resources/values/u8.toml"), include_str!("resources/values/u8.args"); "u8")]
#[test_case("u16", include_str!("resources/values/u16.toml"), include_str!("resources/values/u16.args"); "u16")]
#[test_case("u32", include_str!("resources/values/u32.toml"), include_str!("resources/values/u32.args"); "u32")]
#[test_case("u64", include_str!("resources/values/u64.toml"), include_str!("resources/values/u64.args"); "u64")]
#[test_case("u128", include_str!("resources/values/u128.toml"), include_str!("resources/values/u128.args"); "u128")]
#[test_case("usize", include_str!("resources/values/usize.toml"), include_str!("resources/values/usize.args"); "usize")]
#[test_case("i8", include_str!("resources/values/i8.toml"), include_str!("resources/values/i8.args"); "i8")]
#[test_case("i16", include_str!("resources/values/i16.toml"), include_str!("resources/values/i16.args"); "i16")]
#[test_case("i32", include_str!("resources/values/i32.toml"), include_str!("resources/values/i32.args"); "i32")]
#[test_case("i64", include_str!("resources/values/i64.toml"), include_str!("resources/values/i64.args"); "i64")]
#[test_case("i128", include_str!("resources/values/i128.toml"), include_str!("resources/values/i128.args"); "i128")]
#[test_case("isize", include_str!("resources/values/isize.toml"), include_str!("resources/values/isize.args"); "isize")]
#[test_case("f32", include_str!("resources/values/f32.toml"), include_str!("resources/values/f32.args"); "f32")]
#[test_case("f64", include_str!("resources/values/f64.toml"), include_str!("resources/values/f64.args"); "f64")]
#[test_case("possible_values", include_str!("resources/values/possible_values.toml"), include_str!("resources/values/possible_values.args"); "possible_values")]
fn test_values(name: &str, spec: &str, args: &str) {
    let output = run(spec, args);
    insta::assert_snapshot!(name, output);
}
