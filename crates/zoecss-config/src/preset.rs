//! Composable configuration presets — reusable bundles of rules, variants, and theme values.

use std::borrow::Cow;

use zoecss_core::{Theme, Variant};

use crate::CssEntry;
use crate::rule::Rule;

/// A composable configuration preset — a reusable bundle of rules, variants, and theme values.
#[derive(Debug, Clone)]
pub struct Preset {
    pub name: Cow<'static, str>,
    pub rules: Vec<Rule>,
    pub variants: Vec<Variant>,
    pub theme: Theme,
    pub base_css: Vec<String>,
    /// CSS custom property defaults for composable utilities (e.g. `--tw-translate-x: 0`).
    /// Collected from utility modules and emitted as a universal selector block during compilation.
    pub property_defaults: Vec<CssEntry>,
}

impl Preset {
    pub fn new(name: impl Into<Cow<'static, str>>) -> Self {
        Self {
            name: name.into(),
            rules: Vec::new(),
            variants: Vec::new(),
            theme: Theme::new(),
            base_css: Vec::new(),
            property_defaults: Vec::new(),
        }
    }
}

impl Default for Preset {
    fn default() -> Self {
        Self::new("default")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zoecss_core::{CssEntries, CssEntry};

    #[test]
    fn new_preset_is_empty() {
        let preset = Preset::new("test");
        assert_eq!(preset.name, "test");
        assert!(preset.rules.is_empty());
        assert!(preset.variants.is_empty());
        assert!(preset.theme.sections.is_empty());
        assert!(preset.base_css.is_empty());
        assert!(preset.property_defaults.is_empty());
    }

    #[test]
    fn default_preset() {
        let preset = Preset::default();
        assert_eq!(preset.name, "default");
    }

    #[test]
    fn add_rules_and_variants() {
        let mut preset = Preset::new("my-preset");
        preset.rules.push(Rule::Static {
            token: "flex".into(),
            entries: CssEntries::new(vec![CssEntry::new("display", "flex")]),
        });
        preset.variants.push(Variant::Selector {
            name: "hover".into(),
            template: "&:hover".into(),
        });
        assert_eq!(preset.rules.len(), 1);
        assert_eq!(preset.variants.len(), 1);
    }
}
