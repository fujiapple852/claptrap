use clap::{Parser, Subcommand};
use std::ffi::OsString;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help(true))]
pub struct Cli {
    /// Sets a custom config file
    #[arg(
        short,
        long,
        value_name = "FILE",
        env = "CLAPTRAP_SPEC",
        required_unless_present = "optstring"
    )]
    pub spec: Option<PathBuf>,

    /// Getopts/optstring specification
    #[arg(
        long,
        value_name = "STRING",
        env = "CLAPTRAP_OPTSTRING",
        required_unless_present = "spec"
    )]
    pub optstring: Option<String>,

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
    #[allow(clippy::enum_variant_names)]
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
            SpecFormat::Auto => write!(f, "auto"),
            SpecFormat::Json => write!(f, "json"),
            SpecFormat::Yaml => write!(f, "yaml"),
            SpecFormat::Toml => write!(f, "toml"),
        }
    }
}
