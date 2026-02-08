use zoecss_config::{Preset, Rule};
use zoecss_core::{CssEntries, CssEntry};

/// Registers all border-radius utility rules consuming `{theme.radius.$1}`.
///
/// Directional and single-corner patterns are registered before the shorthand
/// `rounded-(.+)` so the engine's first-match-wins strategy hits the specific
/// pattern first.
pub fn register(preset: &mut Preset) {
    // Static rules — shorthand bare/none/full
    for (token, value) in [
        ("rounded", "0.25rem"),
        ("rounded-none", "0"),
        ("rounded-full", "calc(infinity * 1px)"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("border-radius", value)]),
        });
    }

    // Static rules — physical directional none/full (top, right, bottom, left)
    for (suffix, properties) in [
        (
            "t",
            vec!["border-top-left-radius", "border-top-right-radius"],
        ),
        (
            "r",
            vec!["border-top-right-radius", "border-bottom-right-radius"],
        ),
        (
            "b",
            vec!["border-bottom-right-radius", "border-bottom-left-radius"],
        ),
        (
            "l",
            vec!["border-top-left-radius", "border-bottom-left-radius"],
        ),
    ] {
        for (keyword, value) in [("none", "0"), ("full", "calc(infinity * 1px)")] {
            preset.rules.push(Rule::Static {
                token: format!("rounded-{suffix}-{keyword}").into(),
                entries: CssEntries::new(
                    properties
                        .iter()
                        .map(|p| CssEntry::new(*p, value))
                        .collect(),
                ),
            });
        }
    }

    // Static rules — physical single-corner none/full
    for (suffix, property) in [
        ("tl", "border-top-left-radius"),
        ("tr", "border-top-right-radius"),
        ("br", "border-bottom-right-radius"),
        ("bl", "border-bottom-left-radius"),
    ] {
        for (keyword, value) in [("none", "0"), ("full", "calc(infinity * 1px)")] {
            preset.rules.push(Rule::Static {
                token: format!("rounded-{suffix}-{keyword}").into(),
                entries: CssEntries::new(vec![CssEntry::new(property, value)]),
            });
        }
    }

    // Static rules — logical single-corner none/full
    for (suffix, property) in [
        ("ss", "border-start-start-radius"),
        ("se", "border-start-end-radius"),
        ("ee", "border-end-end-radius"),
        ("es", "border-end-start-radius"),
    ] {
        for (keyword, value) in [("none", "0"), ("full", "calc(infinity * 1px)")] {
            preset.rules.push(Rule::Static {
                token: format!("rounded-{suffix}-{keyword}").into(),
                entries: CssEntries::new(vec![CssEntry::new(property, value)]),
            });
        }
    }

    // Static rules — logical edge-pair none/full
    for (suffix, properties) in [
        (
            "s",
            vec!["border-start-start-radius", "border-end-start-radius"],
        ),
        (
            "e",
            vec!["border-start-end-radius", "border-end-end-radius"],
        ),
    ] {
        for (keyword, value) in [("none", "0"), ("full", "calc(infinity * 1px)")] {
            preset.rules.push(Rule::Static {
                token: format!("rounded-{suffix}-{keyword}").into(),
                entries: CssEntries::new(
                    properties
                        .iter()
                        .map(|p| CssEntry::new(*p, value))
                        .collect(),
                ),
            });
        }
    }

    // Pattern rules — physical single-corner (most specific first)
    for (pattern, property) in [
        (r"^rounded-tl-(.+)$", "border-top-left-radius"),
        (r"^rounded-tr-(.+)$", "border-top-right-radius"),
        (r"^rounded-br-(.+)$", "border-bottom-right-radius"),
        (r"^rounded-bl-(.+)$", "border-bottom-left-radius"),
    ] {
        preset.rules.push(Rule::Pattern {
            pattern: pattern.into(),
            template: CssEntries::new(vec![CssEntry::new(property, "{theme.radius.$1}")]),
        });
    }

    // Pattern rules — logical single-corner
    for (pattern, property) in [
        (r"^rounded-ss-(.+)$", "border-start-start-radius"),
        (r"^rounded-se-(.+)$", "border-start-end-radius"),
        (r"^rounded-ee-(.+)$", "border-end-end-radius"),
        (r"^rounded-es-(.+)$", "border-end-start-radius"),
    ] {
        preset.rules.push(Rule::Pattern {
            pattern: pattern.into(),
            template: CssEntries::new(vec![CssEntry::new(property, "{theme.radius.$1}")]),
        });
    }

    // Pattern rules — physical directional (two properties per side)
    for (pattern, properties) in [
        (
            r"^rounded-t-(.+)$",
            vec!["border-top-left-radius", "border-top-right-radius"],
        ),
        (
            r"^rounded-r-(.+)$",
            vec!["border-top-right-radius", "border-bottom-right-radius"],
        ),
        (
            r"^rounded-b-(.+)$",
            vec!["border-bottom-right-radius", "border-bottom-left-radius"],
        ),
        (
            r"^rounded-l-(.+)$",
            vec!["border-top-left-radius", "border-bottom-left-radius"],
        ),
    ] {
        preset.rules.push(Rule::Pattern {
            pattern: pattern.into(),
            template: CssEntries::new(
                properties
                    .into_iter()
                    .map(|prop| CssEntry::new(prop, "{theme.radius.$1}"))
                    .collect(),
            ),
        });
    }

    // Pattern rules — logical edge-pair (two properties per side)
    for (pattern, properties) in [
        (
            r"^rounded-s-(.+)$",
            vec!["border-start-start-radius", "border-end-start-radius"],
        ),
        (
            r"^rounded-e-(.+)$",
            vec!["border-start-end-radius", "border-end-end-radius"],
        ),
    ] {
        preset.rules.push(Rule::Pattern {
            pattern: pattern.into(),
            template: CssEntries::new(
                properties
                    .into_iter()
                    .map(|prop| CssEntry::new(prop, "{theme.radius.$1}"))
                    .collect(),
            ),
        });
    }

    // Pattern rule — shorthand (must be last)
    preset.rules.push(Rule::Pattern {
        pattern: r"^rounded-(.+)$".into(),
        template: CssEntries::new(vec![CssEntry::new("border-radius", "{theme.radius.$1}")]),
    });
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
    fn rounded_none() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "rounded-none"),
            Some(".rounded-none { border-radius: 0; }".into())
        );
    }

    #[test]
    fn rounded_full() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "rounded-full"),
            Some(".rounded-full { border-radius: calc(infinity * 1px); }".into())
        );
    }

    #[test]
    fn rounded_shorthand() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "rounded-md"),
            Some(".rounded-md { border-radius: 0.375rem; }".into())
        );
    }

    #[test]
    fn rounded_top() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "rounded-t-md"),
            Some(".rounded-t-md { border-top-left-radius: 0.375rem; border-top-right-radius: 0.375rem; }".into())
        );
    }

    #[test]
    fn rounded_single_corner() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "rounded-tl-lg"),
            Some(".rounded-tl-lg { border-top-left-radius: 0.5rem; }".into())
        );
    }

    #[test]
    fn rounded_logical_corner() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "rounded-ss-sm"),
            Some(".rounded-ss-sm { border-start-start-radius: 0.25rem; }".into())
        );
    }

    #[test]
    fn rounded_logical_edge() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "rounded-s-xl"),
            Some(".rounded-s-xl { border-start-start-radius: 0.75rem; border-end-start-radius: 0.75rem; }".into())
        );
    }

    #[test]
    fn rounded_t_none_static() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "rounded-t-none"),
            Some(
                ".rounded-t-none { border-top-left-radius: 0; border-top-right-radius: 0; }".into()
            )
        );
    }

    #[test]
    fn rounded_bare() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "rounded"),
            Some(".rounded { border-radius: 0.25rem; }".into())
        );
    }

    #[test]
    fn unknown_radius_key() {
        let compiled = compile_tailwindcss4();
        assert_eq!(generate(&compiled, "rounded-99"), None);
        assert_eq!(generate(&compiled, "rounded-t-99"), None);
    }
}
