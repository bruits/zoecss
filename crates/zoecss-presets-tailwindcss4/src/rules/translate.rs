use zoecss_config::{Preset, Rule};
use zoecss_core::{CssEntries, CssEntry};

/// Registers translate utility rules consuming `{theme.spacing.$1}`.
///
/// Uses CSS custom properties so axis-specific utilities compose correctly.
pub fn register(preset: &mut Preset) {
    // translate-x
    preset.rules.push(Rule::Pattern {
        pattern: r"^translate-x-(.+)$".into(),
        template: CssEntries::new(vec![
            CssEntry::new("--tw-translate-x", "{theme.spacing.$1}"),
            CssEntry::new("translate", "var(--tw-translate-x) var(--tw-translate-y)"),
        ]),
    });

    // translate-y
    preset.rules.push(Rule::Pattern {
        pattern: r"^translate-y-(.+)$".into(),
        template: CssEntries::new(vec![
            CssEntry::new("--tw-translate-y", "{theme.spacing.$1}"),
            CssEntry::new("translate", "var(--tw-translate-x) var(--tw-translate-y)"),
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
    fn translate_x() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "translate-x-4"),
            Some(".translate-x-4 { --tw-translate-x: 1rem; translate: var(--tw-translate-x) var(--tw-translate-y); }".into())
        );
    }

    #[test]
    fn translate_y() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "translate-y-2"),
            Some(".translate-y-2 { --tw-translate-y: 0.5rem; translate: var(--tw-translate-x) var(--tw-translate-y); }".into())
        );
    }

    #[test]
    fn unknown_spacing_key() {
        let compiled = compile_tailwindcss4();
        assert_eq!(generate(&compiled, "translate-x-99"), None);
        assert_eq!(generate(&compiled, "translate-y-99"), None);
    }
}
