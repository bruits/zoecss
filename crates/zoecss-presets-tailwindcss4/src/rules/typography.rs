use regex::Regex;
use zoecss_config::{Preset, Rule};
use zoecss_core::{CssEntries, CssEntry, Theme};

/// Registers typography utility rules.
///
/// Static rules (O(1) HashMap lookup) are registered first, so tokens like
/// `text-left` resolve immediately before any Pattern rule is attempted.
///
/// Font-family and font-weight share the `font-(.+)` prefix: font-family is
/// registered first so that keys like `font-sans` resolve to a family stack,
/// while unknown family keys (e.g. `font-bold`) fall through to the
/// font-weight rule whose theme namespace contains the match.
pub fn register(preset: &mut Preset) {
    for (token, value) in [("italic", "italic"), ("not-italic", "normal")] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("font-style", value)]),
        });
    }

    preset.rules.push(Rule::Static {
        token: "antialiased".into(),
        entries: CssEntries::new(vec![
            CssEntry::new("-webkit-font-smoothing", "antialiased"),
            CssEntry::new("-moz-osx-font-smoothing", "grayscale"),
        ]),
    });
    preset.rules.push(Rule::Static {
        token: "subpixel-antialiased".into(),
        entries: CssEntries::new(vec![
            CssEntry::new("-webkit-font-smoothing", "auto"),
            CssEntry::new("-moz-osx-font-smoothing", "auto"),
        ]),
    });

    for (token, value) in [
        ("normal-nums", "normal"),
        ("ordinal", "ordinal"),
        ("slashed-zero", "slashed-zero"),
        ("lining-nums", "lining-nums"),
        ("oldstyle-nums", "oldstyle-nums"),
        ("proportional-nums", "proportional-nums"),
        ("tabular-nums", "tabular-nums"),
        ("diagonal-fractions", "diagonal-fractions"),
        ("stacked-fractions", "stacked-fractions"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("font-variant-numeric", value)]),
        });
    }

    for (token, value) in [
        ("text-left", "left"),
        ("text-center", "center"),
        ("text-right", "right"),
        ("text-justify", "justify"),
        ("text-start", "start"),
        ("text-end", "end"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("text-align", value)]),
        });
    }

    for (token, value) in [
        ("underline", "underline"),
        ("overline", "overline"),
        ("line-through", "line-through"),
        ("no-underline", "none"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("text-decoration-line", value)]),
        });
    }

    for (token, value) in [
        ("decoration-solid", "solid"),
        ("decoration-double", "double"),
        ("decoration-dotted", "dotted"),
        ("decoration-dashed", "dashed"),
        ("decoration-wavy", "wavy"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("text-decoration-style", value)]),
        });
    }

    for (token, value) in [
        ("uppercase", "uppercase"),
        ("lowercase", "lowercase"),
        ("capitalize", "capitalize"),
        ("normal-case", "none"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("text-transform", value)]),
        });
    }

    preset.rules.push(Rule::Static {
        token: "truncate".into(),
        entries: CssEntries::new(vec![
            CssEntry::new("overflow", "hidden"),
            CssEntry::new("text-overflow", "ellipsis"),
            CssEntry::new("white-space", "nowrap"),
        ]),
    });
    for (token, value) in [("text-ellipsis", "ellipsis"), ("text-clip", "clip")] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("text-overflow", value)]),
        });
    }

    for (token, value) in [
        ("text-wrap", "wrap"),
        ("text-nowrap", "nowrap"),
        ("text-balance", "balance"),
        ("text-pretty", "pretty"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("text-wrap", value)]),
        });
    }

    for (token, value) in [
        ("whitespace-normal", "normal"),
        ("whitespace-nowrap", "nowrap"),
        ("whitespace-pre", "pre"),
        ("whitespace-pre-line", "pre-line"),
        ("whitespace-pre-wrap", "pre-wrap"),
        ("whitespace-break-spaces", "break-spaces"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("white-space", value)]),
        });
    }

    for (token, value) in [
        ("break-normal", "normal"),
        ("break-all", "break-all"),
        ("break-keep", "keep-all"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("word-break", value)]),
        });
    }

    for (token, value) in [
        ("align-baseline", "baseline"),
        ("align-top", "top"),
        ("align-middle", "middle"),
        ("align-bottom", "bottom"),
        ("align-text-top", "text-top"),
        ("align-text-bottom", "text-bottom"),
        ("align-sub", "sub"),
        ("align-super", "super"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("vertical-align", value)]),
        });
    }

    for (token, value) in [("list-inside", "inside"), ("list-outside", "outside")] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("list-style-position", value)]),
        });
    }

    for (token, value) in [
        ("list-none", "none"),
        ("list-disc", "disc"),
        ("list-decimal", "decimal"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("list-style-type", value)]),
        });
    }

    preset.rules.push(Rule::Static {
        token: "list-image-none".into(),
        entries: CssEntries::new(vec![CssEntry::new("list-style-image", "none")]),
    });

    for (token, value) in [
        ("hyphens-none", "none"),
        ("hyphens-manual", "manual"),
        ("hyphens-auto", "auto"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("hyphens", value)]),
        });
    }

    preset.rules.push(Rule::Static {
        token: "content-none".into(),
        entries: CssEntries::new(vec![CssEntry::new("content", "none")]),
    });

    preset.rules.push(Rule::Dynamic {
        pattern: r"^content-\[(.+)\]$".into(),
        handler: handle_arbitrary_content,
    });

    // Tailwind v4 uses 'wrap-*' instead of 'overflow-wrap-*'
    for (token, value) in [
        ("wrap-break-word", "break-word"),
        ("wrap-anywhere", "anywhere"),
        ("wrap-normal", "normal"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("overflow-wrap", value)]),
        });
    }

    for (token, value) in [
        ("decoration-auto", "auto"),
        ("decoration-from-font", "from-font"),
        ("decoration-0", "0px"),
        ("decoration-1", "1px"),
        ("decoration-2", "2px"),
        ("decoration-4", "4px"),
        ("decoration-8", "8px"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("text-decoration-thickness", value)]),
        });
    }

    for (token, value) in [
        ("underline-offset-auto", "auto"),
        ("underline-offset-0", "0px"),
        ("underline-offset-1", "1px"),
        ("underline-offset-2", "2px"),
        ("underline-offset-4", "4px"),
        ("underline-offset-8", "8px"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("text-underline-offset", value)]),
        });
    }

    preset.rules.push(Rule::Static {
        token: "line-clamp-none".into(),
        entries: CssEntries::new(vec![
            CssEntry::new("-webkit-line-clamp", "unset"),
            CssEntry::new("-webkit-box-orient", "horizontal"),
            CssEntry::new("display", "block"),
            CssEntry::new("overflow", "visible"),
        ]),
    });
    for (token, value) in [
        ("line-clamp-1", "1"),
        ("line-clamp-2", "2"),
        ("line-clamp-3", "3"),
        ("line-clamp-4", "4"),
        ("line-clamp-5", "5"),
        ("line-clamp-6", "6"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![
                CssEntry::new("overflow", "hidden"),
                CssEntry::new("display", "-webkit-box"),
                CssEntry::new("-webkit-box-orient", "vertical"),
                CssEntry::new("-webkit-line-clamp", value),
            ]),
        });
    }

    // Font-family — must precede font-weight (same prefix, fallthrough on miss)
    preset.rules.push(Rule::Pattern {
        pattern: r"^font-(.+)$".into(),
        template: CssEntries::new(vec![CssEntry::new("font-family", "{theme.font.$1}")]),
    });

    // Font-weight — catches `font-bold`, `font-semibold`, etc. that miss font-family
    preset.rules.push(Rule::Pattern {
        pattern: r"^font-(.+)$".into(),
        template: CssEntries::new(vec![CssEntry::new("font-weight", "{theme.font-weight.$1}")]),
    });

    // Font-size (with companion line-height)
    preset.rules.push(Rule::Pattern {
        pattern: r"^text-(.+)$".into(),
        template: CssEntries::new(vec![
            CssEntry::new("font-size", "{theme.text.$1}"),
            CssEntry::new("line-height", "{theme.text.$1--line-height}"),
        ]),
    });

    // Letter-spacing
    preset.rules.push(Rule::Pattern {
        pattern: r"^tracking-(.+)$".into(),
        template: CssEntries::new(vec![CssEntry::new("letter-spacing", "{theme.tracking.$1}")]),
    });

    // Line-height
    preset.rules.push(Rule::Pattern {
        pattern: r"^leading-(.+)$".into(),
        template: CssEntries::new(vec![CssEntry::new("line-height", "{theme.leading.$1}")]),
    });
}

