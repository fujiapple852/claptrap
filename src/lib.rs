#![forbid(unsafe_code)]
#![allow(
    rustdoc::broken_intra_doc_links,
    rustdoc::bare_urls,
    clippy::doc_markdown,
    clippy::doc_lazy_continuation
)]
#![doc = include_str!("../README.md")]

mod types;
pub use types::{Command, ValueParser};
