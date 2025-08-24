#![forbid(unsafe_code)]
#![allow(
    rustdoc::broken_intra_doc_links,
    rustdoc::bare_urls,
    clippy::doc_markdown,
    clippy::doc_lazy_continuation
)]

mod clap_ext;
mod output;
mod parse;
mod template;
mod types;

pub use output::{CatCmd, ExitCode, Output, OutputFormat};
pub use parse::parse;
pub use template::template_values;
pub use types::Command;
