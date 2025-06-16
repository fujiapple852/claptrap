use crate::cli::{Shell, SpecFormat, SubCommand};
use anstream::ColorChoice;
use clap::Parser;
use clap::builder::StyledStr;
use claptrap::command::Command;
use claptrap::output::{CatCmd, ExitCode, Output};
use claptrap::parse;
use std::ffi::OsString;
use std::io::Write;
use std::panic;
use std::panic::AssertUnwindSafe;
use std::path::{Path, PathBuf};
use std::process::exit;

mod cli;
mod error;

fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();
    match cli.command {
        Some(SubCommand::Completion { shell, output }) => {
            run_generate_completions(&cli.spec, cli.spec_format, shell, output)?;
            exit(0);
        }
        Some(SubCommand::Man { output }) => {
            run_generate_man(&cli.spec, cli.spec_format, output)?;
            exit(0);
        }
        Some(SubCommand::Script { shell, output }) => {
            run_generate_template(&cli.spec, cli.spec_format, shell, output)?;
            exit(0);
        }
        None => {
            // As we are being called from an 'eval' in a shell, we have to be
            // careful that everything we output is "eval safe". This includes
            // all errors from the tool and even panics.
            //
            // We use `catch_unwind` to catch panics and return them in a way that
            // is eval safe. If the user has set `--show-panic`, we will not suppress
            // the panic messages, which can be useful for debugging.
            if !cli.show_panic {
                panic::set_hook(Box::new(|_| {}));
            }
            let mut stdout =
                anstream::AutoStream::new(std::io::stdout().lock(), ColorChoice::Always);
            match panic::catch_unwind(AssertUnwindSafe(|| {
                run_app(&cli.spec, cli.spec_format, cli.args)
            })) {
                Ok(val) => match val {
                    Ok(output) => {
                        write!(stdout, "{output}")?;
                        stdout.flush()?;
                        exit(0);
                    }
                    Err(err) => {
                        write!(stdout, "{err}")?;
                        stdout.flush()?;
                        exit(0);
                    }
                },
                Err(err) => {
                    let panic = panic_output(err);
                    write!(stdout, "{panic}")?;
                    stdout.flush()?;
                    exit(0);
                }
            }
        }
    }
}

fn run_generate_completions(
    spec_path: &Path,
    spec_format: SpecFormat,
    shell: clap_complete::Shell,
    output: Option<PathBuf>,
) -> anyhow::Result<()> {
    let spec = parse_spec(spec_path, spec_format)?;
    let mut clap_cmd = clap::Command::from(spec).no_binary_name(true);
    let name = clap_cmd.get_name().to_string();
    let mut buffer: Vec<u8> = vec![];
    clap_complete::generate(shell, &mut clap_cmd, name, &mut buffer);
    if let Some(output_path) = output {
        std::fs::write(output_path, buffer)?;
    } else {
        std::io::stdout().write_all(&buffer)?;
    }
    Ok(())
}

fn run_generate_man(
    spec_path: &Path,
    spec_format: SpecFormat,
    output: Option<PathBuf>,
) -> anyhow::Result<()> {
    let spec = parse_spec(spec_path, spec_format)?;
    let clap_cmd = clap::Command::from(spec).no_binary_name(true);
    let mut buffer: Vec<u8> = vec![];
    clap_mangen::Man::new(clap_cmd).render(&mut buffer)?;
    if let Some(output_path) = output {
        std::fs::write(output_path, buffer)?;
    } else {
        std::io::stdout().write_all(&buffer)?;
    }
    Ok(())
}

#[derive(Debug)]
struct VarInfo {
    env_name: String,
    is_many: bool,
}

fn gather_var_info(cmd: &clap::Command, prefix: &mut Vec<String>, vars: &mut Vec<VarInfo>) {
    for arg in cmd.get_arguments() {
        let id = arg.get_id();
        if id == "help" || id == "version" {
            continue;
        }
        let is_many = arg
            .get_num_args()
            .map(|r| r.max_values() > 1)
            .or(arg.get_value_delimiter().map(|_| true))
            .unwrap_or(false);
        let env_name = if prefix.is_empty() {
            format!("claptrap_{}", id)
        } else {
            format!("claptrap_{}_{}", prefix.join("_"), id)
        };
        vars.push(VarInfo { env_name, is_many });
    }
    for sub in cmd.get_subcommands() {
        prefix.push(sub.get_name().to_string());
        gather_var_info(sub, prefix, vars);
        prefix.pop();
    }
}

