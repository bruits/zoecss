use zoecss_config::{Preset, Rule};
use zoecss_core::{CssEntries, CssEntry};

/// Registers blur filter utility rules consuming `{theme.blur.$1}`.
pub fn register(preset: &mut Preset) {
    preset.rules.push(Rule::Static {
        token: "blur".into(),
        entries: CssEntries::new(vec![CssEntry::new("filter", "blur(8px)")]),
    });

    preset.rules.push(Rule::Static {
        token: "blur-none".into(),
        entries: CssEntries::new(vec![CssEntry::new("filter", "blur(0px)")]),
    });

    preset.rules.push(Rule::Pattern {
        pattern: r"^blur-(.+)$".into(),
        template: CssEntries::new(vec![CssEntry::new("filter", "blur({theme.blur.$1})")]),
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
    fn blur_none() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "blur-none"),
            Some(".blur-none { filter: blur(0px); }".into())
        );
    }

    #[test]
    fn blur_md() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "blur-md"),
            Some(".blur-md { filter: blur(12px); }".into())
        );
    }

    #[test]
    fn blur_bare() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "blur"),
            Some(".blur { filter: blur(8px); }".into())
        );
    }

    #[test]
    fn unknown_blur_key() {
        let compiled = compile_tailwindcss4();
        assert_eq!(generate(&compiled, "blur-99"), None);
    }
}
