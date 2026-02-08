use zoecss_config::{Preset, Rule};
use zoecss_core::{CssEntries, CssEntry};

/// Registers brightness filter utility rules.
pub fn register(preset: &mut Preset) {
    preset.rules.push(Rule::Pattern {
        pattern: r"^brightness-(\d+)$".into(),
        template: CssEntries::new(vec![CssEntry::new("filter", "brightness($1%)")]),
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
    fn brightness_50() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "brightness-50"),
            Some(".brightness-50 { filter: brightness(50%); }".into())
        );
    }

    #[test]
    fn brightness_100() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "brightness-100"),
            Some(".brightness-100 { filter: brightness(100%); }".into())
        );
    }

    #[test]
    fn brightness_non_numeric() {
        let compiled = compile_tailwindcss4();
        assert_eq!(generate(&compiled, "brightness-xyz"), None);
    }
}