fn run_generate_template(
    spec_path: &Path,
    spec_format: SpecFormat,
    shell: Shell,
    output: Option<PathBuf>,
) -> anyhow::Result<()> {
    let spec = parse_spec(spec_path, spec_format)?;
    let clap_cmd = clap::Command::from(spec).no_binary_name(true);
    let mut vars = Vec::new();
    gather_var_info(&clap_cmd, &mut Vec::new(), &mut vars);

    let header = match shell {
        Shell::Bash => Ok("#!/usr/bin/env bash"),
        Shell::Zsh => Ok("#!/usr/bin/env zsh"),
        _ => Err(anyhow::anyhow!(
            "Unsupported shell for boilerplate generation: {:?}",
            shell
        )),
    }?;

    let mut script = String::new();
    script.push_str(header);
    script.push_str("\n\nset -euo pipefail\n\n");
    script.push_str("if ! command -v claptrap &> /dev/null; then\n");
    script.push_str(
        "    echo \"claptrap command not found. Please install it first, see https://claptrap.cli.rs for instructions.\"\n",
    );
    script.push_str("    exit 1\nfi\n\n");

    let mut eval_cmd = String::from("eval \"$(claptrap");
    if !matches!(spec_format, SpecFormat::Auto) {
        eval_cmd.push_str(&format!(" --spec-format {}", spec_format));
    }
    eval_cmd.push_str(&format!(" --spec {} -- \"$@\")\"", spec_path.display()));
    script.push_str(&eval_cmd);
    script.push_str("\n\n");

    for var in vars {
        let label = var.env_name.trim_start_matches("claptrap_").replace('_', " ");
        if var.is_many {
            script.push_str(&format!(
                "if [ ${{#{var}[@]}} -gt 0 ]; then\n    echo \"{label}: ${{{var}[@]}}\"\nfi\n",
                var = var.env_name,
                label = label
            ));
        } else {
            script.push_str(&format!(
                "if [ -n \"${{{var}:-}}\" ]; then\n    echo \"{label}: ${{{var}}}\"\nfi\n",
                var = var.env_name,
                label = label
            ));
        }
    }

    if let Some(output_path) = output {
        std::fs::write(output_path, script)?;
    } else {
        std::io::stdout().write_all(script.as_bytes())?;
    }
    Ok(())
}

fn run_app(
    spec_path: &Path,
    spec_format: SpecFormat,
    args: Vec<OsString>,
) -> error::Result<Output> {
    Ok(parse(parse_spec(spec_path, spec_format)?, args))
}

fn parse_spec(spec_path: &Path, spec_format: SpecFormat) -> anyhow::Result<Command> {
    let spec = if spec_path == PathBuf::from("-") {
        std::io::read_to_string(std::io::stdin())?
    } else {
        std::fs::read_to_string(spec_path)?
    };
    Ok(match spec_format {
        SpecFormat::Auto => {
            if let Some(ext) = spec_path.extension().and_then(|s| s.to_str()) {
                match ext {
                    "json" => serde_json::from_str::<Command>(&spec)?,
                    "yaml" | "yml" => serde_yaml::from_str::<Command>(&spec)?,
                    "toml" => toml::from_str::<Command>(&spec)?,
                    _ => Err(anyhow::anyhow!("Unsupported spec format: {}", ext))?,
                }
            } else {
                toml::from_str::<Command>(&spec)?
            }
        }
        SpecFormat::Json => serde_json::from_str::<Command>(&spec)?,
        SpecFormat::Yaml => serde_yaml::from_str::<Command>(&spec)?,
        SpecFormat::Toml => toml::from_str::<Command>(&spec)?,
    })
}

fn panic_output(err: Box<dyn std::any::Any + Send>) -> Output {
    let panic_message = if let Some(message) = err.downcast_ref::<String>() {
        message
    } else if let Some(message) = err.downcast_ref::<&str>() {
        message
    } else {
        "An unexpected panic occurred"
    };
    Output::Cat(CatCmd::new(
        StyledStr::from(format!("{}\n", panic_message)),
        ExitCode::Panic,
    ))
}
