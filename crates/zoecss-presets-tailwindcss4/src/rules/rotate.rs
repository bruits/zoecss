use zoecss_config::{Preset, Rule};
use zoecss_core::{CssEntries, CssEntry};

/// Registers rotate utility rules.
pub fn register(preset: &mut Preset) {
    // Static `rotate-none` before the numeric pattern
    preset.rules.push(Rule::Static {
        token: "rotate-none".into(),
        entries: CssEntries::new(vec![CssEntry::new("rotate", "none")]),
    });

    preset.rules.push(Rule::Pattern {
        pattern: r"^rotate-(\d+)$".into(),
        template: CssEntries::new(vec![CssEntry::new("rotate", "$1deg")]),
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
    fn rotate_none() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "rotate-none"),
            Some(".rotate-none { rotate: none; }".into())
        );
    }

    #[test]
    fn rotate_45() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "rotate-45"),
            Some(".rotate-45 { rotate: 45deg; }".into())
        );
    }

    #[test]
    fn rotate_180() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "rotate-180"),
            Some(".rotate-180 { rotate: 180deg; }".into())
        );
    }

    #[test]
    fn rotate_non_numeric() {
        let compiled = compile_tailwindcss4();
        assert_eq!(generate(&compiled, "rotate-abc"), None);
    }
}
