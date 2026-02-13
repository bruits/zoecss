//! Top-level ZoeCSS configuration.

use zoecss_core::{Theme, Variant};

use crate::CssEntry;
use crate::preset::Preset;
use crate::rule::Rule;

/// Top-level ZoeCSS configuration, aggregating presets and user overrides.
#[derive(Debug, Clone)]
pub struct Config {
    pub presets: Vec<Preset>,
    pub rules: Vec<Rule>,
    pub variants: Vec<Variant>,
    pub theme: Theme,
    pub base_css: Vec<String>,
    /// CSS custom property defaults for composable utilities.
    /// Merged from presets (in order) then user overrides, same as `base_css`.
    pub property_defaults: Vec<CssEntry>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            presets: Vec::new(),
            rules: Vec::new(),
            variants: Vec::new(),
            theme: Theme::new(),
            base_css: Vec::new(),
            property_defaults: Vec::new(),
        }
    }

    /// Flattens presets into the config, producing a fully resolved configuration.
    ///
    /// Presets are applied in Vec order (first = lowest priority), then user-level
    /// rules/variants/theme override on top. The returned config has an empty
    /// `presets` field since they have been flattened.
    pub fn merge(self) -> Self {
        let mut rules = Vec::new();
        let mut variants = Vec::new();
        let mut theme = Theme::new();
        let mut base_css = Vec::new();
        let mut property_defaults = Vec::new();

        for preset in self.presets {
            rules.extend(preset.rules);
            variants.extend(preset.variants);
            base_css.extend(preset.base_css);
            property_defaults.extend(preset.property_defaults);
            for (section, entries) in preset.theme.sections {
                theme.sections.entry(section).or_default().extend(entries);
            }
        }

        rules.extend(self.rules);
        variants.extend(self.variants);
        base_css.extend(self.base_css);
        property_defaults.extend(self.property_defaults);
        for (section, entries) in self.theme.sections {
            theme.sections.entry(section).or_default().extend(entries);
        }

        Self {
            presets: Vec::new(),
            rules,
            variants,
            theme,
            base_css,
            property_defaults,
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
    use zoecss_core::{CssEntries, CssEntry};

    #[test]
    fn default_config_is_empty() {
        let config = Config::default();
        assert!(config.presets.is_empty());
        assert!(config.rules.is_empty());
        assert!(config.variants.is_empty());
        assert!(config.theme.sections.is_empty());
        assert!(config.base_css.is_empty());
        assert!(config.property_defaults.is_empty());
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

    #[test]
    fn merge_empty_config() {
        let config = Config::new().merge();
        assert!(config.presets.is_empty());
        assert!(config.rules.is_empty());
        assert!(config.variants.is_empty());
        assert!(config.theme.sections.is_empty());
        assert!(config.base_css.is_empty());
        assert!(config.property_defaults.is_empty());
    }

    #[test]
    fn merge_single_preset() {
        let mut preset = Preset::new("base");
        preset.rules.push(Rule::Static {
            token: "flex".into(),
            entries: CssEntries::new(vec![CssEntry::new("display", "flex")]),
        });
        preset.variants.push(Variant::Selector {
            name: "hover".into(),
            template: "&:hover".into(),
        });
        preset.theme.insert("colors", "red", "#ef4444");

        let mut config = Config::new();
        config.presets.push(preset);
        let merged = config.merge();

        assert!(merged.presets.is_empty());
        assert_eq!(merged.rules.len(), 1);
        assert_eq!(merged.variants.len(), 1);
        assert_eq!(merged.theme.get("colors", "red"), Some("#ef4444"));
    }

    #[test]
    fn merge_multi_preset_rule_order() {
        let mut first = Preset::new("first");
        first.rules.push(Rule::Static {
            token: "a".into(),
            entries: CssEntries::new(vec![CssEntry::new("order", "1")]),
        });

        let mut second = Preset::new("second");
        second.rules.push(Rule::Static {
            token: "b".into(),
            entries: CssEntries::new(vec![CssEntry::new("order", "2")]),
        });

        let mut config = Config::new();
        config.presets = vec![first, second];
        let merged = config.merge();

        assert_eq!(merged.rules.len(), 2);
        match (&merged.rules[0], &merged.rules[1]) {
            (Rule::Static { token: t1, .. }, Rule::Static { token: t2, .. }) => {
                assert_eq!(t1.as_ref(), "a");
                assert_eq!(t2.as_ref(), "b");
            }
            _ => panic!("expected Static rules"),
        }
    }

    #[test]
    fn merge_multi_preset_variant_order() {
        let mut first = Preset::new("first");
        first.variants.push(Variant::Selector {
            name: "hover".into(),
            template: "&:hover".into(),
        });

        let mut second = Preset::new("second");
        second.variants.push(Variant::AtRule {
            name: "sm".into(),
            rule: "@media (min-width: 640px)".into(),
        });

        let mut config = Config::new();
        config.presets = vec![first, second];
        let merged = config.merge();

        assert_eq!(merged.variants.len(), 2);
        assert_eq!(
            merged.variants[0],
            Variant::Selector {
                name: "hover".into(),
                template: "&:hover".into(),
            }
        );
        assert_eq!(
            merged.variants[1],
            Variant::AtRule {
                name: "sm".into(),
                rule: "@media (min-width: 640px)".into(),
            }
        );
    }

    #[test]
    fn merge_user_rules_after_presets() {
        let mut preset = Preset::new("base");
        preset.rules.push(Rule::Static {
            token: "preset-rule".into(),
            entries: CssEntries::new(vec![CssEntry::new("a", "1")]),
        });

        let mut config = Config::new();
        config.presets.push(preset);
        config.rules.push(Rule::Static {
            token: "user-rule".into(),
            entries: CssEntries::new(vec![CssEntry::new("b", "2")]),
        });
        let merged = config.merge();

        assert_eq!(merged.rules.len(), 2);
        match (&merged.rules[0], &merged.rules[1]) {
            (Rule::Static { token: t1, .. }, Rule::Static { token: t2, .. }) => {
                assert_eq!(t1.as_ref(), "preset-rule");
                assert_eq!(t2.as_ref(), "user-rule");
            }
            _ => panic!("expected Static rules"),
        }
    }

    #[test]
    fn merge_user_variants_after_presets() {
        let mut preset = Preset::new("base");
        preset.variants.push(Variant::Selector {
            name: "hover".into(),
            template: "&:hover".into(),
        });

        let mut config = Config::new();
        config.presets.push(preset);
        config.variants.push(Variant::Selector {
            name: "focus".into(),
            template: "&:focus".into(),
        });
        let merged = config.merge();

        assert_eq!(merged.variants.len(), 2);
        assert_eq!(
            merged.variants[0],
            Variant::Selector {
                name: "hover".into(),
                template: "&:hover".into(),
            }
        );
        assert_eq!(
            merged.variants[1],
            Variant::Selector {
                name: "focus".into(),
                template: "&:focus".into(),
            }
        );
    }

    #[test]
    fn merge_theme_deep_merge_preserves_unrelated_keys() {
        let mut preset = Preset::new("base");
        preset.theme.insert("colors", "red", "#ef4444");
        preset.theme.insert("spacing", "1", "0.25rem");

        let mut config = Config::new();
        config.presets.push(preset);
        config.theme.insert("colors", "blue", "#3b82f6");
        let merged = config.merge();

        assert_eq!(merged.theme.get("colors", "red"), Some("#ef4444"));
        assert_eq!(merged.theme.get("colors", "blue"), Some("#3b82f6"));
        assert_eq!(merged.theme.get("spacing", "1"), Some("0.25rem"));
    }

    #[test]
    fn merge_theme_user_overrides_preset_key() {
        let mut preset = Preset::new("base");
        preset.theme.insert("colors", "red", "#ef4444");

        let mut config = Config::new();
        config.presets.push(preset);
        config.theme.insert("colors", "red", "#ff0000");
        let merged = config.merge();

        assert_eq!(merged.theme.get("colors", "red"), Some("#ff0000"));
    }

    #[test]
    fn merge_theme_later_preset_overrides_earlier() {
        let mut first = Preset::new("first");
        first.theme.insert("colors", "red", "first-red");
        first.theme.insert("colors", "green", "first-green");

        let mut second = Preset::new("second");
        second.theme.insert("colors", "red", "second-red");

        let mut config = Config::new();
        config.presets = vec![first, second];
        let merged = config.merge();

        assert_eq!(merged.theme.get("colors", "red"), Some("second-red"));
        assert_eq!(merged.theme.get("colors", "green"), Some("first-green"));
    }

    #[test]
    fn merge_no_deduplication() {
        let mut first = Preset::new("first");
        first.rules.push(Rule::Static {
            token: "flex".into(),
            entries: CssEntries::new(vec![CssEntry::new("display", "flex")]),
        });

        let mut second = Preset::new("second");
        second.rules.push(Rule::Static {
            token: "flex".into(),
            entries: CssEntries::new(vec![CssEntry::new("display", "flex")]),
        });

        let mut config = Config::new();
        config.presets = vec![first, second];
        let merged = config.merge();

        assert_eq!(merged.rules.len(), 2);
    }

    #[test]
    fn merge_user_only_no_presets() {
        let mut config = Config::new();
        config.rules.push(Rule::Static {
            token: "user-rule".into(),
            entries: CssEntries::new(vec![CssEntry::new("a", "1")]),
        });
        config.variants.push(Variant::Selector {
            name: "focus".into(),
            template: "&:focus".into(),
        });
        config.theme.insert("colors", "primary", "#000");
        let merged = config.merge();

        assert!(merged.presets.is_empty());
        assert_eq!(merged.rules.len(), 1);
        match &merged.rules[0] {
            Rule::Static { token, .. } => assert_eq!(token.as_ref(), "user-rule"),
            _ => panic!("expected Static rule"),
        }
        assert_eq!(merged.variants.len(), 1);
        assert_eq!(
            merged.variants[0],
            Variant::Selector {
                name: "focus".into(),
                template: "&:focus".into(),
            }
        );
        assert_eq!(merged.theme.get("colors", "primary"), Some("#000"));
    }

    #[test]
    fn merge_base_css_from_presets() {
        let mut first = Preset::new("first");
        first.base_css.push("/* reset */".into());

        let mut second = Preset::new("second");
        second.base_css.push("/* normalize */".into());

        let mut config = Config::new();
        config.presets = vec![first, second];
        let merged = config.merge();

        assert_eq!(merged.base_css, vec!["/* reset */", "/* normalize */"]);
    }

    #[test]
    fn merge_base_css_user_after_presets() {
        let mut preset = Preset::new("base");
        preset.base_css.push("/* preset base */".into());

        let mut config = Config::new();
        config.presets.push(preset);
        config.base_css.push("/* user base */".into());
        let merged = config.merge();

        assert_eq!(
            merged.base_css,
            vec!["/* preset base */", "/* user base */"]
        );
    }

    #[test]
    fn merge_base_css_empty_presets() {
        let preset = Preset::new("empty");
        let mut config = Config::new();
        config.presets.push(preset);
        let merged = config.merge();

        assert!(merged.base_css.is_empty());
    }

    #[test]
    fn merge_property_defaults_from_presets() {
        let mut first = Preset::new("first");
        first
            .property_defaults
            .push(CssEntry::new("--tw-translate-x", "0"));

        let mut second = Preset::new("second");
        second
            .property_defaults
            .push(CssEntry::new("--tw-scale-x", "1"));

        let mut config = Config::new();
        config.presets = vec![first, second];
        let merged = config.merge();

        assert_eq!(
            merged.property_defaults,
            vec![
                CssEntry::new("--tw-translate-x", "0"),
                CssEntry::new("--tw-scale-x", "1"),
            ]
        );
    }

    #[test]
    fn merge_property_defaults_user_after_presets() {
        let mut preset = Preset::new("base");
        preset
            .property_defaults
            .push(CssEntry::new("--tw-translate-x", "0"));

        let mut config = Config::new();
        config.presets.push(preset);
        config
            .property_defaults
            .push(CssEntry::new("--tw-translate-x", "10px"));
        let merged = config.merge();

        // User override appears after preset — last-wins during compilation.
        assert_eq!(
            merged.property_defaults,
            vec![
                CssEntry::new("--tw-translate-x", "0"),
                CssEntry::new("--tw-translate-x", "10px"),
            ]
        );
    }
}
