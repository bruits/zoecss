use zoecss_config::{Preset, Rule};
use zoecss_core::{CssEntries, CssEntry};

/// Registers saturate filter utility rules.
pub fn register(preset: &mut Preset) {
    preset.rules.push(Rule::Pattern {
        pattern: r"^saturate-(\d+)$".into(),
        template: CssEntries::new(vec![CssEntry::new("filter", "saturate($1%)")]),
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
    fn saturate_150() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "saturate-150"),
            Some(".saturate-150 { filter: saturate(150%); }".into())
        );
    }

    #[test]
    fn saturate_non_numeric() {
        let compiled = compile_tailwindcss4();
        assert_eq!(generate(&compiled, "saturate-abc"), None);
    }
}
