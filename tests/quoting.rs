#[path = "util.rs"]
mod util;
use test_case::test_case;

fn run(spec: &str, args: &str) -> claptrap::output::Output {
    util::run(spec, args)
}

#[test_case("single_value_with_quote", include_str!("resources/quoting/single_value.toml"), include_str!("resources/quoting/single_value.args"))]
#[test_case("many_values_special_chars", include_str!("resources/quoting/many_values.toml"), include_str!("resources/quoting/many_values.args"))]
fn test_quoting(name: &str, spec: &str, args: &str) {
    let output = run(spec, args);
    insta::assert_snapshot!(name, output);
}
