mod util;
use test_case::test_case;

#[test_case(case!("quoting", "single_value_with_quote"))]
#[test_case(case!("quoting", "many_values_special_chars"))]
fn test_quoting((name, spec, args): (&str, &str, &str)) {
    insta::assert_snapshot!(name, util::run(spec, args));
}
