use zoecss_config::{Preset, Rule};
use zoecss_core::{CssEntries, CssEntry};

/// Registers contrast filter utility rules.
pub fn register(preset: &mut Preset) {
    preset.rules.push(Rule::Pattern {
        pattern: r"^contrast-(\d+)$".into(),
        template: CssEntries::new(vec![CssEntry::new("filter", "contrast($1%)")]),
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
    fn contrast_75() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "contrast-75"),
            Some(".contrast-75 { filter: contrast(75%); }".into())
        );
    }

    #[test]
    fn contrast_non_numeric() {
        let compiled = compile_tailwindcss4();
        assert_eq!(generate(&compiled, "contrast-abc"), None);
    }
}
