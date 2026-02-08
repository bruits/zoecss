use zoecss_config::{Preset, Rule};
use zoecss_core::{CssEntries, CssEntry};

/// Registers drop-shadow filter utility rules consuming `{theme.drop-shadow.$1}`.
pub fn register(preset: &mut Preset) {
    preset.rules.push(Rule::Static {
        token: "drop-shadow".into(),
        entries: CssEntries::new(vec![CssEntry::new(
            "filter",
            "drop-shadow(0 1px 2px rgb(0 0 0 / 0.1)) drop-shadow(0 1px 1px rgb(0 0 0 / 0.06))",
        )]),
    });

    preset.rules.push(Rule::Static {
        token: "drop-shadow-none".into(),
        entries: CssEntries::new(vec![CssEntry::new("filter", "drop-shadow(0 0 #0000)")]),
    });

    preset.rules.push(Rule::Pattern {
        pattern: r"^drop-shadow-(.+)$".into(),
        template: CssEntries::new(vec![CssEntry::new(
            "filter",
            "drop-shadow({theme.drop-shadow.$1})",
        )]),
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
    fn drop_shadow_none() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "drop-shadow-none"),
            Some(".drop-shadow-none { filter: drop-shadow(0 0 #0000); }".into())
        );
    }

    #[test]
    fn drop_shadow_md() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "drop-shadow-md"),
            Some(".drop-shadow-md { filter: drop-shadow(0 3px 3px rgb(0 0 0 / 0.12)); }".into())
        );
    }

    #[test]
    fn drop_shadow_bare() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "drop-shadow"),
            Some(".drop-shadow { filter: drop-shadow(0 1px 2px rgb(0 0 0 / 0.1)) drop-shadow(0 1px 1px rgb(0 0 0 / 0.06)); }".into())
        );
    }

    #[test]
    fn unknown_drop_shadow_key() {
        let compiled = compile_tailwindcss4();
        assert_eq!(generate(&compiled, "drop-shadow-99"), None);
    }
}
