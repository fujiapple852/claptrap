#![cfg(not(windows))]

use test_case::test_matrix;

const CLAPTRAP_BIN: &str = env!("CARGO_BIN_EXE_claptrap");

#[test_matrix(["bash", "zsh"]; "generate_template")]
fn test_generate_template(shell: &str) {
    let output = std::process::Command::new(CLAPTRAP_BIN)
        .arg("--spec")
        .arg("tests/resources/config/toml/myapp.toml")
        .arg("script")
        .arg(shell)
        .output()
        .expect("Failed to execute command");
    assert_eq!(Some(0), output.status.code());
    insta::assert_snapshot!(
        format!("test_generate_template_{shell}"),
        String::from_utf8_lossy(&output.stdout)
    );
}
