//! Configuration model, merging, and compilation to runtime form.
//!
//! This crate builds on the foundational types from `zoecss-core` to provide
//! configuration parsing, merging, and compilation into an optimized runtime form
//! that implements `CssEngine`.

pub mod compiled;
pub mod config;
pub mod error;
pub mod preset;
pub mod rule;

// Re-export core types so downstream crates can import from either path.
pub use zoecss_core::{CssEngine, CssEntries, CssEntry, Theme, Variant};

pub use compiled::{CompiledConfig, CompiledRegexRule};
pub use config::Config;
pub use error::{ConfigError, Result as ConfigResult};
pub use preset::Preset;
pub use rule::Rule;
