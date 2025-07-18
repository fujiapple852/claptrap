const CLAPTRAP_BIN: &str = env!("CARGO_BIN_EXE_claptrap");

#[test]
fn test_schema() {
    let output = std::process::Command::new(CLAPTRAP_BIN)
        .arg("schema")
        .output()
        .expect("failed to run claptrap");
    assert_eq!(Some(0), output.status.code());
    insta::assert_snapshot!("test_schema", String::from_utf8_lossy(&output.stdout));
}
