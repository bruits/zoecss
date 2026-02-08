use zoecss_config::{Preset, Rule};
use zoecss_core::{CssEntries, CssEntry};

/// Registers box-shadow utility rules consuming `{theme.shadow.$1}`.
pub fn register(preset: &mut Preset) {
    preset.rules.push(Rule::Static {
        token: "shadow".into(),
        entries: CssEntries::new(vec![CssEntry::new(
            "box-shadow",
            "0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)",
        )]),
    });

    preset.rules.push(Rule::Static {
        token: "shadow-none".into(),
        entries: CssEntries::new(vec![CssEntry::new("box-shadow", "0 0 #0000")]),
    });

    preset.rules.push(Rule::Static {
        token: "shadow-inner".into(),
        entries: CssEntries::new(vec![CssEntry::new(
            "box-shadow",
            "inset 0 2px 4px 0 rgb(0 0 0 / 0.05)",
        )]),
    });

    preset.rules.push(Rule::Pattern {
        pattern: r"^shadow-(.+)$".into(),
        template: CssEntries::new(vec![CssEntry::new("box-shadow", "{theme.shadow.$1}")]),
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
    fn shadow_none() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "shadow-none"),
            Some(".shadow-none { box-shadow: 0 0 #0000; }".into())
        );
    }

    #[test]
    fn shadow_md() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "shadow-md"),
            Some(".shadow-md { box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1); }".into())
        );
    }

    #[test]
    fn shadow_bare() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "shadow"),
            Some(".shadow { box-shadow: 0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1); }".into())
        );
    }

    #[test]
    fn shadow_inner() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "shadow-inner"),
            Some(".shadow-inner { box-shadow: inset 0 2px 4px 0 rgb(0 0 0 / 0.05); }".into())
        );
    }

    #[test]
    fn unknown_shadow_key() {
        let compiled = compile_tailwindcss4();
        assert_eq!(generate(&compiled, "shadow-99"), None);
    }
}
