use zoecss_config::{Preset, Rule};
use zoecss_core::{CssEntries, CssEntry};

/// Registers border-spacing utility rules consuming `{theme.spacing.$1}`.
///
/// Uses CSS custom properties so axis-specific utilities compose correctly.
pub fn register(preset: &mut Preset) {
    // border-spacing-x — horizontal only
    preset.rules.push(Rule::Pattern {
        pattern: r"^border-spacing-x-(.+)$".into(),
        template: CssEntries::new(vec![
            CssEntry::new("--tw-border-spacing-x", "{theme.spacing.$1}"),
            CssEntry::new(
                "border-spacing",
                "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
            ),
        ]),
    });

    // border-spacing-y — vertical only
    preset.rules.push(Rule::Pattern {
        pattern: r"^border-spacing-y-(.+)$".into(),
        template: CssEntries::new(vec![
            CssEntry::new("--tw-border-spacing-y", "{theme.spacing.$1}"),
            CssEntry::new(
                "border-spacing",
                "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
            ),
        ]),
    });

    // border-spacing — uniform
    preset.rules.push(Rule::Pattern {
        pattern: r"^border-spacing-(.+)$".into(),
        template: CssEntries::new(vec![
            CssEntry::new("--tw-border-spacing-x", "{theme.spacing.$1}"),
            CssEntry::new("--tw-border-spacing-y", "{theme.spacing.$1}"),
            CssEntry::new(
                "border-spacing",
                "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
            ),
        ]),
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
    fn border_spacing_shorthand() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "border-spacing-4"),
            Some(".border-spacing-4 { --tw-border-spacing-x: 1rem; --tw-border-spacing-y: 1rem; border-spacing: var(--tw-border-spacing-x) var(--tw-border-spacing-y); }".into())
        );
    }

    #[test]
    fn border_spacing_x() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "border-spacing-x-2"),
            Some(".border-spacing-x-2 { --tw-border-spacing-x: 0.5rem; border-spacing: var(--tw-border-spacing-x) var(--tw-border-spacing-y); }".into())
        );
    }

    #[test]
    fn border_spacing_y() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "border-spacing-y-4"),
            Some(".border-spacing-y-4 { --tw-border-spacing-y: 1rem; border-spacing: var(--tw-border-spacing-x) var(--tw-border-spacing-y); }".into())
        );
    }

    #[test]
    fn unknown_spacing_key() {
        let compiled = compile_tailwindcss4();
        assert_eq!(generate(&compiled, "border-spacing-99"), None);
    }
}
