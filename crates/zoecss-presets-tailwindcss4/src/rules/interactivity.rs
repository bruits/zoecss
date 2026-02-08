use zoecss_config::{Preset, Rule};
use zoecss_core::{CssEntries, CssEntry};

/// Registers interactivity utility rules (cursor, pointer-events, user-select,
/// appearance, resize, touch-action, scroll-behavior, scroll-snap, will-change,
/// forced-color-adjust, color-scheme, field-sizing).
pub fn register(preset: &mut Preset) {
    for (token, value) in [
        ("cursor-auto", "auto"),
        ("cursor-default", "default"),
        ("cursor-pointer", "pointer"),
        ("cursor-wait", "wait"),
        ("cursor-text", "text"),
        ("cursor-move", "move"),
        ("cursor-help", "help"),
        ("cursor-not-allowed", "not-allowed"),
        ("cursor-none", "none"),
        ("cursor-context-menu", "context-menu"),
        ("cursor-progress", "progress"),
        ("cursor-cell", "cell"),
        ("cursor-crosshair", "crosshair"),
        ("cursor-vertical-text", "vertical-text"),
        ("cursor-alias", "alias"),
        ("cursor-copy", "copy"),
        ("cursor-no-drop", "no-drop"),
        ("cursor-grab", "grab"),
        ("cursor-grabbing", "grabbing"),
        ("cursor-all-scroll", "all-scroll"),
        ("cursor-col-resize", "col-resize"),
        ("cursor-row-resize", "row-resize"),
        ("cursor-n-resize", "n-resize"),
        ("cursor-e-resize", "e-resize"),
        ("cursor-s-resize", "s-resize"),
        ("cursor-w-resize", "w-resize"),
        ("cursor-ne-resize", "ne-resize"),
        ("cursor-nw-resize", "nw-resize"),
        ("cursor-se-resize", "se-resize"),
        ("cursor-sw-resize", "sw-resize"),
        ("cursor-ew-resize", "ew-resize"),
        ("cursor-ns-resize", "ns-resize"),
        ("cursor-nesw-resize", "nesw-resize"),
        ("cursor-nwse-resize", "nwse-resize"),
        ("cursor-zoom-in", "zoom-in"),
        ("cursor-zoom-out", "zoom-out"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("cursor", value)]),
        });
    }

    for (token, value) in [
        ("pointer-events-none", "none"),
        ("pointer-events-auto", "auto"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("pointer-events", value)]),
        });
    }

    for (token, value) in [
        ("select-none", "none"),
        ("select-text", "text"),
        ("select-all", "all"),
        ("select-auto", "auto"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("user-select", value)]),
        });
    }

    for (token, value) in [("appearance-none", "none"), ("appearance-auto", "auto")] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("appearance", value)]),
        });
    }

    for (token, value) in [
        ("resize-none", "none"),
        ("resize", "both"),
        ("resize-x", "horizontal"),
        ("resize-y", "vertical"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("resize", value)]),
        });
    }

    for (token, value) in [
        ("touch-auto", "auto"),
        ("touch-none", "none"),
        ("touch-pan-x", "pan-x"),
        ("touch-pan-left", "pan-left"),
        ("touch-pan-right", "pan-right"),
        ("touch-pan-y", "pan-y"),
        ("touch-pan-up", "pan-up"),
        ("touch-pan-down", "pan-down"),
        ("touch-pinch-zoom", "pinch-zoom"),
        ("touch-manipulation", "manipulation"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("touch-action", value)]),
        });
    }

    for (token, value) in [("scroll-auto", "auto"), ("scroll-smooth", "smooth")] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("scroll-behavior", value)]),
        });
    }

    for (token, value) in [
        ("snap-start", "start"),
        ("snap-end", "end"),
        ("snap-center", "center"),
        ("snap-align-none", "none"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("scroll-snap-align", value)]),
        });
    }

    for (token, value) in [("snap-normal", "normal"), ("snap-always", "always")] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("scroll-snap-stop", value)]),
        });
    }

    preset.rules.push(Rule::Static {
        token: "snap-none".into(),
        entries: CssEntries::new(vec![CssEntry::new("scroll-snap-type", "none")]),
    });
    for (token, axis) in [("snap-x", "x"), ("snap-y", "y"), ("snap-both", "both")] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new(
                "scroll-snap-type",
                format!("{axis} var(--tw-scroll-snap-strictness, proximity)"),
            )]),
        });
    }
    // Strictness modifiers (set a CSS custom property consumed by snap-x/y/both)
    for (token, value) in [
        ("snap-mandatory", "mandatory"),
        ("snap-proximity", "proximity"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("--tw-scroll-snap-strictness", value)]),
        });
    }

    for (token, value) in [
        ("will-change-auto", "auto"),
        ("will-change-scroll", "scroll-position"),
        ("will-change-contents", "contents"),
        ("will-change-transform", "transform"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("will-change", value)]),
        });
    }

    for (token, value) in [
        ("forced-color-adjust-auto", "auto"),
        ("forced-color-adjust-none", "none"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("forced-color-adjust", value)]),
        });
    }

    for (token, value) in [
        ("scheme-normal", "normal"),
        ("scheme-dark", "dark"),
        ("scheme-light", "light"),
        ("scheme-light-dark", "light dark"),
        ("scheme-only-dark", "only dark"),
        ("scheme-only-light", "only light"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("color-scheme", value)]),
        });
    }

    for (token, value) in [
        ("field-sizing-fixed", "fixed"),
        ("field-sizing-content", "content"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("field-sizing", value)]),
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
    fn cursor_pointer() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "cursor-pointer"),
            Some(".cursor-pointer { cursor: pointer; }".into())
        );
    }

    #[test]
    fn cursor_not_allowed() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "cursor-not-allowed"),
            Some(".cursor-not-allowed { cursor: not-allowed; }".into())
        );
    }

    #[test]
    fn pointer_events_none() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "pointer-events-none"),
            Some(".pointer-events-none { pointer-events: none; }".into())
        );
    }

    #[test]
    fn select_none() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "select-none"),
            Some(".select-none { user-select: none; }".into())
        );
    }

    #[test]
    fn appearance_none() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "appearance-none"),
            Some(".appearance-none { appearance: none; }".into())
        );
    }

    #[test]
    fn resize_both() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "resize"),
            Some(".resize { resize: both; }".into())
        );
    }

    #[test]
    fn resize_none() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "resize-none"),
            Some(".resize-none { resize: none; }".into())
        );
    }

    #[test]
    fn touch_none() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "touch-none"),
            Some(".touch-none { touch-action: none; }".into())
        );
    }

    #[test]
    fn scroll_smooth() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "scroll-smooth"),
            Some(".scroll-smooth { scroll-behavior: smooth; }".into())
        );
    }

    #[test]
    fn snap_start() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "snap-start"),
            Some(".snap-start { scroll-snap-align: start; }".into())
        );
    }

    #[test]
    fn snap_mandatory() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "snap-mandatory"),
            Some(".snap-mandatory { --tw-scroll-snap-strictness: mandatory; }".into())
        );
    }

    #[test]
    fn snap_x() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "snap-x"),
            Some(
                ".snap-x { scroll-snap-type: x var(--tw-scroll-snap-strictness, proximity); }"
                    .into()
            )
        );
    }

    #[test]
    fn will_change_transform() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "will-change-transform"),
            Some(".will-change-transform { will-change: transform; }".into())
        );
    }

    #[test]
    fn forced_color_adjust_none() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "forced-color-adjust-none"),
            Some(".forced-color-adjust-none { forced-color-adjust: none; }".into())
        );
    }

    #[test]
    fn scheme_normal() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "scheme-normal"),
            Some(".scheme-normal { color-scheme: normal; }".into())
        );
    }

    #[test]
    fn scheme_light_dark() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "scheme-light-dark"),
            Some(".scheme-light-dark { color-scheme: light dark; }".into())
        );
    }

    #[test]
    fn scheme_only_dark() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "scheme-only-dark"),
            Some(".scheme-only-dark { color-scheme: only dark; }".into())
        );
    }

    #[test]
    fn field_sizing_content() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "field-sizing-content"),
            Some(".field-sizing-content { field-sizing: content; }".into())
        );
    }
}
