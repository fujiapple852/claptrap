#![cfg(not(windows))]

const CLAPTRAP_BIN: &str = env!("CARGO_BIN_EXE_claptrap");

#[test]
fn bash_show_usage() {
    let output = std::process::Command::new("tests/resources/bash_file.sh")
        .env("CLAPTRAP_BIN", CLAPTRAP_BIN)
        .output()
        .expect("Failed to execute command");
    assert_eq!(Some(2), output.status.code());
    insta::assert_snapshot!(String::from_utf8_lossy(&output.stdout));
}

#[test]
fn bash_spec_file() {
    let output = std::process::Command::new("tests/resources/bash_file.sh")
        .env("CLAPTRAP_BIN", CLAPTRAP_BIN)
        .arg("--mode")
        .arg("stream")
        .arg("--protocol")
        .arg("udp")
        .output()
        .expect("Failed to execute command");
    assert_eq!(Some(0), output.status.code());
    insta::assert_snapshot!(String::from_utf8_lossy(&output.stdout));
}

#[test]
fn bash_spec_stdin_redirect() {
    let output = std::process::Command::new("tests/resources/bash_stdin_redirect.sh")
        .env("CLAPTRAP_BIN", CLAPTRAP_BIN)
        .arg("--mode")
        .arg("stream")
        .arg("--protocol")
        .arg("udp")
        .output()
        .expect("Failed to execute command");
    assert_eq!(Some(0), output.status.code());
    insta::assert_snapshot!(String::from_utf8_lossy(&output.stdout));
}

#[test]
fn bash_spec_stdin_heredoc() {
    let output = std::process::Command::new("tests/resources/bash_stdin_heredoc.sh")
        .env("CLAPTRAP_BIN", CLAPTRAP_BIN)
        .arg("--mode")
        .arg("stream")
        .arg("--protocol")
        .arg("udp")
        .output()
        .expect("Failed to execute command");
    assert_eq!(Some(0), output.status.code());
    insta::assert_snapshot!(String::from_utf8_lossy(&output.stdout));
}

#[test]
fn bash_panic() {
    let output = std::process::Command::new("tests/resources/bash_panic.sh")
        .env("CLAPTRAP_BIN", CLAPTRAP_BIN)
        .output()
        .expect("Failed to execute command");
    assert_eq!(Some(3), output.status.code());
    insta::assert_snapshot!(String::from_utf8_lossy(&output.stdout));
}
