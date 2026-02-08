use zoecss_config::{Preset, Rule};
use zoecss_core::{CssEntries, CssEntry};

/// Registers transition utility rules.
pub fn register(preset: &mut Preset) {
    preset.rules.push(Rule::Static {
        token: "transition-none".into(),
        entries: CssEntries::new(vec![CssEntry::new("transition-property", "none")]),
    });

    preset.rules.push(Rule::Static {
        token: "transition-all".into(),
        entries: CssEntries::new(vec![
            CssEntry::new("transition-property", "all"),
            CssEntry::new(
                "transition-timing-function",
                "var(--default-transition-timing-function)",
            ),
            CssEntry::new("transition-duration", "var(--default-transition-duration)"),
        ]),
    });

    preset.rules.push(Rule::Static {
        token: "transition".into(),
        entries: CssEntries::new(vec![
            CssEntry::new(
                "transition-property",
                "color, background-color, border-color, outline-color, text-decoration-color, fill, stroke, --tw-gradient-from, --tw-gradient-via, --tw-gradient-to, opacity, box-shadow, transform, translate, scale, rotate, filter, -webkit-backdrop-filter, backdrop-filter",
            ),
            CssEntry::new(
                "transition-timing-function",
                "var(--default-transition-timing-function)",
            ),
            CssEntry::new(
                "transition-duration",
                "var(--default-transition-duration)",
            ),
        ]),
    });

    preset.rules.push(Rule::Static {
        token: "transition-colors".into(),
        entries: CssEntries::new(vec![
            CssEntry::new(
                "transition-property",
                "color, background-color, border-color, outline-color, text-decoration-color, fill, stroke, --tw-gradient-from, --tw-gradient-via, --tw-gradient-to",
            ),
            CssEntry::new(
                "transition-timing-function",
                "var(--default-transition-timing-function)",
            ),
            CssEntry::new(
                "transition-duration",
                "var(--default-transition-duration)",
            ),
        ]),
    });

    preset.rules.push(Rule::Static {
        token: "transition-opacity".into(),
        entries: CssEntries::new(vec![
            CssEntry::new("transition-property", "opacity"),
            CssEntry::new(
                "transition-timing-function",
                "var(--default-transition-timing-function)",
            ),
            CssEntry::new("transition-duration", "var(--default-transition-duration)"),
        ]),
    });

    preset.rules.push(Rule::Static {
        token: "transition-shadow".into(),
        entries: CssEntries::new(vec![
            CssEntry::new("transition-property", "box-shadow"),
            CssEntry::new(
                "transition-timing-function",
                "var(--default-transition-timing-function)",
            ),
            CssEntry::new("transition-duration", "var(--default-transition-duration)"),
        ]),
    });

    preset.rules.push(Rule::Static {
        token: "transition-transform".into(),
        entries: CssEntries::new(vec![
            CssEntry::new("transition-property", "transform, translate, scale, rotate"),
            CssEntry::new(
                "transition-timing-function",
                "var(--default-transition-timing-function)",
            ),
            CssEntry::new("transition-duration", "var(--default-transition-duration)"),
        ]),
    });

    preset.rules.push(Rule::Static {
        token: "transition-discrete".into(),
        entries: CssEntries::new(vec![CssEntry::new("transition-behavior", "allow-discrete")]),
    });

    preset.rules.push(Rule::Static {
        token: "transition-normal".into(),
        entries: CssEntries::new(vec![CssEntry::new("transition-behavior", "normal")]),
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
    fn transition_none() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "transition-none"),
            Some(".transition-none { transition-property: none; }".into())
        );
    }

    #[test]
    fn transition_all() {
        let compiled = compile_tailwindcss4();
        let result = generate(&compiled, "transition-all");
        assert!(result.is_some());
        let css = result.unwrap();
        assert!(css.contains("transition-property: all"));
        assert!(
            css.contains("transition-timing-function: var(--default-transition-timing-function)")
        );
        assert!(css.contains("transition-duration: var(--default-transition-duration)"));
    }

    #[test]
    fn transition_opacity() {
        let compiled = compile_tailwindcss4();
        let result = generate(&compiled, "transition-opacity");
        assert!(result.is_some());
        let css = result.unwrap();
        assert!(css.contains("transition-property: opacity"));
        assert!(
            css.contains("transition-timing-function: var(--default-transition-timing-function)")
        );
        assert!(css.contains("transition-duration: var(--default-transition-duration)"));
    }

    #[test]
    fn transition_default() {
        let compiled = compile_tailwindcss4();
        let result = generate(&compiled, "transition");
        assert!(result.is_some());
        let css = result.unwrap();
        assert!(css.contains("transition-property:"));
        assert!(css.contains("background-color"));
        assert!(css.contains("backdrop-filter"));
    }

    #[test]
    fn transition_discrete() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "transition-discrete"),
            Some(".transition-discrete { transition-behavior: allow-discrete; }".into())
        );
    }

    #[test]
    fn transition_normal() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "transition-normal"),
            Some(".transition-normal { transition-behavior: normal; }".into())
        );
    }
}
