use crate::cli::SubCommand;
use anstream::ColorChoice;
use clap::Parser;
use clap::builder::StyledStr;
use clap_complete::Shell;
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
            run_generate_completions(&cli.spec, shell, output)?;
            exit(0);
        }
        Some(SubCommand::Man { output }) => {
            run_generate_man(&cli.spec, output)?;
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
            match panic::catch_unwind(AssertUnwindSafe(|| run_app(&cli.spec, cli.args))) {
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
    shell: Shell,
    output: Option<PathBuf>,
) -> anyhow::Result<()> {
    let spec = read_spec(spec_path)?;
    let cmd = toml::from_str::<Command>(&spec)?;
    let mut clap_cmd = clap::Command::from(cmd).no_binary_name(true);
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

fn run_generate_man(spec_path: &Path, output: Option<PathBuf>) -> anyhow::Result<()> {
    let spec = read_spec(spec_path)?;
    let cmd = toml::from_str::<Command>(&spec)?;
    let clap_cmd = clap::Command::from(cmd).no_binary_name(true);
    let mut buffer: Vec<u8> = vec![];
    clap_mangen::Man::new(clap_cmd).render(&mut buffer)?;
    if let Some(output_path) = output {
        std::fs::write(output_path, buffer)?;
    } else {
        std::io::stdout().write_all(&buffer)?;
    }
    Ok(())
}

fn run_app(spec_path: &Path, args: Vec<OsString>) -> error::Result<Output> {
    let spec = read_spec(spec_path)?;
    let cmd = toml::from_str::<Command>(&spec)?;
    Ok(parse(cmd, args))
}

fn read_spec(spec: &Path) -> std::io::Result<String> {
    Ok(if spec == PathBuf::from("-") {
        std::io::read_to_string(std::io::stdin())?
    } else {
        std::fs::read_to_string(spec)?
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
