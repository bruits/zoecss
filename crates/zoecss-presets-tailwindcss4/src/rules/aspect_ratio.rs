use zoecss_config::{Preset, Rule};
use zoecss_core::{CssEntries, CssEntry};

/// Registers aspect ratio utility rules (`aspect-auto`, `aspect-square`,
/// `aspect-video`).
pub fn register(preset: &mut Preset) {
    // Static keyword
    preset.rules.push(Rule::Static {
        token: "aspect-auto".into(),
        entries: CssEntries::new(vec![CssEntry::new("aspect-ratio", "auto")]),
    });

    // Theme-based pattern (catches `aspect-square`, `aspect-video`, etc.)
    preset.rules.push(Rule::Pattern {
        pattern: r"^aspect-(.+)$".into(),
        template: CssEntries::new(vec![CssEntry::new("aspect-ratio", "{theme.aspect.$1}")]),
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
    fn aspect_auto() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "aspect-auto"),
            Some(".aspect-auto { aspect-ratio: auto; }".into())
        );
    }

    #[test]
    fn aspect_square() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "aspect-square"),
            Some(".aspect-square { aspect-ratio: 1 / 1; }".into())
        );
    }

    #[test]
    fn aspect_video() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "aspect-video"),
            Some(".aspect-video { aspect-ratio: 16 / 9; }".into())
        );
    }

    #[test]
    fn aspect_unknown() {
        let compiled = compile_tailwindcss4();
        assert_eq!(generate(&compiled, "aspect-unknown"), None);
    }
}
