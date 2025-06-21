#![forbid(unsafe_code)]

use crate::cli::{DocFormat, Shell, SpecFormat, SubCommand};
use claptrap::getopts::parse_getopts;
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
    let spec = build_spec(&cli)?;
    match cli.command {
        Some(SubCommand::Completion { shell, output }) => {
            run_generate_completions(&spec, shell, output)?;
            exit(0);
        }
        Some(SubCommand::Man { output }) => {
            run_generate_man(&spec, output)?;
            exit(0);
        }
        Some(SubCommand::Script { shell, output }) => {
            run_generate_template(&shell, output)?;
            exit(0);
        }
        Some(SubCommand::Doc { format, output }) => {
            run_generate_doc(&spec, format, output)?;
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
            match panic::catch_unwind(AssertUnwindSafe(|| { run_app(&spec, cli.args) })) {
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
                    let panic = panic_output(&err);
                    write!(stdout, "{panic}")?;
                    stdout.flush()?;
                    exit(0);
                }
            }
        }
    }
}

fn run_generate_completions(
    spec: &Command,
    shell: clap_complete::Shell,
    output: Option<PathBuf>,
) -> anyhow::Result<()> {
    let mut clap_cmd = clap::Command::from(spec.clone()).no_binary_name(true);
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
    spec: &Command,
    output: Option<PathBuf>,
) -> anyhow::Result<()> {
    let clap_cmd = clap::Command::from(spec.clone()).no_binary_name(true);
    let mut buffer: Vec<u8> = vec![];
    clap_mangen::Man::new(clap_cmd).render(&mut buffer)?;
    if let Some(output_path) = output {
        std::fs::write(output_path, buffer)?;
    } else {
        std::io::stdout().write_all(&buffer)?;
    }
    Ok(())
}

fn run_generate_template(
    shell: &Shell,
    output: Option<PathBuf>,
) -> anyhow::Result<()> {
    let template = match shell {
        Shell::Bash => Ok(include_str!("../templates/bash_template.sh")),
        Shell::Zsh => Ok(include_str!("../templates/zsh_template.sh")),
        _ => Err(anyhow::anyhow!(
            "Unsupported shell for boilerplate generation: {:?}",
            shell
        )),
    }?;
    if let Some(output_path) = output {
        std::fs::write(output_path, template)?;
    } else {
        std::io::stdout().write_all(template.as_bytes())?;
    }
    Ok(())
}

fn run_generate_doc(
    spec: &Command,
    doc_format: DocFormat,
    output: Option<PathBuf>,
) -> anyhow::Result<()> {
    let clap_cmd = clap::Command::from(spec.clone()).no_binary_name(true);
    let markdown = clap_markdown::help_markdown_command(&clap_cmd);
    let bytes = match doc_format {
        DocFormat::Markdown => markdown.into_bytes(),
    };
    if let Some(output_path) = output {
        std::fs::write(output_path, bytes)?;
    } else {
        std::io::stdout().write_all(&bytes)?;
    }
    Ok(())
}

fn run_app(spec: &Command, args: Vec<OsString>) -> error::Result<Output> {
    Ok(parse(spec.clone(), args))
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

fn build_spec(cli: &cli::Cli) -> anyhow::Result<Command> {
    if let Some(getopts) = &cli.getopts {
        parse_getopts(getopts)
    } else if let Some(ref path) = cli.spec {
        parse_spec(path, cli.spec_format)
    } else {
        Err(anyhow::anyhow!("spec or getopts required"))
    }
}

fn panic_output(err: &Box<dyn std::any::Any + Send>) -> Output {
    let panic_message = if let Some(message) = err.downcast_ref::<String>() {
        message
    } else if let Some(message) = err.downcast_ref::<&str>() {
        message
    } else {
        "An unexpected panic occurred"
    };
    Output::Cat(CatCmd::new(
        StyledStr::from(format!("{panic_message}\n")),
        ExitCode::Panic,
    ))
}
