#![cfg(not(windows))]

use test_case::test_matrix;

const CLAPTRAP_BIN: &str = env!("CARGO_BIN_EXE_claptrap");

#[test_matrix(["bash", "zsh"], ["yaml", "json", "toml"]; "shell_show_usage")]
fn test_show_usage(shell: &str, format: &str) {
    let output = std::process::Command::new(format!("tests/resources/shell/{shell}/file.sh"))
        .env("CLAPTRAP_BIN", CLAPTRAP_BIN)
        .env(
            "CLAPTRAP_SPEC",
            format!("tests/resources/config/{format}/myapp.{format}"),
        )
        .output()
        .expect("Failed to execute command");
    assert_eq!(Some(2), output.status.code());
    insta::assert_snapshot!(
        format!("test_show_usage_{shell}_{format}"),
        String::from_utf8_lossy(&output.stdout)
    );
}

#[test_matrix(["bash", "zsh"], ["yaml", "json", "toml"]; "test_spec_file")]
fn test_spec_file(shell: &str, format: &str) {
    let output = std::process::Command::new(format!("tests/resources/shell/{shell}/file.sh"))
        .env("CLAPTRAP_BIN", CLAPTRAP_BIN)
        .env(
            "CLAPTRAP_SPEC",
            format!("tests/resources/config/{format}/myapp.{format}"),
        )
        .arg("--mode")
        .arg("stream")
        .arg("--protocol")
        .arg("udp")
        .output()
        .expect("Failed to execute command");
    assert_eq!(Some(0), output.status.code());
    insta::assert_snapshot!(
        format!("test_spec_file_{shell}_{format}"),
        String::from_utf8_lossy(&output.stdout)
    );
}

#[test_matrix(["bash", "zsh"], ["yaml", "json", "toml"]; "test_spec_stdin_redirect")]
fn test_spec_stdin_redirect(shell: &str, format: &str) {
    let output =
        std::process::Command::new(format!("tests/resources/shell/{shell}/stdin_redirect.sh"))
            .env("CLAPTRAP_BIN", CLAPTRAP_BIN)
            .env("CLAPTRAP_SPEC_FORMAT", format)
            .env(
                "CLAPTRAP_SPEC",
                format!("tests/resources/config/{format}/myapp.{format}"),
            )
            .arg("--mode")
            .arg("stream")
            .arg("--protocol")
            .arg("udp")
            .output()
            .expect("Failed to execute command");
    assert_eq!(Some(0), output.status.code());
    insta::assert_snapshot!(
        format!("test_spec_stdin_redirect_{shell}_{format}"),
        String::from_utf8_lossy(&output.stdout)
    );
}

#[test_matrix(["bash", "zsh"], ["yaml", "json", "toml"]; "test_spec_stdin_heredoc")]
fn test_spec_stdin_heredoc(shell: &str, format: &str) {
    let output = std::process::Command::new(format!(
        "tests/resources/shell/{shell}/stdin_heredoc_{format}.sh"
    ))
    .env("CLAPTRAP_BIN", CLAPTRAP_BIN)
    .arg("--mode")
    .arg("stream")
    .arg("--protocol")
    .arg("udp")
    .output()
    .expect("Failed to execute command");
    assert_eq!(Some(0), output.status.code());
    insta::assert_snapshot!(
        format!("test_spec_stdin_heredoc_{shell}_{format}"),
        String::from_utf8_lossy(&output.stdout)
    );
}

#[test_matrix(["bash", "zsh"]; "test_panic")]
fn test_panic(shell: &str) {
    let output = std::process::Command::new(format!("tests/resources/shell/{shell}/panic.sh"))
        .env("CLAPTRAP_BIN", CLAPTRAP_BIN)
        .output()
        .expect("Failed to execute command");
    assert_eq!(Some(3), output.status.code());
    insta::assert_snapshot!(String::from_utf8_lossy(&output.stdout));
}
