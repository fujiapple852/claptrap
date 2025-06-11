use clap::{Parser, Subcommand};
use clap_complete::Shell;
use std::ffi::OsString;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help(true))]
pub struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE", env = "CLAPTRAP_SPEC")]
    pub spec: PathBuf,

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
        shell: Shell,

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
}
