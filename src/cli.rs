use clap::{Args, Parser, Subcommand};
use std::ffi::OsString;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    version,
    about,
    arg_required_else_help(true),
    args_conflicts_with_subcommands(true),
    subcommand_negates_reqs(true)
)]
pub struct Cli {
    /// Sets a custom config file
    #[arg(
        short,
        long,
        required(true),
        value_name = "FILE",
        env = "CLAPTRAP_SPEC"
    )]
    pub spec: Option<PathBuf>,

    /// The format of the spec file
    #[arg(long, value_name = "FORMAT", env = "CLAPTRAP_SPEC_FORMAT", default_value_t = SpecFormat::Auto)]
    pub spec_format: SpecFormat,

    /// Do not suppress panic messages
    #[arg(long)]
    pub show_panic: bool,

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
        #[command(flatten)]
        spec: SpecInfo,

        /// The shell to generate completions for
        #[arg(value_enum)]
        shell: clap_complete::Shell,

        /// The output file for the completions
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,
    },
    /// Generate ROFF man page
    Man {
        #[command(flatten)]
        spec: SpecInfo,

        /// The output file for the ROFF man page
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,
    },
    /// Generate a template script
    Script {
        #[command(flatten)]
        spec: SpecInfo,

        /// The shell to generate template script for
        #[arg(value_enum)]
        shell: Shell,

        /// The output file for the template script
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,
    },
    /// Generate documentation
    Doc {
        #[command(flatten)]
        spec: SpecInfo,

        /// The format of the documentation
        #[arg(value_enum, short = 'f', long, default_value_t = DocFormat::Markdown)]
        format: DocFormat,

        /// The output file for the documentation
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,
    },
}

#[derive(Args, Debug, Clone)]
pub struct SpecInfo {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE", env = "CLAPTRAP_SPEC")]
    pub spec: PathBuf,

    /// The format of the spec file
    #[arg(long, value_name = "FORMAT", env = "CLAPTRAP_SPEC_FORMAT", default_value_t = SpecFormat::Auto)]
    pub spec_format: SpecFormat,
}

/// Shell with template script available.
#[derive(clap::ValueEnum, Debug, Clone)]
#[non_exhaustive]
pub enum Shell {
    /// Bourne Again `SHell` (bash)
    Bash,
    /// Friendly Interactive `SHell` (fish)
    Fish,
    /// `PowerShell`
    #[expect(clippy::enum_variant_names)]
    PowerShell,
    /// Z `SHell` (zsh)
    Zsh,
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
