use zoecss_config::{Preset, Rule};
use zoecss_core::{CssEntries, CssEntry};

/// Registers transition-delay utility rules.
pub fn register(preset: &mut Preset) {
    preset.rules.push(Rule::Pattern {
        pattern: r"^delay-(\d+)$".into(),
        template: CssEntries::new(vec![CssEntry::new("transition-delay", "$1ms")]),
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
    fn delay_150() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "delay-150"),
            Some(".delay-150 { transition-delay: 150ms; }".into())
        );
    }

    #[test]
    fn delay_0() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "delay-0"),
            Some(".delay-0 { transition-delay: 0ms; }".into())
        );
    }

    #[test]
    fn delay_non_numeric() {
        let compiled = compile_tailwindcss4();
        assert_eq!(generate(&compiled, "delay-abc"), None);
    }
}
