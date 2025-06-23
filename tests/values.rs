#[path = "util.rs"]
mod util;
use test_case::test_case;

#[test_case(case!("values", "bool"))]
#[test_case(case!("values", "boolish"))]
#[test_case(case!("values", "falsey"))]
#[test_case(case!("values", "string"))]
#[test_case(case!("values", "u8"))]
#[test_case(case!("values", "u16"))]
#[test_case(case!("values", "u32"))]
#[test_case(case!("values", "u64"))]
#[test_case(case!("values", "u128"))]
#[test_case(case!("values", "usize"))]
#[test_case(case!("values", "i8"))]
#[test_case(case!("values", "i16"))]
#[test_case(case!("values", "i32"))]
#[test_case(case!("values", "i64"))]
#[test_case(case!("values", "i128"))]
#[test_case(case!("values", "isize"))]
#[test_case(case!("values", "f32"))]
#[test_case(case!("values", "f64"))]
#[test_case(case!("values", "possible_values"))]
fn test_values((name, spec, args): (&str, &str, &str)) {
    insta::assert_snapshot!(name, util::run(spec, args));
}
