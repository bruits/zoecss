use zoecss_config::{Preset, Rule};
use zoecss_core::{CssEntries, CssEntry};

/// Registers sizing utility rules (width, min-width, height, min-height, max-height, size)
/// consuming `{theme.spacing.$1}`.
///
/// Static keyword rules are registered before patterns so the engine's
/// first-match-wins strategy resolves keywords without a theme lookup.
pub fn register(preset: &mut Preset) {
    // Width keyword statics
    for (token, value) in [
        ("w-auto", "auto"),
        ("w-full", "100%"),
        ("w-screen", "100vw"),
        ("w-svw", "100svw"),
        ("w-lvw", "100lvw"),
        ("w-dvw", "100dvw"),
        ("w-min", "min-content"),
        ("w-max", "max-content"),
        ("w-fit", "fit-content"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("width", value)]),
        });
    }

    // Height keyword statics
    for (token, value) in [
        ("h-auto", "auto"),
        ("h-full", "100%"),
        ("h-screen", "100vh"),
        ("h-svh", "100svh"),
        ("h-lvh", "100lvh"),
        ("h-dvh", "100dvh"),
        ("h-min", "min-content"),
        ("h-max", "max-content"),
        ("h-fit", "fit-content"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("height", value)]),
        });
    }

    // Size keyword statics (both width and height)
    for (token, value) in [
        ("size-auto", "auto"),
        ("size-full", "100%"),
        ("size-min", "min-content"),
        ("size-max", "max-content"),
        ("size-fit", "fit-content"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![
                CssEntry::new("width", value),
                CssEntry::new("height", value),
            ]),
        });
    }

    // Min-width keyword statics
    for (token, value) in [
        ("min-w-full", "100%"),
        ("min-w-min", "min-content"),
        ("min-w-max", "max-content"),
        ("min-w-fit", "fit-content"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("min-width", value)]),
        });
    }

    // Min-height keyword statics
    for (token, value) in [
        ("min-h-full", "100%"),
        ("min-h-screen", "100vh"),
        ("min-h-svh", "100svh"),
        ("min-h-lvh", "100lvh"),
        ("min-h-dvh", "100dvh"),
        ("min-h-min", "min-content"),
        ("min-h-max", "max-content"),
        ("min-h-fit", "fit-content"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("min-height", value)]),
        });
    }

    // Max-height keyword statics
    for (token, value) in [
        ("max-h-none", "none"),
        ("max-h-full", "100%"),
        ("max-h-screen", "100vh"),
        ("max-h-svh", "100svh"),
        ("max-h-lvh", "100lvh"),
        ("max-h-dvh", "100dvh"),
        ("max-h-min", "min-content"),
        ("max-h-max", "max-content"),
        ("max-h-fit", "fit-content"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("max-height", value)]),
        });
    }

    // Width
    preset.rules.push(Rule::Pattern {
        pattern: r"^w-(.+)$".into(),
        template: CssEntries::new(vec![CssEntry::new("width", "{theme.spacing.$1}")]),
    });

    // Height
    preset.rules.push(Rule::Pattern {
        pattern: r"^h-(.+)$".into(),
        template: CssEntries::new(vec![CssEntry::new("height", "{theme.spacing.$1}")]),
    });

    // Size (sets both width and height)
    preset.rules.push(Rule::Pattern {
        pattern: r"^size-(.+)$".into(),
        template: CssEntries::new(vec![
            CssEntry::new("width", "{theme.spacing.$1}"),
            CssEntry::new("height", "{theme.spacing.$1}"),
        ]),
    });

    // Min-width
    preset.rules.push(Rule::Pattern {
        pattern: r"^min-w-(.+)$".into(),
        template: CssEntries::new(vec![CssEntry::new("min-width", "{theme.spacing.$1}")]),
    });

    // Min-height
    preset.rules.push(Rule::Pattern {
        pattern: r"^min-h-(.+)$".into(),
        template: CssEntries::new(vec![CssEntry::new("min-height", "{theme.spacing.$1}")]),
    });

    // Max-height
    preset.rules.push(Rule::Pattern {
        pattern: r"^max-h-(.+)$".into(),
        template: CssEntries::new(vec![CssEntry::new("max-height", "{theme.spacing.$1}")]),
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
    fn width() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "w-4"),
            Some(".w-4 { width: 1rem; }".into())
        );
    }

    #[test]
    fn height() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "h-8"),
            Some(".h-8 { height: 2rem; }".into())
        );
    }

    #[test]
    fn size() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "size-4"),
            Some(".size-4 { width: 1rem; height: 1rem; }".into())
        );
    }

    #[test]
    fn width_auto() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "w-auto"),
            Some(".w-auto { width: auto; }".into())
        );
    }

    #[test]
    fn width_full() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "w-full"),
            Some(".w-full { width: 100%; }".into())
        );
    }

    #[test]
    fn width_screen() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "w-screen"),
            Some(".w-screen { width: 100vw; }".into())
        );
    }

    #[test]
    fn width_svw() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "w-svw"),
            Some(".w-svw { width: 100svw; }".into())
        );
    }

    #[test]
    fn width_min() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "w-min"),
            Some(".w-min { width: min-content; }".into())
        );
    }

    #[test]
    fn height_auto() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "h-auto"),
            Some(".h-auto { height: auto; }".into())
        );
    }

    #[test]
    fn height_full() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "h-full"),
            Some(".h-full { height: 100%; }".into())
        );
    }

    #[test]
    fn height_screen() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "h-screen"),
            Some(".h-screen { height: 100vh; }".into())
        );
    }

    #[test]
    fn height_dvh() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "h-dvh"),
            Some(".h-dvh { height: 100dvh; }".into())
        );
    }

    #[test]
    fn height_fit() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "h-fit"),
            Some(".h-fit { height: fit-content; }".into())
        );
    }

    #[test]
    fn size_auto() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "size-auto"),
            Some(".size-auto { width: auto; height: auto; }".into())
        );
    }

    #[test]
    fn size_full() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "size-full"),
            Some(".size-full { width: 100%; height: 100%; }".into())
        );
    }

    #[test]
    fn unknown_spacing_key() {
        let compiled = compile_tailwindcss4();
        assert_eq!(generate(&compiled, "w-99"), None);
        assert_eq!(generate(&compiled, "h-99"), None);
        assert_eq!(generate(&compiled, "size-99"), None);
    }

    #[test]
    fn min_width_spacing() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "min-w-4"),
            Some(".min-w-4 { min-width: 1rem; }".into())
        );
    }

    #[test]
    fn min_width_full() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "min-w-full"),
            Some(".min-w-full { min-width: 100%; }".into())
        );
    }

    #[test]
    fn min_width_min() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "min-w-min"),
            Some(".min-w-min { min-width: min-content; }".into())
        );
    }

    #[test]
    fn min_width_unknown() {
        let compiled = compile_tailwindcss4();
        assert_eq!(generate(&compiled, "min-w-unknown"), None);
    }

    #[test]
    fn min_height_spacing() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "min-h-8"),
            Some(".min-h-8 { min-height: 2rem; }".into())
        );
    }

    #[test]
    fn min_height_screen() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "min-h-screen"),
            Some(".min-h-screen { min-height: 100vh; }".into())
        );
    }

    #[test]
    fn min_height_fit() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "min-h-fit"),
            Some(".min-h-fit { min-height: fit-content; }".into())
        );
    }

    #[test]
    fn min_height_unknown() {
        let compiled = compile_tailwindcss4();
        assert_eq!(generate(&compiled, "min-h-unknown"), None);
    }

    #[test]
    fn max_height_spacing() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "max-h-16"),
            Some(".max-h-16 { max-height: 4rem; }".into())
        );
    }

    #[test]
    fn max_height_none() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "max-h-none"),
            Some(".max-h-none { max-height: none; }".into())
        );
    }

    #[test]
    fn max_height_screen() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "max-h-screen"),
            Some(".max-h-screen { max-height: 100vh; }".into())
        );
    }

    #[test]
    fn max_height_dvh() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "max-h-dvh"),
            Some(".max-h-dvh { max-height: 100dvh; }".into())
        );
    }

    #[test]
    fn max_height_unknown() {
        let compiled = compile_tailwindcss4();
        assert_eq!(generate(&compiled, "max-h-unknown"), None);
    }
}
