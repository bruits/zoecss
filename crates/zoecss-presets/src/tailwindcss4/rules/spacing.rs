use zoecss_config::{Preset, Rule};
use zoecss_core::{CssEntries, CssEntry};

/// Registers all spacing utility rules (padding, margin, gap) consuming `{theme.spacing.$1}`.
///
/// Directional rules are registered before shorthand rules so the engine's
/// first-match-wins strategy hits the specific pattern before the general one.
pub fn register(preset: &mut Preset) {
    // Padding — directional first
    for (pattern, properties) in [
        (r"^px-(.+)$", vec!["padding-left", "padding-right"]),
        (r"^py-(.+)$", vec!["padding-top", "padding-bottom"]),
        (r"^pt-(.+)$", vec!["padding-top"]),
        (r"^pr-(.+)$", vec!["padding-right"]),
        (r"^pb-(.+)$", vec!["padding-bottom"]),
        (r"^pl-(.+)$", vec!["padding-left"]),
        (r"^p-(.+)$", vec!["padding"]),
    ] {
        preset.rules.push(Rule::Pattern {
            pattern: pattern.into(),
            template: CssEntries::new(
                properties
                    .into_iter()
                    .map(|prop| CssEntry::new(prop, "{theme.spacing.$1}"))
                    .collect(),
            ),
        });
    }

    // Margin — directional first
    for (pattern, properties) in [
        (r"^mx-(.+)$", vec!["margin-left", "margin-right"]),
        (r"^my-(.+)$", vec!["margin-top", "margin-bottom"]),
        (r"^mt-(.+)$", vec!["margin-top"]),
        (r"^mr-(.+)$", vec!["margin-right"]),
        (r"^mb-(.+)$", vec!["margin-bottom"]),
        (r"^ml-(.+)$", vec!["margin-left"]),
        (r"^m-(.+)$", vec!["margin"]),
    ] {
        preset.rules.push(Rule::Pattern {
            pattern: pattern.into(),
            template: CssEntries::new(
                properties
                    .into_iter()
                    .map(|prop| CssEntry::new(prop, "{theme.spacing.$1}"))
                    .collect(),
            ),
        });
    }

    // Gap — directional first
    for (pattern, property) in [
        (r"^gap-x-(.+)$", "column-gap"),
        (r"^gap-y-(.+)$", "row-gap"),
        (r"^gap-(.+)$", "gap"),
    ] {
        preset.rules.push(Rule::Pattern {
            pattern: pattern.into(),
            template: CssEntries::new(vec![CssEntry::new(property, "{theme.spacing.$1}")]),
        });
    }
}

#[cfg(test)]
mod tests {
    use zoecss_config::{CompiledConfig, Config};
    use zoecss_core::generate;

    use crate::tailwindcss4::tailwindcss4;

    fn compile_tailwindcss4() -> CompiledConfig {
        let mut config = Config::new();
        config.presets.push(tailwindcss4());
        CompiledConfig::compile(config.merge()).expect("tailwindcss4 preset compiles")
    }

    #[test]
    fn padding_shorthand() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "p-4"),
            Some(".p-4 { padding: 1rem; }".into())
        );
    }

    #[test]
    fn margin_shorthand() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "m-2"),
            Some(".m-2 { margin: 0.5rem; }".into())
        );
    }

    #[test]
    fn padding_top() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "pt-4"),
            Some(".pt-4 { padding-top: 1rem; }".into())
        );
    }

    #[test]
    fn padding_horizontal() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "px-4"),
            Some(".px-4 { padding-left: 1rem; padding-right: 1rem; }".into())
        );
    }

    #[test]
    fn padding_vertical() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "py-2"),
            Some(".py-2 { padding-top: 0.5rem; padding-bottom: 0.5rem; }".into())
        );
    }

    #[test]
    fn margin_horizontal() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "mx-8"),
            Some(".mx-8 { margin-left: 2rem; margin-right: 2rem; }".into())
        );
    }

    #[test]
    fn margin_vertical() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "my-4"),
            Some(".my-4 { margin-top: 1rem; margin-bottom: 1rem; }".into())
        );
    }

    #[test]
    fn gap_shorthand() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "gap-2"),
            Some(".gap-2 { gap: 0.5rem; }".into())
        );
    }

    #[test]
    fn gap_x() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "gap-x-4"),
            Some(".gap-x-4 { column-gap: 1rem; }".into())
        );
    }

    #[test]
    fn gap_y() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "gap-y-3"),
            Some(".gap-y-3 { row-gap: 0.75rem; }".into())
        );
    }

    #[test]
    fn unknown_spacing_key() {
        let compiled = compile_tailwindcss4();
        assert_eq!(generate(&compiled, "p-99"), None);
        assert_eq!(generate(&compiled, "pt-99"), None);
        assert_eq!(generate(&compiled, "gap-99"), None);
    }
}
