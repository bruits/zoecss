//! Tailwind CSS compatible preset — rules, variants, theme, and preflight.

mod theme;

use regex::Regex;
use zoecss_config::{Preset, Rule};
use zoecss_core::{CssEntries, CssEntry, Theme, Variant};

/// Returns the Tailwind CSS compatible preset — a set of rules, variants,
/// theme values, and a modern CSS reset (preflight) that mirrors Tailwind CSS v4.
pub fn tailwindcss4() -> Preset {
    let mut preset = Preset::new("tailwindcss4");

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

    // Theme — Tailwind CSS v4 default theme

    theme::register_theme(&mut preset.theme);

    preset
        .base_css
        .push(include_str!("preflight.css").to_owned());

    preset.base_css.push(
        "\
@keyframes spin { to { transform: rotate(360deg); } }
@keyframes ping { 75%, 100% { transform: scale(2); opacity: 0; } }
@keyframes pulse { 50% { opacity: 0.5; } }
@keyframes bounce { 0%, 100% { transform: translateY(-25%); animation-timing-function: cubic-bezier(0.8, 0, 1, 1); } 50% { transform: none; animation-timing-function: cubic-bezier(0, 0, 0.2, 1); } }"
            .to_owned(),
    );

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

    fn compile_tailwindcss4() -> CompiledConfig {
        let mut config = Config::new();
        config.presets.push(tailwindcss4());
        CompiledConfig::compile(config.merge()).expect("tailwindcss4 preset compiles")
    }

    #[test]
    fn static_rule_flex() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "flex"),
            Some(".flex { display: flex; }".into())
        );
    }

    #[test]
    fn static_rule_hidden() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "hidden"),
            Some(".hidden { display: none; }".into())
        );
    }

    #[test]
    fn pattern_rule_padding() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "p-4"),
            Some(".p-4 { padding: 1rem; }".into())
        );
    }

    #[test]
    fn pattern_rule_margin() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "m-2"),
            Some(".m-2 { margin: 0.5rem; }".into())
        );
    }

    #[test]
    fn pattern_rule_unknown_theme_key() {
        let compiled = compile_tailwindcss4();
        assert_eq!(generate(&compiled, "p-99"), None);
    }

    #[test]
    fn dynamic_rule_arbitrary_color() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "text-[#ff0000]"),
            Some(".text-\\[\\#ff0000\\] { color: #ff0000; }".into())
        );
    }

    #[test]
    fn selector_variant() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "hover:flex"),
            Some(".hover\\:flex:hover { display: flex; }".into())
        );
    }

    #[test]
    fn at_rule_variant() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "sm:flex"),
            Some("@media (min-width: 640px) { .sm\\:flex { display: flex; } }".into())
        );
    }

    #[test]
    fn composed_variants() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "sm:hover:flex"),
            Some(
                "@media (min-width: 640px) { .sm\\:hover\\:flex:hover { display: flex; } }".into()
            )
        );
    }

    #[test]
    fn variant_with_pattern_rule() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "hover:p-4"),
            Some(".hover\\:p-4:hover { padding: 1rem; }".into())
        );
    }

    #[test]
    fn preset_has_base_css() {
        let preset = tailwindcss4();
        assert!(
            !preset.base_css.is_empty(),
            "tailwindcss4 preset should include preflight CSS"
        );
    }

    #[test]
    fn preset_base_css_contains_reset_selectors() {
        let preset = tailwindcss4();
        let css = preset.base_css.join("\n");
        assert!(
            css.contains("box-sizing: border-box"),
            "should contain box-sizing reset"
        );
        assert!(
            css.contains("::after"),
            "should contain universal selector reset"
        );
        assert!(
            css.contains("display: none !important"),
            "should contain hidden attribute reset"
        );
    }

    #[test]
    fn preset_has_default_font_theme() {
        let preset = tailwindcss4();
        assert!(
            preset.theme.sections.contains_key("default"),
            "tailwindcss4 preset should have a 'default' theme section"
        );
    }

    #[test]
    fn base_css_includes_keyframes() {
        let preset = tailwindcss4();
        let css = preset.base_css.join("\n");
        assert!(
            css.contains("@keyframes spin"),
            "should contain spin keyframes"
        );
        assert!(
            css.contains("@keyframes ping"),
            "should contain ping keyframes"
        );
        assert!(
            css.contains("@keyframes pulse"),
            "should contain pulse keyframes"
        );
        assert!(
            css.contains("@keyframes bounce"),
            "should contain bounce keyframes"
        );
    }
}
