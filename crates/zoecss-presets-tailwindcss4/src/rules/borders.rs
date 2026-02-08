use zoecss_config::{Preset, Rule};
use zoecss_core::{CssEntries, CssEntry};

/// Registers border style, border width, outline style, outline width, and outline offset utility rules.
pub fn register(preset: &mut Preset) {
    for (token, value) in [
        ("border-solid", "solid"),
        ("border-dashed", "dashed"),
        ("border-dotted", "dotted"),
        ("border-double", "double"),
        ("border-hidden", "hidden"),
        ("border-none", "none"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("border-style", value)]),
        });
    }

    preset.rules.push(Rule::Static {
        token: "outline".into(),
        entries: CssEntries::new(vec![CssEntry::new("outline-width", "1px")]),
    });
    for (token, value) in [
        ("outline-solid", "solid"),
        ("outline-dashed", "dashed"),
        ("outline-dotted", "dotted"),
        ("outline-double", "double"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("outline-style", value)]),
        });
    }
    preset.rules.push(Rule::Static {
        token: "outline-none".into(),
        entries: CssEntries::new(vec![CssEntry::new("outline-style", "none")]),
    });
    preset.rules.push(Rule::Static {
        token: "outline-hidden".into(),
        entries: CssEntries::new(vec![
            CssEntry::new("outline", "2px solid transparent"),
            CssEntry::new("outline-offset", "2px"),
        ]),
    });

    for (token, value) in [
        ("border", "1px"),
        ("border-0", "0px"),
        ("border-2", "2px"),
        ("border-4", "4px"),
        ("border-8", "8px"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("border-width", value)]),
        });
    }

    // Border-x (left + right)
    for (token, value) in [
        ("border-x", "1px"),
        ("border-x-0", "0px"),
        ("border-x-2", "2px"),
        ("border-x-4", "4px"),
        ("border-x-8", "8px"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![
                CssEntry::new("border-left-width", value),
                CssEntry::new("border-right-width", value),
            ]),
        });
    }

    // Border-y (top + bottom)
    for (token, value) in [
        ("border-y", "1px"),
        ("border-y-0", "0px"),
        ("border-y-2", "2px"),
        ("border-y-4", "4px"),
        ("border-y-8", "8px"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![
                CssEntry::new("border-top-width", value),
                CssEntry::new("border-bottom-width", value),
            ]),
        });
    }

    // Per-side border width
    for (prefix, property) in [
        ("border-t", "border-top-width"),
        ("border-r", "border-right-width"),
        ("border-b", "border-bottom-width"),
        ("border-l", "border-left-width"),
        ("border-s", "border-inline-start-width"),
        ("border-e", "border-inline-end-width"),
    ] {
        for (suffix, value) in [
            ("", "1px"),
            ("-0", "0px"),
            ("-2", "2px"),
            ("-4", "4px"),
            ("-8", "8px"),
        ] {
            let token = format!("{prefix}{suffix}");
            preset.rules.push(Rule::Static {
                token: token.into(),
                entries: CssEntries::new(vec![CssEntry::new(property, value)]),
            });
        }
    }

    for (token, value) in [
        ("outline-0", "0px"),
        ("outline-1", "1px"),
        ("outline-2", "2px"),
        ("outline-4", "4px"),
        ("outline-8", "8px"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("outline-width", value)]),
        });
    }

    for (token, value) in [
        ("outline-offset-0", "0px"),
        ("outline-offset-1", "1px"),
        ("outline-offset-2", "2px"),
        ("outline-offset-4", "4px"),
        ("outline-offset-8", "8px"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("outline-offset", value)]),
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
    fn border_solid() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "border-solid"),
            Some(".border-solid { border-style: solid; }".into())
        );
    }

    #[test]
    fn border_dashed() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "border-dashed"),
            Some(".border-dashed { border-style: dashed; }".into())
        );
    }

    #[test]
    fn outline_bare() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "outline"),
            Some(".outline { outline-width: 1px; }".into())
        );
    }

    #[test]
    fn outline_solid() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "outline-solid"),
            Some(".outline-solid { outline-style: solid; }".into())
        );
    }

    #[test]
    fn outline_none() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "outline-none"),
            Some(".outline-none { outline-style: none; }".into())
        );
    }

    #[test]
    fn outline_hidden() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "outline-hidden"),
            Some(".outline-hidden { outline: 2px solid transparent; outline-offset: 2px; }".into())
        );
    }

    #[test]
    fn border_default_width() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "border"),
            Some(".border { border-width: 1px; }".into())
        );
    }

    #[test]
    fn border_0() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "border-0"),
            Some(".border-0 { border-width: 0px; }".into())
        );
    }

    #[test]
    fn border_2() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "border-2"),
            Some(".border-2 { border-width: 2px; }".into())
        );
    }

    #[test]
    fn border_x_default() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "border-x"),
            Some(".border-x { border-left-width: 1px; border-right-width: 1px; }".into())
        );
    }

    #[test]
    fn border_y_4() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "border-y-4"),
            Some(".border-y-4 { border-top-width: 4px; border-bottom-width: 4px; }".into())
        );
    }

    #[test]
    fn border_t_2() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "border-t-2"),
            Some(".border-t-2 { border-top-width: 2px; }".into())
        );
    }

    #[test]
    fn border_s_default() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "border-s"),
            Some(".border-s { border-inline-start-width: 1px; }".into())
        );
    }

    #[test]
    fn outline_width_1() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "outline-1"),
            Some(".outline-1 { outline-width: 1px; }".into())
        );
    }

    #[test]
    fn outline_width_4() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "outline-4"),
            Some(".outline-4 { outline-width: 4px; }".into())
        );
    }

    #[test]
    fn outline_offset_0() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "outline-offset-0"),
            Some(".outline-offset-0 { outline-offset: 0px; }".into())
        );
    }

    #[test]
    fn outline_offset_4() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "outline-offset-4"),
            Some(".outline-offset-4 { outline-offset: 4px; }".into())
        );
    }
}
