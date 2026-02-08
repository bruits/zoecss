use regex::Regex;
use zoecss_config::{Preset, Rule};
use zoecss_core::{CssEntries, CssEntry, Theme};

/// Registers all color utility rules (text, background, border, outline, accent,
/// caret, fill, stroke, decoration) consuming `{theme.color.$1}`.
///
/// Directional border-color patterns are registered before the shorthand `border-(.+)`
/// so the engine's first-match-wins strategy hits the specific pattern first.
pub fn register(preset: &mut Preset) {
    let base_groups: &[(&str, &str)] = &[
        ("text", "color"),
        ("bg", "background-color"),
        ("border", "border-color"),
        ("outline", "outline-color"),
        ("accent", "accent-color"),
        ("caret", "caret-color"),
        ("fill", "fill"),
        ("stroke", "stroke"),
        ("decoration", "text-decoration-color"),
    ];

    let border_directional: &[(&str, &str)] = &[
        ("border-x", "border-inline-color"),
        ("border-y", "border-block-color"),
        ("border-t", "border-top-color"),
        ("border-r", "border-right-color"),
        ("border-b", "border-bottom-color"),
        ("border-l", "border-left-color"),
        ("border-s", "border-inline-start-color"),
        ("border-e", "border-inline-end-color"),
    ];

    // Static keyword rules for base groups
    for &(prefix, property) in base_groups {
        for (keyword, value) in [
            ("inherit", "inherit"),
            ("current", "currentColor"),
            ("transparent", "transparent"),
        ] {
            preset.rules.push(Rule::Static {
                token: format!("{prefix}-{keyword}").into(),
                entries: CssEntries::new(vec![CssEntry::new(property, value)]),
            });
        }
    }

    // SVG-specific `none` keyword — fill/stroke only, not applicable to other color groups.
    for &(prefix, property) in &[("fill", "fill"), ("stroke", "stroke")] {
        preset.rules.push(Rule::Static {
            token: format!("{prefix}-none").into(),
            entries: CssEntries::new(vec![CssEntry::new(property, "none")]),
        });
    }

    // `accent-auto` resets accent-color to its initial value.
    preset.rules.push(Rule::Static {
        token: "accent-auto".into(),
        entries: CssEntries::new(vec![CssEntry::new("accent-color", "auto")]),
    });

    // Static keyword rules for directional border-color
    for &(prefix, property) in border_directional {
        for (keyword, value) in [
            ("inherit", "inherit"),
            ("current", "currentColor"),
            ("transparent", "transparent"),
        ] {
            preset.rules.push(Rule::Static {
                token: format!("{prefix}-{keyword}").into(),
                entries: CssEntries::new(vec![CssEntry::new(property, value)]),
            });
        }
    }

    // Dynamic rule — arbitrary text color via bracket syntax (must precede generic pattern)
    preset.rules.push(Rule::Dynamic {
        pattern: r"^text-\[(.+)\]$".into(),
        handler: handle_arbitrary_color,
    });

    // Pattern rules — directional border-color before shorthand
    for &(prefix, property) in border_directional {
        preset.rules.push(Rule::Pattern {
            pattern: format!(r"^{prefix}-(.+)$").into(),
            template: CssEntries::new(vec![CssEntry::new(property, "{theme.color.$1}")]),
        });
    }

    // Pattern rules — base groups (border shorthand is included here, after directional)
    for &(prefix, property) in base_groups {
        preset.rules.push(Rule::Pattern {
            pattern: format!(r"^{prefix}-(.+)$").into(),
            template: CssEntries::new(vec![CssEntry::new(property, "{theme.color.$1}")]),
        });
    }
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
    use zoecss_config::{CompiledConfig, Config};
    use zoecss_core::generate;

    use crate::tailwindcss4;

    fn compile_tailwindcss4() -> CompiledConfig {
        let mut config = Config::new();
        config.presets.push(tailwindcss4());
        CompiledConfig::compile(config.merge()).expect("tailwindcss4 preset compiles")
    }

    #[test]
    fn text_color() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "text-red-500"),
            Some(".text-red-500 { color: oklch(63.7% 0.237 25.331); }".into())
        );
    }

    #[test]
    fn bg_color() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "bg-red-500"),
            Some(".bg-red-500 { background-color: oklch(63.7% 0.237 25.331); }".into())
        );
    }

    #[test]
    fn border_color() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "border-red-500"),
            Some(".border-red-500 { border-color: oklch(63.7% 0.237 25.331); }".into())
        );
    }

    #[test]
    fn outline_color() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "outline-red-500"),
            Some(".outline-red-500 { outline-color: oklch(63.7% 0.237 25.331); }".into())
        );
    }

    #[test]
    fn accent_color() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "accent-red-500"),
            Some(".accent-red-500 { accent-color: oklch(63.7% 0.237 25.331); }".into())
        );
    }

    #[test]
    fn caret_color() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "caret-red-500"),
            Some(".caret-red-500 { caret-color: oklch(63.7% 0.237 25.331); }".into())
        );
    }

    #[test]
    fn fill_color() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "fill-red-500"),
            Some(".fill-red-500 { fill: oklch(63.7% 0.237 25.331); }".into())
        );
    }

    #[test]
    fn stroke_color() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "stroke-red-500"),
            Some(".stroke-red-500 { stroke: oklch(63.7% 0.237 25.331); }".into())
        );
    }

    #[test]
    fn decoration_color() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "decoration-red-500"),
            Some(
                ".decoration-red-500 { text-decoration-color: oklch(63.7% 0.237 25.331); }".into()
            )
        );
    }

    #[test]
    fn keyword_inherit() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "text-inherit"),
            Some(".text-inherit { color: inherit; }".into())
        );
    }

    #[test]
    fn keyword_current() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "bg-current"),
            Some(".bg-current { background-color: currentColor; }".into())
        );
    }

    #[test]
    fn keyword_transparent() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "border-transparent"),
            Some(".border-transparent { border-color: transparent; }".into())
        );
    }

    #[test]
    fn unknown_color_key() {
        let compiled = compile_tailwindcss4();
        assert_eq!(generate(&compiled, "text-nope-999"), None);
        assert_eq!(generate(&compiled, "bg-doesnotexist"), None);
    }

    #[test]
    fn border_directional_color() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "border-t-red-500"),
            Some(".border-t-red-500 { border-top-color: oklch(63.7% 0.237 25.331); }".into())
        );
        assert_eq!(
            generate(&compiled, "border-x-red-500"),
            Some(".border-x-red-500 { border-inline-color: oklch(63.7% 0.237 25.331); }".into())
        );
        assert_eq!(
            generate(&compiled, "border-s-red-500"),
            Some(
                ".border-s-red-500 { border-inline-start-color: oklch(63.7% 0.237 25.331); }"
                    .into()
            )
        );
    }

    #[test]
    fn border_directional_keyword() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "border-l-inherit"),
            Some(".border-l-inherit { border-left-color: inherit; }".into())
        );
    }

    #[test]
    fn fill_none() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "fill-none"),
            Some(".fill-none { fill: none; }".into())
        );
    }

    #[test]
    fn stroke_none() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "stroke-none"),
            Some(".stroke-none { stroke: none; }".into())
        );
    }

    #[test]
    fn accent_auto() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "accent-auto"),
            Some(".accent-auto { accent-color: auto; }".into())
        );
    }

    #[test]
    fn arbitrary_bracket_color() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "text-[#ff0000]"),
            Some(".text-\\[\\#ff0000\\] { color: #ff0000; }".into())
        );
    }
}
