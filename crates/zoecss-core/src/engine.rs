//! Trait abstracting compiled configuration for CSS generation.
//!
//! Any configuration backend can implement `CssEngine` to plug into
//! the core `generate` function — ZoeCSS's `CompiledConfig` is one such
//! implementation, but external libraries can provide their own.

use crate::entries::CssEntries;
use crate::variant::Variant;

/// A compiled configuration backend that the core engine queries at generation time.
///
/// Implementors resolve utility tokens to CSS entries and look up variant
/// definitions. The core `generate` function is generic over this trait,
/// allowing external crates to provide their own configuration systems.
pub trait CssEngine {
    /// Resolves a base utility token (without variant prefixes) into CSS entries.
    ///
    /// Returns `None` when the token doesn't match any rule, or when a required
    /// substitution (captures, theme lookups) fails.
    fn resolve_token(&self, token: &str) -> Option<CssEntries>;

    /// O(1) lookup for a variant by name.
    fn get_variant(&self, name: &str) -> Option<&Variant>;
}
