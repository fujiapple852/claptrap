#![cfg(not(windows))]

use test_case::test_matrix;

const CLAPTRAP_BIN: &str = env!("CARGO_BIN_EXE_claptrap");

#[test_matrix(["toml"]; "examples_git")]
fn test_example_git(format: &str) {
    let output = std::process::Command::new("../../examples/git/git.sh")
        .env("CLAPTRAP_BIN", CLAPTRAP_BIN)
        .env("CLAPTRAP_SPEC", format!("../../examples/git/spec.{format}"))
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

#[test_matrix(["toml", "yaml"]; "examples_pacman")]
fn test_example_pacman(format: &str) {
    let output = std::process::Command::new("../../examples/pacman/pacman.sh")
        .env("CLAPTRAP_BIN", CLAPTRAP_BIN)
        .env(
            "CLAPTRAP_SPEC",
            format!("../../examples/pacman/spec.{format}"),
        )
        .arg("--sync")
        .arg("-i")
        .arg("claptrap")
        .arg("trippy")
        .output()
        .expect("Failed to execute command");
    assert_eq!(Some(0), output.status.code());
    insta::assert_snapshot!(
        format!("examples_pacman_{format}"),
        String::from_utf8_lossy(&output.stdout)
    );
}
