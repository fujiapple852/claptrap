#![cfg(not(windows))]

use std::os::unix::fs::PermissionsExt;
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

#[test_matrix(["bash", "zsh"]; "test_pacman_script")]
fn test_pacman_script(shell: &str) {
    let output = std::process::Command::new(CLAPTRAP_BIN)
        .arg("script")
        .arg("--spec")
        .arg("examples/pacman/spec.toml")
        .arg(shell)
        .output()
        .expect("failed to run claptrap");
    assert_eq!(Some(0), output.status.code());
    insta::assert_snapshot!(
        format!("test_pacman_script_{shell}"),
        String::from_utf8_lossy(&output.stdout)
    );
}

#[test_matrix(["bash", "zsh"]; "test_script_exec")]
fn test_script_exec(shell: &str) {
    let output = std::process::Command::new(CLAPTRAP_BIN)
        .arg("script")
        .arg("--spec")
        .arg(format!("tests/resources/script/{shell}/spec.toml"))
        .arg(shell)
        .output()
        .expect("failed to run claptrap");
    assert_eq!(Some(0), output.status.code());
    let script = String::from_utf8_lossy(&output.stdout);
    let mut file = tempfile::NamedTempFile::new().expect("temp file");
    std::io::Write::write_all(&mut file, script.as_bytes()).expect("write script");
    let path = file.into_temp_path();
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).expect("chmod");
    let path_env = format!(
        "{}:{}",
        std::path::Path::new(CLAPTRAP_BIN)
            .parent()
            .unwrap()
            .display(),
        std::env::var("PATH").unwrap()
    );
    let run = |args: &[&str]| {
        std::process::Command::new(&path)
            .args(args)
            .env("PATH", &path_env)
            .output()
            .expect("run script")
    };
    let mut output_text = String::new();
    let out = run(&["foo", "--multi1", "one", "two"]);
    assert_eq!(Some(0), out.status.code());
    output_text.push_str(&String::from_utf8_lossy(&out.stdout));
    output_text.push_str("--\n");
    let out = run(&["first", "bar", "--multi2", "x", "y"]);
    assert_eq!(Some(0), out.status.code());
    output_text.push_str(&String::from_utf8_lossy(&out.stdout));
    output_text.push_str("--\n");
    let out = run(&["first", "nested", "--arg3", "q", "--multi3", "m1", "m2"]);
    assert_eq!(Some(0), out.status.code());
    output_text.push_str(&String::from_utf8_lossy(&out.stdout));
    output_text.push_str("--\n");
    let out = run(&["another", "baz", "--multi4", "a", "b"]);
    assert_eq!(Some(0), out.status.code());
    output_text.push_str(&String::from_utf8_lossy(&out.stdout));
    insta::assert_snapshot!(format!("test_script_exec_{shell}"), output_text);
}
