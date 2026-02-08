use zoecss_config::{Preset, Rule};
use zoecss_core::{CssEntries, CssEntry};

/// Registers text-indent utility rules consuming `{theme.spacing.$1}`.
pub fn register(preset: &mut Preset) {
    preset.rules.push(Rule::Pattern {
        pattern: r"^indent-(.+)$".into(),
        template: CssEntries::new(vec![CssEntry::new("text-indent", "{theme.spacing.$1}")]),
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
    fn text_indent() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "indent-4"),
            Some(".indent-4 { text-indent: 1rem; }".into())
        );
    }

    #[test]
    fn text_indent_zero() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "indent-0"),
            Some(".indent-0 { text-indent: 0px; }".into())
        );
    }

    #[test]
    fn unknown_spacing_key() {
        let compiled = compile_tailwindcss4();
        assert_eq!(generate(&compiled, "indent-99"), None);
    }
}
