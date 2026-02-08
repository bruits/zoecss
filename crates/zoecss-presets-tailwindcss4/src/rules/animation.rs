use zoecss_config::{Preset, Rule};
use zoecss_core::{CssEntries, CssEntry};

/// Registers animation utility rules consuming `{theme.animate.$1}`.
pub fn register(preset: &mut Preset) {
    preset.rules.push(Rule::Static {
        token: "animate-none".into(),
        entries: CssEntries::new(vec![CssEntry::new("animation", "none")]),
    });

    preset.rules.push(Rule::Pattern {
        pattern: r"^animate-(.+)$".into(),
        template: CssEntries::new(vec![CssEntry::new("animation", "{theme.animate.$1}")]),
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
    fn animate_none() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "animate-none"),
            Some(".animate-none { animation: none; }".into())
        );
    }

    #[test]
    fn animate_spin() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "animate-spin"),
            Some(".animate-spin { animation: spin 1s linear infinite; }".into())
        );
    }

    #[test]
    fn unknown_animate_key() {
        let compiled = compile_tailwindcss4();
        assert_eq!(generate(&compiled, "animate-99"), None);
    }
}
