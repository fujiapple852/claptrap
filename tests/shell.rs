const CLAPTRAP_BIN: &str = env!("CARGO_BIN_EXE_claptrap");

#[cfg(not(windows))]
mod posix {
    use crate::CLAPTRAP_BIN;
    use std::process::Command;
    use test_case::test_matrix;

    fn shell_cmd(shell: &str, script: &str) -> Command {
        let mut cmd = Command::new(format!("tests/resources/shell/{shell}/{script}.sh"));
        cmd.env("CLAPTRAP_BIN", CLAPTRAP_BIN);
        cmd
    }

    #[test_matrix(["bash", "zsh"], ["yaml", "json", "toml"]; "shell_show_usage")]
    fn test_show_usage(shell: &str, format: &str) {
        let output = shell_cmd(shell, "file")
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
        let output = shell_cmd(shell, "file")
            .env(
                "CLAPTRAP_SPEC",
                format!("tests/resources/config/{format}/myapp.{format}"),
            )
            .args(["--mode", "stream", "--protocol", "udp"])
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
        let output = shell_cmd(shell, "stdin_redirect")
            .env("CLAPTRAP_SPEC_FORMAT", format)
            .env(
                "CLAPTRAP_SPEC",
                format!("tests/resources/config/{format}/myapp.{format}"),
            )
            .args(["--mode", "stream", "--protocol", "udp"])
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
        let output = shell_cmd(shell, &format!("stdin_heredoc_{format}"))
            .args(["--mode", "stream", "--protocol", "udp"])
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
        let output = shell_cmd(shell, "panic")
            .output()
            .expect("Failed to execute command");
        assert_eq!(Some(3), output.status.code());
        insta::assert_snapshot!(
            format!("test_spec_panic_{shell}"),
            String::from_utf8_lossy(&output.stdout)
        );
    }
}

#[cfg(windows)]
mod windows {
    use crate::CLAPTRAP_BIN;
    use std::path::Path;
    use std::process::Command;
    use test_case::test_matrix;

    fn powershell() -> &'static str {
        if Path::new(r"C:\Program Files\PowerShell").exists() {
            "pwsh"
        } else {
            "powershell"
        }
    }

    fn shell_cmd(shell: &str, script: &str) -> Command {
        let mut cmd = Command::new(powershell());
        cmd.args([
            "-NoLogo",
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-File",
            &format!("tests/resources/shell/{shell}/{script}.ps1"),
        ]);
        cmd.env("CLAPTRAP_BIN", CLAPTRAP_BIN);
        cmd
    }

    #[test_matrix(["powershell"], ["yaml", "json", "toml"]; "shell_show_usage")]
    fn test_show_usage(shell: &str, format: &str) {
        let output = shell_cmd(shell, "file")
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

    #[test_matrix(["powershell"], ["yaml", "json", "toml"]; "test_spec_file")]
    fn test_spec_file(shell: &str, format: &str) {
        let output = shell_cmd(shell, "file")
            .env(
                "CLAPTRAP_SPEC",
                format!("tests/resources/config/{format}/myapp.{format}"),
            )
            .args(["--mode", "stream", "--protocol", "udp"])
            .output()
            .expect("Failed to execute command");
        assert_eq!(Some(0), output.status.code());
        insta::assert_snapshot!(
            format!("test_spec_file_{shell}_{format}"),
            String::from_utf8_lossy(&output.stdout)
        );
    }

    #[test_matrix(["powershell"], ["yaml", "json", "toml"]; "test_spec_stdin_heredoc")]
    fn test_spec_stdin_heredoc(shell: &str, format: &str) {
        let output = shell_cmd(shell, &format!("stdin_heredoc_{format}"))
            .args(["--mode", "stream", "--protocol", "udp"])
            .output()
            .expect("Failed to execute command");
        assert_eq!(Some(0), output.status.code());
        insta::assert_snapshot!(
            format!("test_spec_stdin_heredoc_{shell}_{format}"),
            String::from_utf8_lossy(&output.stdout)
        );
    }

    #[test_matrix(["powershell"]; "test_panic")]
    fn test_panic(shell: &str) {
        let output = shell_cmd(shell, "panic")
            .output()
            .expect("Failed to execute command");
        assert_eq!(Some(3), output.status.code());
        insta::assert_snapshot!(
            format!("test_spec_panic_{shell}"),
            String::from_utf8_lossy(&output.stdout)
        );
    }
}
