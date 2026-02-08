use zoecss_config::{Preset, Rule};
use zoecss_core::{CssEntries, CssEntry};

/// Registers scale utility rules (shorthand + per-axis).
pub fn register(preset: &mut Preset) {
    // Static `scale-none` first
    preset.rules.push(Rule::Static {
        token: "scale-none".into(),
        entries: CssEntries::new(vec![CssEntry::new("scale", "none")]),
    });

    // scale-x before shorthand
    preset.rules.push(Rule::Pattern {
        pattern: r"^scale-x-(\d+)$".into(),
        template: CssEntries::new(vec![CssEntry::new("scale", "$1% 1")]),
    });

    // scale-y before shorthand
    preset.rules.push(Rule::Pattern {
        pattern: r"^scale-y-(\d+)$".into(),
        template: CssEntries::new(vec![CssEntry::new("scale", "1 $1%")]),
    });

    // scale shorthand
    preset.rules.push(Rule::Pattern {
        pattern: r"^scale-(\d+)$".into(),
        template: CssEntries::new(vec![CssEntry::new("scale", "$1%")]),
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
    fn scale_none() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "scale-none"),
            Some(".scale-none { scale: none; }".into())
        );
    }

    #[test]
    fn scale_75() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "scale-75"),
            Some(".scale-75 { scale: 75%; }".into())
        );
    }

    #[test]
    fn scale_x_50() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "scale-x-50"),
            Some(".scale-x-50 { scale: 50% 1; }".into())
        );
    }

    #[test]
    fn scale_y_125() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "scale-y-125"),
            Some(".scale-y-125 { scale: 1 125%; }".into())
        );
    }

    #[test]
    fn scale_non_numeric() {
        let compiled = compile_tailwindcss4();
        assert_eq!(generate(&compiled, "scale-abc"), None);
    }
}
