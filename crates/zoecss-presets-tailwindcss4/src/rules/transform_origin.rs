use zoecss_config::{Preset, Rule};
use zoecss_core::{CssEntries, CssEntry};

/// Registers all transform-origin utility rules.
pub fn register(preset: &mut Preset) {
    // Transform-origin utilities
    for (token, value) in [
        ("origin-center", "center"),
        ("origin-top", "top"),
        ("origin-top-right", "top right"),
        ("origin-right", "right"),
        ("origin-bottom-right", "bottom right"),
        ("origin-bottom", "bottom"),
        ("origin-bottom-left", "bottom left"),
        ("origin-left", "left"),
        ("origin-top-left", "top left"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("transform-origin", value)]),
        });
    }
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
    fn origin_center() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "origin-center"),
            Some(".origin-center { transform-origin: center; }".into())
        );
    }

    #[test]
    fn origin_top() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "origin-top"),
            Some(".origin-top { transform-origin: top; }".into())
        );
    }

    #[test]
    fn origin_top_right() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "origin-top-right"),
            Some(".origin-top-right { transform-origin: top right; }".into())
        );
    }

    #[test]
    fn origin_bottom_left() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "origin-bottom-left"),
            Some(".origin-bottom-left { transform-origin: bottom left; }".into())
        );
    }

    #[test]
    fn origin_left() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "origin-left"),
            Some(".origin-left { transform-origin: left; }".into())
        );
    }

    #[test]
    fn origin_unknown() {
        let compiled = compile_tailwindcss4();
        assert_eq!(generate(&compiled, "origin-xyz"), None);
    }
}
