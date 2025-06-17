use clap::{Parser, Subcommand};
use claptrap::shell::Shell;
use std::ffi::OsString;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help(true))]
pub struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE", env = "CLAPTRAP_SPEC")]
    pub spec: PathBuf,

    /// The format of the spec file
    #[arg(long, value_name = "FORMAT", env = "CLAPTRAP_SPEC_FORMAT", default_value_t = SpecFormat::Auto)]
    pub spec_format: SpecFormat,

    /// Do not suppress panic messages
    #[arg(long)]
    pub show_panic: bool,

    /// The shell to format output for
    #[arg(long, value_enum, env = "CLAPTRAP_SHELL", default_value_t = Shell::Bash)]
    pub shell: Shell,

    #[command(subcommand)]
    pub command: Option<SubCommand>,

    /// Arguments to pass to the command
    #[arg(last = true)]
    pub args: Vec<OsString>,
}

#[derive(Subcommand)]
pub enum SubCommand {
    /// Generate shell completion
    Completion {
        /// The shell to generate completions for
        #[arg(value_enum)]
        shell: clap_complete::Shell,

        /// The output file for the completions
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,
    },
    /// Generate ROFF man page
    Man {
        /// The output file for the ROFF man page
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,
    },
    /// Generate a template script
    Script {
        /// The shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,

        /// The output file for the template script
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,
    },
    /// Generate documentation
    Doc {
        /// The format of the documentation
        #[arg(value_enum, short = 'f', long, default_value_t = DocFormat::Markdown)]
        format: DocFormat,

        /// The output file for the documentation
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,
    },
}

/// Spec file format.
#[derive(clap::ValueEnum, Debug, Copy, Clone, Default)]
#[non_exhaustive]
pub enum SpecFormat {
    /// Automatically detect the spec format
    #[default]
    Auto,
    /// JSON spec format
    Json,
    /// YAML spec format
    Yaml,
    /// TOML spec format
    Toml,
}

impl Display for SpecFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::Json => write!(f, "json"),
            Self::Yaml => write!(f, "yaml"),
            Self::Toml => write!(f, "toml"),
        }
    }
}

/// Documentation output format.
#[derive(clap::ValueEnum, Debug, Copy, Clone, Default)]
#[non_exhaustive]
pub enum DocFormat {
    /// Markdown formatted output
    #[default]
    Markdown,
}

impl Display for DocFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Markdown => write!(f, "markdown"),
        }
    }
}
