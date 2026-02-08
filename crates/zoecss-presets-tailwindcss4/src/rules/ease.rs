use zoecss_config::{Preset, Rule};
use zoecss_core::{CssEntries, CssEntry};

/// Registers transition-timing-function utility rules consuming `{theme.ease.$1}`.
pub fn register(preset: &mut Preset) {
    for (token, value) in [("ease-linear", "linear"), ("ease-initial", "initial")] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("transition-timing-function", value)]),
        });
    }

    preset.rules.push(Rule::Pattern {
        pattern: r"^ease-(.+)$".into(),
        template: CssEntries::new(vec![CssEntry::new(
            "transition-timing-function",
            "{theme.ease.$1}",
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
    fn ease_linear() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "ease-linear"),
            Some(".ease-linear { transition-timing-function: linear; }".into())
        );
    }

    #[test]
    fn ease_initial() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "ease-initial"),
            Some(".ease-initial { transition-timing-function: initial; }".into())
        );
    }

    #[test]
    fn ease_in() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "ease-in"),
            Some(".ease-in { transition-timing-function: cubic-bezier(0.4, 0, 1, 1); }".into())
        );
    }

    #[test]
    fn unknown_ease_key() {
        let compiled = compile_tailwindcss4();
        assert_eq!(generate(&compiled, "ease-99"), None);
    }
}
