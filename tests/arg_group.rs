#[path = "util.rs"]
mod util;
use test_case::test_case;

fn run(name: &str, spec: &str, args: &str) -> String {
    let output = util::run(spec, args);
    let mut rendered = format!("{output}");
    if name == "arg" {
        rendered = rendered
            .replace("'true'", "true")
            .replace("'false'", "false");
    }
    rendered
}

#[test_case("arg", include_str!("resources/arg_group/args.toml"), include_str!("resources/arg_group/args.args"))]
#[test_case("multiple", include_str!("resources/arg_group/multiple.toml"), include_str!("resources/arg_group/multiple.args"))]
#[test_case("required", include_str!("resources/arg_group/required.toml"), include_str!("resources/arg_group/required.args"))]
#[test_case("requires", include_str!("resources/arg_group/requires.toml"), include_str!("resources/arg_group/requires.args"))]
#[test_case("requires_all", include_str!("resources/arg_group/requires_all.toml"), include_str!("resources/arg_group/requires_all.args"))]
#[test_case("conflicts_with", include_str!("resources/arg_group/conflicts_with.toml"), include_str!("resources/arg_group/conflicts_with.args"))]
#[test_case("conflicts_with_all", include_str!("resources/arg_group/conflicts_with_all.toml"), include_str!("resources/arg_group/conflicts_with_all.args"))]
fn test_arg_group(name: &str, spec: &str, args: &str) {
    let output = run(name, spec, args);
    insta::assert_snapshot!(name, output);
}
