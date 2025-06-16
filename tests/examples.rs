#![cfg(not(windows))]

use test_case::test_matrix;

const CLAPTRAP_BIN: &str = env!("CARGO_BIN_EXE_claptrap");

#[test_matrix(["toml"]; "examples_git")]
fn test_example_git(format: &str) {
    let output = std::process::Command::new("examples/git/git.sh")
        .env("CLAPTRAP_BIN", CLAPTRAP_BIN)
        .env("CLAPTRAP_SPEC", format!("examples/git/spec.{format}"))
        .arg("clone")
        .arg("origin")
        .output()
        .expect("Failed to execute command");
    assert_eq!(Some(0), output.status.code());
    insta::assert_snapshot!(
        format!("examples_git_{format}"),
        String::from_utf8_lossy(&output.stdout)
    );
}
