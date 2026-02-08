use zoecss_config::{Preset, Rule};
use zoecss_core::{CssEntries, CssEntry};

/// Registers skew utility rules (per-axis).
pub fn register(preset: &mut Preset) {
    // skew-x before skew-y
    preset.rules.push(Rule::Pattern {
        pattern: r"^skew-x-(\d+)$".into(),
        template: CssEntries::new(vec![CssEntry::new("transform", "skewX($1deg)")]),
    });

    // skew-y
    preset.rules.push(Rule::Pattern {
        pattern: r"^skew-y-(\d+)$".into(),
        template: CssEntries::new(vec![CssEntry::new("transform", "skewY($1deg)")]),
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
    fn skew_x_6() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "skew-x-6"),
            Some(".skew-x-6 { transform: skewX(6deg); }".into())
        );
    }

    #[test]
    fn skew_y_3() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "skew-y-3"),
            Some(".skew-y-3 { transform: skewY(3deg); }".into())
        );
    }

    #[test]
    fn skew_x_12() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "skew-x-12"),
            Some(".skew-x-12 { transform: skewX(12deg); }".into())
        );
    }

    #[test]
    fn skew_non_numeric() {
        let compiled = compile_tailwindcss4();
        assert_eq!(generate(&compiled, "skew-abc"), None);
    }

    #[test]
    fn skew_x_non_numeric() {
        let compiled = compile_tailwindcss4();
        assert_eq!(generate(&compiled, "skew-x-abc"), None);
    }
}
