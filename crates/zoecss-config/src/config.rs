//! Top-level ZoeCSS configuration.

use crate::preset::Preset;
use crate::rule::Rule;
use crate::theme::Theme;
use crate::variant::Variant;

/// Top-level ZoeCSS configuration, aggregating presets and user overrides.
#[derive(Debug, Clone)]
pub struct Config {
    pub presets: Vec<Preset>,
    pub rules: Vec<Rule>,
    pub variants: Vec<Variant>,
    pub theme: Theme,
}

impl Config {
    pub fn new() -> Self {
        Self {
            presets: Vec::new(),
            rules: Vec::new(),
            variants: Vec::new(),
            theme: Theme::new(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entries::{CssEntries, CssEntry};

    #[test]
    fn default_config_is_empty() {
        let config = Config::default();
        assert!(config.presets.is_empty());
        assert!(config.rules.is_empty());
        assert!(config.variants.is_empty());
        assert!(config.theme.sections.is_empty());
    }

    #[test]
    fn config_with_preset() {
        let mut config = Config::new();
        let mut preset = Preset::new("base");
        preset.rules.push(Rule::Static {
            token: "block".into(),
            entries: CssEntries::new(vec![CssEntry::new("display", "block")]),
        });
        config.presets.push(preset);
        assert_eq!(config.presets.len(), 1);
        assert_eq!(config.presets[0].name, "base");
    }
}
