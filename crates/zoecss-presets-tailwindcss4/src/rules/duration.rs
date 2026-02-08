use zoecss_config::{Preset, Rule};
use zoecss_core::{CssEntries, CssEntry};

/// Registers transition-duration utility rules.
pub fn register(preset: &mut Preset) {
    preset.rules.push(Rule::Pattern {
        pattern: r"^duration-(\d+)$".into(),
        template: CssEntries::new(vec![CssEntry::new("transition-duration", "$1ms")]),
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
    fn duration_300() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "duration-300"),
            Some(".duration-300 { transition-duration: 300ms; }".into())
        );
    }

    #[test]
    fn duration_0() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "duration-0"),
            Some(".duration-0 { transition-duration: 0ms; }".into())
        );
    }

    #[test]
    fn duration_non_numeric() {
        let compiled = compile_tailwindcss4();
        assert_eq!(generate(&compiled, "duration-abc"), None);
    }
}
