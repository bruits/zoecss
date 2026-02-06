//! Configuration model, merging, and compilation to runtime form.
//!
//! This crate defines the foundational types that all other ZoeCSS crates depend on.
//! It models CSS entries, rules (static, pattern, dynamic), themes, variants, presets,
//! and the top-level configuration.

pub mod compiled;
pub mod config;
pub mod entries;
pub mod preset;
pub mod rule;
pub mod theme;
pub mod variant;

pub use compiled::{CompiledConfig, CompiledRegexRule};
pub use config::Config;
pub use entries::{CssEntries, CssEntry};
pub use preset::Preset;
pub use rule::Rule;
pub use theme::Theme;
pub use variant::Variant;
