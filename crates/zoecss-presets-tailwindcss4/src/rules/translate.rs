use zoecss_config::{Preset, Rule};
use zoecss_core::{CssEntries, CssEntry};

/// Registers translate utility rules consuming `{theme.spacing.$1}`.
///
/// Uses the native CSS `translate` property (CSS Transforms Level 2).
pub fn register(preset: &mut Preset) {
    // translate-x before shorthand
    preset.rules.push(Rule::Pattern {
        pattern: r"^translate-x-(.+)$".into(),
        template: CssEntries::new(vec![CssEntry::new("translate", "{theme.spacing.$1} 0")]),
    });

    // translate-y
    preset.rules.push(Rule::Pattern {
        pattern: r"^translate-y-(.+)$".into(),
        template: CssEntries::new(vec![CssEntry::new("translate", "0 {theme.spacing.$1}")]),
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
            Some(".translate-x-4 { translate: 1rem 0; }".into())
        );
    }

    #[test]
    fn translate_y() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "translate-y-2"),
            Some(".translate-y-2 { translate: 0 0.5rem; }".into())
        );
    }

    #[test]
    fn unknown_spacing_key() {
        let compiled = compile_tailwindcss4();
        assert_eq!(generate(&compiled, "translate-x-99"), None);
        assert_eq!(generate(&compiled, "translate-y-99"), None);
    }
}
