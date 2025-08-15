const CLAPTRAP_BIN: &str = env!("CARGO_BIN_EXE_claptrap");

#[cfg(not(windows))]
mod posix {
    use crate::CLAPTRAP_BIN;
    use std::os::unix::fs::PermissionsExt;
    use test_case::test_matrix;

    #[test_matrix(["bash", "zsh", "powershell", "fish"]; "test_script")]
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

    #[test_matrix(["bash", "zsh", "powershell", "fish"]; "test_pacman_script")]
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

    #[test_matrix(["bash", "zsh", "fish"]; "test_script_exec")]
    fn test_script_exec(shell: &str) {
        let output = std::process::Command::new(CLAPTRAP_BIN)
            .arg("script")
            .arg("--spec")
            .arg(format!("tests/resources/script/{shell}/spec.toml"))
            .arg(shell)
            .output()
            .expect("failed to run claptrap");
        assert_eq!(Some(0), output.status.code());
        let script = String::from_utf8_lossy(&output.stdout).to_string();
        let path_env = format!(
            "{}:{}",
            std::path::Path::new(CLAPTRAP_BIN)
                .parent()
                .unwrap()
                .display(),
            std::env::var("PATH").unwrap()
        );
        let run = |args: &[&str]| {
            let dir = tempfile::tempdir().expect("temp dir");
            let path = dir.path().join("script");
            std::fs::write(&path, script.as_bytes()).expect("write script");
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).expect("chmod");
            let output = std::process::Command::new(&path)
                .args(args)
                .env("PATH", &path_env)
                .output()
                .expect("run script");
            drop(dir);
            output
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
}

#[cfg(windows)]
mod windows {
    use crate::CLAPTRAP_BIN;
    use std::path::Path;
    use std::process::Command;

    fn powershell() -> &'static str {
        if Path::new(r"C:\Program Files\PowerShell").exists() {
            "pwsh"
        } else {
            "powershell"
        }
    }

    #[test]
    fn test_script_exec() {
        let output = Command::new(CLAPTRAP_BIN)
            .arg("script")
            .arg("--spec")
            .arg("tests/resources/script/powershell/spec.toml")
            .arg("powershell")
            .output()
            .expect("failed to run claptrap");
        assert_eq!(Some(0), output.status.code());
        let script = String::from_utf8_lossy(&output.stdout);
        let path = {
            let mut file = tempfile::NamedTempFile::with_suffix(".ps1").expect("temp file");
            std::io::Write::write_all(&mut file, script.as_bytes()).expect("write script");
            file.as_file_mut().sync_all().expect("sync script");
            file.into_temp_path()
        };
        let path_str = path.to_string_lossy().to_string();
        let path_env = format!(
            "{};{}",
            std::path::Path::new(CLAPTRAP_BIN)
                .parent()
                .unwrap()
                .display(),
            std::env::var("PATH").unwrap()
        );
        let run = |args: &[&str]| {
            Command::new(powershell())
                .args([
                    "-NoLogo",
                    "-NoProfile",
                    "-ExecutionPolicy",
                    "Bypass",
                    "-File",
                    &path_str,
                ])
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
        insta::assert_snapshot!("test_script_exec_powershell", output_text);
    }
}
