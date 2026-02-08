use zoecss_config::{Preset, Rule};
use zoecss_core::{CssEntries, CssEntry};

/// Registers hue-rotate filter utility rules.
pub fn register(preset: &mut Preset) {
    preset.rules.push(Rule::Pattern {
        pattern: r"^hue-rotate-(\d+)$".into(),
        template: CssEntries::new(vec![CssEntry::new("filter", "hue-rotate($1deg)")]),
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
    fn hue_rotate_90() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "hue-rotate-90"),
            Some(".hue-rotate-90 { filter: hue-rotate(90deg); }".into())
        );
    }

    #[test]
    fn hue_rotate_non_numeric() {
        let compiled = compile_tailwindcss4();
        assert_eq!(generate(&compiled, "hue-rotate-abc"), None);
    }
}
