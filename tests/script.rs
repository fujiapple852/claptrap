#![cfg(not(windows))]

use test_case::test_matrix;

const CLAPTRAP_BIN: &str = env!("CARGO_BIN_EXE_claptrap");

#[test_matrix(["bash", "zsh"]; "test_script")]
fn test_script(shell: &str) {
    let output = std::process::Command::new(CLAPTRAP_BIN)
        .arg("script")
        .arg("--spec")
        .arg(format!("tests/resources/script/{shell}/spec.toml"))
        .arg(shell)
        .output()
        .expect("failed to run claptrap");
    assert_eq!(Some(0), output.status.code());
    insta::assert_snapshot!(
        format!("test_script_{shell}"),
        String::from_utf8_lossy(&output.stdout)
    );
}
