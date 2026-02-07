//! Error types for configuration compilation.

use thiserror::Error;

/// A type alias for results in this crate.
pub type Result<T> = std::result::Result<T, ConfigError>;

/// Errors that can occur during configuration compilation.
#[derive(Debug, Clone, Error)]
pub enum ConfigError {
    /// A rule contains an invalid regex pattern.
    #[error("invalid regex pattern '{pattern}': {message}")]
    InvalidRegex { pattern: String, message: String },
}
