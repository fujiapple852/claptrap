use anstream::ColorChoice;
use anyhow::anyhow;
use clap::builder::StyledStr;
use clap::Parser;
use claptrap::Command;
use cli::{DocFormat, Shell, SpecFormat, SpecInfo, SubCommand};
use minijinja::{context, Environment};
use output::{CatCmd, ExitCode, Output};
use std::ffi::OsString;
use std::io::Write;
use std::iter::once;
use std::panic;
use std::panic::AssertUnwindSafe;
use std::path::{Path, PathBuf};
use std::process::exit;
use template::template_values;

mod clap_ext;
mod cli;
mod error;
mod output;
mod parse;
mod template;
mod tests;

fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();
    match cli.command {
        Some(SubCommand::Completion {
            spec: SpecInfo { spec, spec_format },
            shell,
            output,
        }) => {
            run_generate_completions(&spec, spec_format, shell, output)?;
            exit(0);
        }
        Some(SubCommand::Man {
            spec: SpecInfo { spec, spec_format },
            output,
        }) => {
            run_generate_man(&spec, spec_format, output)?;
            exit(0);
        }
        Some(SubCommand::Script {
            spec: SpecInfo { spec, spec_format },
            shell,
            output,
        }) => {
            run_generate_template(&spec, spec_format, &shell, output)?;
            exit(0);
        }
        Some(SubCommand::Doc {
            spec: SpecInfo { spec, spec_format },
            format,
            output,
        }) => {
            run_generate_doc(&spec, spec_format, format, output)?;
            exit(0);
        }
        Some(SubCommand::Schema { output }) => {
            run_generate_schema(output)?;
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
                let spec = cli
                    .spec
                    .ok_or_else(|| anyhow!("internal error: spec was None"))?;
                run_app(&spec, cli.spec_format, cli.args)
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
    spec_path: &Path,
    spec_format: SpecFormat,
    shell: clap_complete::Shell,
    output: Option<PathBuf>,
) -> anyhow::Result<()> {
    let spec = parse_spec(spec_path, spec_format)?;
    let mut clap_cmd = clap::Command::from(spec);
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
    let clap_cmd = clap::Command::from(spec);
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
    spec_path: &Path,
    spec_format: SpecFormat,
    shell: &Shell,
    output: Option<PathBuf>,
) -> anyhow::Result<()> {
    let spec = parse_spec(spec_path, spec_format)?;
    let clap_cmd = clap::Command::from(spec);
    let template = match shell {
        Shell::Bash => Ok(include_str!("../templates/bash_template.sh.j2")),
        Shell::Zsh => Ok(include_str!("../templates/zsh_template.sh.j2")),
        _ => Err(anyhow::anyhow!(
            "Unsupported shell for script generation: {:?}",
            shell
        )),
    }?;
    let values = template_values(&clap_cmd);
    let mut env = Environment::new();
    env.set_trim_blocks(true);
    env.set_lstrip_blocks(true);
    env.add_template("script", template)?;
    let template = env.get_template("script")?;
    let rendered = template.render(context! { spec => spec_path, values => values })?;
    if let Some(output_path) = output {
        std::fs::write(output_path, rendered)?;
    } else {
        std::io::stdout().write_all(rendered.as_bytes())?;
    }
    Ok(())
}

fn run_generate_doc(
    spec_path: &Path,
    spec_format: SpecFormat,
    doc_format: DocFormat,
    output: Option<PathBuf>,
) -> anyhow::Result<()> {
    let spec = parse_spec(spec_path, spec_format)?;
    let clap_cmd = clap::Command::from(spec);
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

fn run_generate_schema(output: Option<PathBuf>) -> anyhow::Result<()> {
    let schema = schemars::schema_for!(Command);
    let bytes = serde_json::to_vec_pretty(&schema)?;
    if let Some(output_path) = output {
        std::fs::write(output_path, bytes)?;
    } else {
        std::io::stdout().write_all(&bytes)?;
    }
    Ok(())
}

fn run_app(
    spec_path: &Path,
    spec_format: SpecFormat,
    args: Vec<OsString>,
) -> error::Result<Output> {
    let spec = parse_spec(spec_path, spec_format)?;
    let name = spec.get_name().to_string();
    let args = once(OsString::from(name)).chain(args);
    Ok(parse::parse(spec, args))
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
