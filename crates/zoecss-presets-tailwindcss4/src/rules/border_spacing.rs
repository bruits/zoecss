use zoecss_config::{Preset, Rule};
use zoecss_core::{CssEntries, CssEntry};

/// Registers border-spacing utility rules consuming `{theme.spacing.$1}`.
///
/// Axis rules are registered before the shorthand so the engine's
/// first-match-wins strategy hits the specific pattern first.
pub fn register(preset: &mut Preset) {
    // border-spacing-x — horizontal only
    preset.rules.push(Rule::Pattern {
        pattern: r"^border-spacing-x-(.+)$".into(),
        template: CssEntries::new(vec![CssEntry::new(
            "border-spacing",
            "{theme.spacing.$1} 0",
        )]),
    });

    // border-spacing-y — vertical only
    preset.rules.push(Rule::Pattern {
        pattern: r"^border-spacing-y-(.+)$".into(),
        template: CssEntries::new(vec![CssEntry::new(
            "border-spacing",
            "0 {theme.spacing.$1}",
        )]),
    });

    // border-spacing — uniform
    preset.rules.push(Rule::Pattern {
        pattern: r"^border-spacing-(.+)$".into(),
        template: CssEntries::new(vec![CssEntry::new("border-spacing", "{theme.spacing.$1}")]),
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
            Some(".border-spacing-4 { border-spacing: 1rem; }".into())
        );
    }

    #[test]
    fn border_spacing_x() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "border-spacing-x-2"),
            Some(".border-spacing-x-2 { border-spacing: 0.5rem 0; }".into())
        );
    }

    #[test]
    fn border_spacing_y() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "border-spacing-y-4"),
            Some(".border-spacing-y-4 { border-spacing: 0 1rem; }".into())
        );
    }

    #[test]
    fn unknown_spacing_key() {
        let compiled = compile_tailwindcss4();
        assert_eq!(generate(&compiled, "border-spacing-99"), None);
    }
}
