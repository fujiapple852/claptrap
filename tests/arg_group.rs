mod util;
use test_case::test_case;

#[test_case(case!("arg_group", "args"))]
#[test_case(case!("arg_group", "multiple"))]
#[test_case(case!("arg_group", "required"))]
#[test_case(case!("arg_group", "requires"))]
#[test_case(case!("arg_group", "requires_all"))]
#[test_case(case!("arg_group", "conflicts_with"))]
#[test_case(case!("arg_group", "conflicts_with_all"))]
fn test_arg_group((name, spec, args): (&str, &str, &str)) {
    insta::assert_snapshot!(name, util::run(spec, args));
}