fn handle_arbitrary_content(token: &str, _theme: &Theme) -> Option<CssEntries> {
    use std::sync::LazyLock;
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^content-\[(.+)\]$").expect("valid regex"));
    let caps = RE.captures(token)?;
    let value = caps.get(1)?.as_str();
    Some(CssEntries::new(vec![CssEntry::new(
        "content",
        value.to_owned(),
    )]))
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
    fn font_family_sans() {
        let compiled = compile_tailwindcss4();
        let result = generate(&compiled, "font-sans");
        assert!(result.is_some());
        let css = result.unwrap();
        assert!(css.contains("font-family:"));
        assert!(css.contains("ui-sans-serif"));
    }

    #[test]
    fn font_weight_bold() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "font-bold"),
            Some(".font-bold { font-weight: 700; }".into())
        );
    }

    #[test]
    fn text_sm_produces_font_size_and_line_height() {
        let compiled = compile_tailwindcss4();
        let result = generate(&compiled, "text-sm");
        assert_eq!(
            result,
            Some(".text-sm { font-size: 0.875rem; line-height: calc(1.25 / 0.875); }".into())
        );
    }

    #[test]
    fn tracking_tight() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "tracking-tight"),
            Some(".tracking-tight { letter-spacing: -0.025em; }".into())
        );
    }

    #[test]
    fn leading_tight() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "leading-tight"),
            Some(".leading-tight { line-height: 1.25; }".into())
        );
    }

    #[test]
    fn leading_none() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "leading-none"),
            Some(".leading-none { line-height: 1; }".into())
        );
    }

    #[test]
    fn leading_4() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "leading-4"),
            Some(".leading-4 { line-height: 1rem; }".into())
        );
    }

    #[test]
    fn unknown_font_key() {
        let compiled = compile_tailwindcss4();
        assert_eq!(generate(&compiled, "font-unknown"), None);
    }

    #[test]
    fn unknown_text_key() {
        let compiled = compile_tailwindcss4();
        assert_eq!(generate(&compiled, "text-unknown"), None);
    }

    #[test]
    fn unknown_tracking_key() {
        let compiled = compile_tailwindcss4();
        assert_eq!(generate(&compiled, "tracking-unknown"), None);
    }

    #[test]
    fn unknown_leading_key() {
        let compiled = compile_tailwindcss4();
        assert_eq!(generate(&compiled, "leading-unknown"), None);
    }

    // Verify 'text-*' resolves to color when not a font-size key

    #[test]
    fn text_color_still_works() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "text-red-500"),
            Some(".text-red-500 { color: oklch(63.7% 0.237 25.331); }".into())
        );
    }

    #[test]
    fn text_arbitrary_color_still_works() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "text-[#ff0000]"),
            Some(".text-\\[\\#ff0000\\] { color: #ff0000; }".into())
        );
    }

    // Verify 'font-*' resolves correctly for both namespaces

    #[test]
    fn font_bold_resolves_to_weight_not_family() {
        let compiled = compile_tailwindcss4();
        let result = generate(&compiled, "font-bold");
        assert!(result.is_some());
        let css = result.unwrap();
        assert!(
            css.contains("font-weight: 700"),
            "font-bold should resolve to font-weight, not font-family"
        );
    }

    #[test]
    fn font_sans_resolves_to_family() {
        let compiled = compile_tailwindcss4();
        let result = generate(&compiled, "font-sans");
        assert!(result.is_some());
        let css = result.unwrap();
        assert!(
            css.contains("font-family:"),
            "font-sans should resolve to font-family"
        );
    }

    #[test]
    fn italic() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "italic"),
            Some(".italic { font-style: italic; }".into())
        );
    }

    #[test]
    fn not_italic() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "not-italic"),
            Some(".not-italic { font-style: normal; }".into())
        );
    }

    #[test]
    fn antialiased() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "antialiased"),
            Some(".antialiased { -webkit-font-smoothing: antialiased; -moz-osx-font-smoothing: grayscale; }".into())
        );
    }

    #[test]
    fn subpixel_antialiased() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "subpixel-antialiased"),
            Some(".subpixel-antialiased { -webkit-font-smoothing: auto; -moz-osx-font-smoothing: auto; }".into())
        );
    }

    #[test]
    fn normal_nums() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "normal-nums"),
            Some(".normal-nums { font-variant-numeric: normal; }".into())
        );
    }

    #[test]
    fn tabular_nums() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "tabular-nums"),
            Some(".tabular-nums { font-variant-numeric: tabular-nums; }".into())
        );
    }

    #[test]
    fn text_left() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "text-left"),
            Some(".text-left { text-align: left; }".into())
        );
    }

    #[test]
    fn text_center() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "text-center"),
            Some(".text-center { text-align: center; }".into())
        );
    }

    #[test]
    fn underline() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "underline"),
            Some(".underline { text-decoration-line: underline; }".into())
        );
    }

    #[test]
    fn no_underline() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "no-underline"),
            Some(".no-underline { text-decoration-line: none; }".into())
        );
    }

    #[test]
    fn decoration_wavy() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "decoration-wavy"),
            Some(".decoration-wavy { text-decoration-style: wavy; }".into())
        );
    }

    #[test]
    fn uppercase() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "uppercase"),
            Some(".uppercase { text-transform: uppercase; }".into())
        );
    }

    #[test]
    fn normal_case() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "normal-case"),
            Some(".normal-case { text-transform: none; }".into())
        );
    }

    #[test]
    fn truncate() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "truncate"),
            Some(
                ".truncate { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }"
                    .into()
            )
        );
    }

    #[test]
    fn text_ellipsis() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "text-ellipsis"),
            Some(".text-ellipsis { text-overflow: ellipsis; }".into())
        );
    }

    #[test]
    fn text_wrap() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "text-wrap"),
            Some(".text-wrap { text-wrap: wrap; }".into())
        );
    }

    #[test]
    fn text_pretty() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "text-pretty"),
            Some(".text-pretty { text-wrap: pretty; }".into())
        );
    }

    #[test]
    fn whitespace_nowrap() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "whitespace-nowrap"),
            Some(".whitespace-nowrap { white-space: nowrap; }".into())
        );
    }

    #[test]
    fn break_all() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "break-all"),
            Some(".break-all { word-break: break-all; }".into())
        );
    }

    #[test]
    fn break_keep() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "break-keep"),
            Some(".break-keep { word-break: keep-all; }".into())
        );
    }

    #[test]
    fn align_middle() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "align-middle"),
            Some(".align-middle { vertical-align: middle; }".into())
        );
    }

    #[test]
    fn list_inside() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "list-inside"),
            Some(".list-inside { list-style-position: inside; }".into())
        );
    }

    #[test]
    fn list_disc() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "list-disc"),
            Some(".list-disc { list-style-type: disc; }".into())
        );
    }

    #[test]
    fn hyphens_auto() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "hyphens-auto"),
            Some(".hyphens-auto { hyphens: auto; }".into())
        );
    }

    #[test]
    fn content_none() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "content-none"),
            Some(".content-none { content: none; }".into())
        );
    }

    #[test]
    fn content_arbitrary_string() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "content-['hello']"),
            Some(".content-\\[\'hello\'\\] { content: 'hello'; }".into())
        );
    }

    #[test]
    fn content_arbitrary_attr() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "content-[attr(title)]"),
            Some(".content-\\[attr\\(title\\)\\] { content: attr(title); }".into())
        );
    }

    #[test]
    fn wrap_break_word() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "wrap-break-word"),
            Some(".wrap-break-word { overflow-wrap: break-word; }".into())
        );
    }

    #[test]
    fn wrap_normal() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "wrap-normal"),
            Some(".wrap-normal { overflow-wrap: normal; }".into())
        );
    }

    #[test]
    fn decoration_auto() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "decoration-auto"),
            Some(".decoration-auto { text-decoration-thickness: auto; }".into())
        );
    }

    #[test]
    fn decoration_from_font() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "decoration-from-font"),
            Some(".decoration-from-font { text-decoration-thickness: from-font; }".into())
        );
    }

    #[test]
    fn decoration_2_thickness() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "decoration-2"),
            Some(".decoration-2 { text-decoration-thickness: 2px; }".into())
        );
    }

    #[test]
    fn underline_offset_auto() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "underline-offset-auto"),
            Some(".underline-offset-auto { text-underline-offset: auto; }".into())
        );
    }

    #[test]
    fn underline_offset_2() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "underline-offset-2"),
            Some(".underline-offset-2 { text-underline-offset: 2px; }".into())
        );
    }

    #[test]
    fn line_clamp_3() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "line-clamp-3"),
            Some(".line-clamp-3 { overflow: hidden; display: -webkit-box; -webkit-box-orient: vertical; -webkit-line-clamp: 3; }".into())
        );
    }

    #[test]
    fn line_clamp_none() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "line-clamp-none"),
            Some(".line-clamp-none { -webkit-line-clamp: unset; -webkit-box-orient: horizontal; display: block; overflow: visible; }".into())
        );
    }
}
