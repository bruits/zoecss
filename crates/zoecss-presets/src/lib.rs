//! Composable configuration presets (reusable config fragments).

use regex::Regex;
use zoecss_config::{Preset, Rule};
use zoecss_core::{CssEntries, CssEntry, Theme, Variant};

/// Returns the base preset — a minimal set of rules, variants, and theme values
/// that exercises all rule types and variant kinds.
pub fn base() -> Preset {
    let mut preset = Preset::new("base");

    // Static display utilities
    for (token, value) in [
        ("flex", "flex"),
        ("block", "block"),
        ("inline", "inline"),
        ("grid", "grid"),
        ("hidden", "none"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("display", value)]),
        });
    }

    // Pattern rules — spacing with theme lookup
    preset.rules.push(Rule::Pattern {
        pattern: r"^p-(.+)$".into(),
        template: CssEntries::new(vec![CssEntry::new("padding", "{theme.spacing.$1}")]),
    });
    preset.rules.push(Rule::Pattern {
        pattern: r"^m-(.+)$".into(),
        template: CssEntries::new(vec![CssEntry::new("margin", "{theme.spacing.$1}")]),
    });

    // Dynamic rule — arbitrary color via bracket syntax
    preset.rules.push(Rule::Dynamic {
        pattern: r"^text-\[(.+)\]$".into(),
        handler: handle_arbitrary_color,
    });

    // Variants
    preset.variants.push(Variant::Selector {
        name: "hover".into(),
        template: "&:hover".into(),
    });
    preset.variants.push(Variant::AtRule {
        name: "sm".into(),
        rule: "@media (min-width: 640px)".into(),
    });
    preset.variants.push(Variant::AtRule {
        name: "md".into(),
        rule: "@media (min-width: 768px)".into(),
    });

    // Theme — spacing
    preset.theme.insert("spacing", "1", "0.25rem");
    preset.theme.insert("spacing", "2", "0.5rem");
    preset.theme.insert("spacing", "4", "1rem");
    preset.theme.insert("spacing", "8", "2rem");

    // Theme — colors
    preset.theme.insert("colors", "red", "#ef4444");
    preset.theme.insert("colors", "blue", "#3b82f6");

    preset
}

fn handle_arbitrary_color(token: &str, _theme: &Theme) -> Option<CssEntries> {
    use std::sync::LazyLock;
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^text-\[(.+)\]$").expect("valid regex"));
    let caps = RE.captures(token)?;
    let color = caps.get(1)?.as_str();
    Some(CssEntries::new(vec![CssEntry::new(
        "color",
        color.to_owned(),
    )]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use zoecss_config::{CompiledConfig, Config};
    use zoecss_core::generate;

    fn compile_base() -> CompiledConfig {
        let mut config = Config::new();
        config.presets.push(base());
        CompiledConfig::compile(config.merge()).expect("base preset compiles")
    }

    #[test]
    fn static_rule_flex() {
        let compiled = compile_base();
        assert_eq!(
            generate(&compiled, "flex"),
            Some(".flex { display: flex; }".into())
        );
    }

    #[test]
    fn static_rule_hidden() {
        let compiled = compile_base();
        assert_eq!(
            generate(&compiled, "hidden"),
            Some(".hidden { display: none; }".into())
        );
    }

    #[test]
    fn pattern_rule_padding() {
        let compiled = compile_base();
        assert_eq!(
            generate(&compiled, "p-4"),
            Some(".p-4 { padding: 1rem; }".into())
        );
    }

    #[test]
    fn pattern_rule_margin() {
        let compiled = compile_base();
        assert_eq!(
            generate(&compiled, "m-2"),
            Some(".m-2 { margin: 0.5rem; }".into())
        );
    }

    #[test]
    fn pattern_rule_unknown_theme_key() {
        let compiled = compile_base();
        assert_eq!(generate(&compiled, "p-99"), None);
    }

    #[test]
    fn dynamic_rule_arbitrary_color() {
        let compiled = compile_base();
        assert_eq!(
            generate(&compiled, "text-[#ff0000]"),
            Some(".text-\\[\\#ff0000\\] { color: #ff0000; }".into())
        );
    }

    #[test]
    fn selector_variant() {
        let compiled = compile_base();
        assert_eq!(
            generate(&compiled, "hover:flex"),
            Some(".hover\\:flex:hover { display: flex; }".into())
        );
    }

    #[test]
    fn at_rule_variant() {
        let compiled = compile_base();
        assert_eq!(
            generate(&compiled, "sm:flex"),
            Some("@media (min-width: 640px) { .sm\\:flex { display: flex; } }".into())
        );
    }

    #[test]
    fn composed_variants() {
        let compiled = compile_base();
        assert_eq!(
            generate(&compiled, "sm:hover:flex"),
            Some(
                "@media (min-width: 640px) { .sm\\:hover\\:flex:hover { display: flex; } }".into()
            )
        );
    }

    #[test]
    fn variant_with_pattern_rule() {
        let compiled = compile_base();
        assert_eq!(
            generate(&compiled, "hover:p-4"),
            Some(".hover\\:p-4:hover { padding: 1rem; }".into())
        );
    }
}
