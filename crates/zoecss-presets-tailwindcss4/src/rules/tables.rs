use zoecss_config::{Preset, Rule};
use zoecss_core::{CssEntries, CssEntry};

/// Registers table utility rules (table-layout, caption-side, border-collapse).
pub fn register(preset: &mut Preset) {
    for (token, value) in [("table-auto", "auto"), ("table-fixed", "fixed")] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("table-layout", value)]),
        });
    }

    for (token, value) in [("caption-top", "top"), ("caption-bottom", "bottom")] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("caption-side", value)]),
        });
    }

    for (token, value) in [
        ("border-collapse", "collapse"),
        ("border-separate", "separate"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("border-collapse", value)]),
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
    fn table_auto() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "table-auto"),
            Some(".table-auto { table-layout: auto; }".into())
        );
    }

    #[test]
    fn table_fixed() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "table-fixed"),
            Some(".table-fixed { table-layout: fixed; }".into())
        );
    }

    #[test]
    fn caption_top() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "caption-top"),
            Some(".caption-top { caption-side: top; }".into())
        );
    }

    #[test]
    fn caption_bottom() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "caption-bottom"),
            Some(".caption-bottom { caption-side: bottom; }".into())
        );
    }

    #[test]
    fn border_collapse() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "border-collapse"),
            Some(".border-collapse { border-collapse: collapse; }".into())
        );
    }
}
