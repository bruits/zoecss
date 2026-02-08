use zoecss_config::{Preset, Rule};
use zoecss_core::{CssEntries, CssEntry};

/// Registers flex-basis utility rules consuming `{theme.spacing.$1}`.
///
/// Static keyword rules are registered before the pattern so the engine's
/// first-match-wins strategy resolves keywords without a theme lookup.
pub fn register(preset: &mut Preset) {
    for (token, value) in [("basis-auto", "auto"), ("basis-full", "100%")] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("flex-basis", value)]),
        });
    }

    preset.rules.push(Rule::Pattern {
        pattern: r"^basis-(.+)$".into(),
        template: CssEntries::new(vec![CssEntry::new("flex-basis", "{theme.spacing.$1}")]),
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
    fn flex_basis() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "basis-4"),
            Some(".basis-4 { flex-basis: 1rem; }".into())
        );
    }

    #[test]
    fn flex_basis_zero() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "basis-0"),
            Some(".basis-0 { flex-basis: 0px; }".into())
        );
    }

    #[test]
    fn flex_basis_auto() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "basis-auto"),
            Some(".basis-auto { flex-basis: auto; }".into())
        );
    }

    #[test]
    fn flex_basis_full() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "basis-full"),
            Some(".basis-full { flex-basis: 100%; }".into())
        );
    }

    #[test]
    fn unknown_spacing_key() {
        let compiled = compile_tailwindcss4();
        assert_eq!(generate(&compiled, "basis-99"), None);
    }
}
