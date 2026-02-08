use regex::Regex;
use zoecss_config::{Preset, Rule};
use zoecss_core::{CssEntries, CssEntry, Theme};

/// Registers all layout utility rules (display, position, visibility, box-sizing,
/// overflow, float, clear, isolation, object-fit/position, overscroll, sr-only).
pub fn register(preset: &mut Preset) {
    // Display utilities — all standard CSS `display` values plus Tailwind aliases
    for (token, value) in [
        ("inline", "inline"),
        ("block", "block"),
        ("inline-block", "inline-block"),
        ("flow-root", "flow-root"),
        ("flex", "flex"),
        ("inline-flex", "inline-flex"),
        ("grid", "grid"),
        ("inline-grid", "inline-grid"),
        ("contents", "contents"),
        ("table", "table"),
        ("inline-table", "inline-table"),
        ("table-caption", "table-caption"),
        ("table-cell", "table-cell"),
        ("table-column", "table-column"),
        ("table-column-group", "table-column-group"),
        ("table-footer-group", "table-footer-group"),
        ("table-header-group", "table-header-group"),
        ("table-row-group", "table-row-group"),
        ("table-row", "table-row"),
        ("list-item", "list-item"),
        ("hidden", "none"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("display", value)]),
        });
    }

    // Screen-reader-only: visually hidden but accessible to assistive technology
    preset.rules.push(Rule::Static {
        token: "sr-only".into(),
        entries: CssEntries::new(vec![
            CssEntry::new("position", "absolute"),
            CssEntry::new("width", "1px"),
            CssEntry::new("height", "1px"),
            CssEntry::new("padding", "0"),
            CssEntry::new("margin", "-1px"),
            CssEntry::new("overflow", "hidden"),
            CssEntry::new("clip-path", "inset(50%)"),
            CssEntry::new("white-space", "nowrap"),
            CssEntry::new("border-width", "0"),
        ]),
    });

    // Undo screen-reader-only: restore normal visual rendering
    preset.rules.push(Rule::Static {
        token: "not-sr-only".into(),
        entries: CssEntries::new(vec![
            CssEntry::new("position", "static"),
            CssEntry::new("width", "auto"),
            CssEntry::new("height", "auto"),
            CssEntry::new("padding", "0"),
            CssEntry::new("margin", "0"),
            CssEntry::new("overflow", "visible"),
            CssEntry::new("clip-path", "none"),
            CssEntry::new("white-space", "normal"),
        ]),
    });

    // Position utilities
    for (token, value) in [
        ("static", "static"),
        ("fixed", "fixed"),
        ("absolute", "absolute"),
        ("relative", "relative"),
        ("sticky", "sticky"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("position", value)]),
        });
    }

    // Visibility utilities
    for (token, value) in [
        ("visible", "visible"),
        ("invisible", "hidden"),
        ("collapse", "collapse"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("visibility", value)]),
        });
    }

    // Box-sizing utilities
    for (token, value) in [("box-border", "border-box"), ("box-content", "content-box")] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("box-sizing", value)]),
        });
    }

    // Overflow utilities — shorthand and per-axis
    for (token, property, value) in [
        ("overflow-auto", "overflow", "auto"),
        ("overflow-hidden", "overflow", "hidden"),
        ("overflow-clip", "overflow", "clip"),
        ("overflow-visible", "overflow", "visible"),
        ("overflow-scroll", "overflow", "scroll"),
        ("overflow-x-auto", "overflow-x", "auto"),
        ("overflow-x-hidden", "overflow-x", "hidden"),
        ("overflow-x-clip", "overflow-x", "clip"),
        ("overflow-x-visible", "overflow-x", "visible"),
        ("overflow-x-scroll", "overflow-x", "scroll"),
        ("overflow-y-auto", "overflow-y", "auto"),
        ("overflow-y-hidden", "overflow-y", "hidden"),
        ("overflow-y-clip", "overflow-y", "clip"),
        ("overflow-y-visible", "overflow-y", "visible"),
        ("overflow-y-scroll", "overflow-y", "scroll"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new(property, value)]),
        });
    }

    // Float utilities
    for (token, value) in [
        ("float-right", "right"),
        ("float-left", "left"),
        ("float-start", "inline-start"),
        ("float-end", "inline-end"),
        ("float-none", "none"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("float", value)]),
        });
    }

    // Clear utilities
    for (token, value) in [
        ("clear-left", "left"),
        ("clear-right", "right"),
        ("clear-both", "both"),
        ("clear-start", "inline-start"),
        ("clear-end", "inline-end"),
        ("clear-none", "none"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("clear", value)]),
        });
    }

    // Isolation utilities
    for (token, value) in [("isolate", "isolate"), ("isolation-auto", "auto")] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("isolation", value)]),
        });
    }

    // Object-fit utilities
    for (token, value) in [
        ("object-contain", "contain"),
        ("object-cover", "cover"),
        ("object-fill", "fill"),
        ("object-none", "none"),
        ("object-scale-down", "scale-down"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("object-fit", value)]),
        });
    }

    // Object-position utilities
    for (token, value) in [
        ("object-top", "top"),
        ("object-top-left", "left top"),
        ("object-top-right", "right top"),
        ("object-left", "left"),
        ("object-center", "center"),
        ("object-right", "right"),
        ("object-bottom", "bottom"),
        ("object-bottom-left", "left bottom"),
        ("object-bottom-right", "right bottom"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("object-position", value)]),
        });
    }

    // Overscroll-behavior utilities — shorthand and per-axis
    for (token, property, value) in [
        ("overscroll-auto", "overscroll-behavior", "auto"),
        ("overscroll-contain", "overscroll-behavior", "contain"),
        ("overscroll-none", "overscroll-behavior", "none"),
        ("overscroll-x-auto", "overscroll-behavior-x", "auto"),
        ("overscroll-x-contain", "overscroll-behavior-x", "contain"),
        ("overscroll-x-none", "overscroll-behavior-x", "none"),
        ("overscroll-y-auto", "overscroll-behavior-y", "auto"),
        ("overscroll-y-contain", "overscroll-behavior-y", "contain"),
        ("overscroll-y-none", "overscroll-behavior-y", "none"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new(property, value)]),
        });
    }

    preset.rules.push(Rule::Static {
        token: "columns-auto".into(),
        entries: CssEntries::new(vec![CssEntry::new("columns", "auto")]),
    });
    for i in 1..=12 {
        preset.rules.push(Rule::Static {
            token: format!("columns-{i}").into(),
            entries: CssEntries::new(vec![CssEntry::new("columns", format!("{i}"))]),
        });
    }
    for (token, value) in [
        ("columns-3xs", "16rem"),
        ("columns-2xs", "18rem"),
        ("columns-xs", "20rem"),
        ("columns-sm", "24rem"),
        ("columns-md", "28rem"),
        ("columns-lg", "32rem"),
        ("columns-xl", "36rem"),
        ("columns-2xl", "42rem"),
        ("columns-3xl", "48rem"),
        ("columns-4xl", "56rem"),
        ("columns-5xl", "64rem"),
        ("columns-6xl", "72rem"),
        ("columns-7xl", "80rem"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("columns", value)]),
        });
    }

    for (token, value) in [
        ("break-before-auto", "auto"),
        ("break-before-avoid", "avoid"),
        ("break-before-all", "all"),
        ("break-before-avoid-page", "avoid-page"),
        ("break-before-page", "page"),
        ("break-before-left", "left"),
        ("break-before-right", "right"),
        ("break-before-column", "column"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("break-before", value)]),
        });
    }

    for (token, value) in [
        ("break-after-auto", "auto"),
        ("break-after-avoid", "avoid"),
        ("break-after-all", "all"),
        ("break-after-avoid-page", "avoid-page"),
        ("break-after-page", "page"),
        ("break-after-left", "left"),
        ("break-after-right", "right"),
        ("break-after-column", "column"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("break-after", value)]),
        });
    }

    for (token, value) in [
        ("break-inside-auto", "auto"),
        ("break-inside-avoid", "avoid"),
        ("break-inside-avoid-page", "avoid-page"),
        ("break-inside-avoid-column", "avoid-column"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("break-inside", value)]),
        });
    }

    // Box-decoration-break utilities
    for (token, value) in [
        ("box-decoration-clone", "clone"),
        ("box-decoration-slice", "slice"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("box-decoration-break", value)]),
        });
    }

    preset.rules.push(Rule::Static {
        token: "z-auto".into(),
        entries: CssEntries::new(vec![CssEntry::new("z-index", "auto")]),
    });
    for z in [0, 10, 20, 30, 40, 50] {
        preset.rules.push(Rule::Static {
            token: format!("z-{z}").into(),
            entries: CssEntries::new(vec![CssEntry::new("z-index", format!("{z}"))]),
        });
    }

    // Z-Index (dynamic for any arbitrary integer)
    preset.rules.push(Rule::Dynamic {
        pattern: r"^z-(\d+)$".into(),
        handler: handle_z_index,
    });
}

fn handle_z_index(token: &str, _theme: &Theme) -> Option<CssEntries> {
    use std::sync::LazyLock;
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^z-(\d+)$").expect("valid regex"));
    let caps = RE.captures(token)?;
    let value = caps.get(1)?.as_str();
    Some(CssEntries::new(vec![CssEntry::new(
        "z-index",
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
    fn display_block() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "block"),
            Some(".block { display: block; }".into())
        );
    }

    #[test]
    fn display_inline_flex() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "inline-flex"),
            Some(".inline-flex { display: inline-flex; }".into())
        );
    }

    #[test]
    fn display_table_row() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "table-row"),
            Some(".table-row { display: table-row; }".into())
        );
    }

    #[test]
    fn display_hidden() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "hidden"),
            Some(".hidden { display: none; }".into())
        );
    }

    #[test]
    fn sr_only() {
        let compiled = compile_tailwindcss4();
        let result = generate(&compiled, "sr-only");
        assert!(result.is_some());
        let css = result.unwrap();
        assert!(css.contains("position: absolute"));
        assert!(css.contains("width: 1px"));
        assert!(css.contains("clip-path: inset(50%)"));
    }

    #[test]
    fn not_sr_only() {
        let compiled = compile_tailwindcss4();
        let result = generate(&compiled, "not-sr-only");
        assert!(result.is_some());
        let css = result.unwrap();
        assert!(css.contains("position: static"));
        assert!(css.contains("width: auto"));
        assert!(css.contains("clip-path: none"));
    }

    #[test]
    fn position_sticky() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "sticky"),
            Some(".sticky { position: sticky; }".into())
        );
    }

    #[test]
    fn position_absolute() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "absolute"),
            Some(".absolute { position: absolute; }".into())
        );
    }

    #[test]
    fn visibility_invisible() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "invisible"),
            Some(".invisible { visibility: hidden; }".into())
        );
    }

    #[test]
    fn visibility_collapse() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "collapse"),
            Some(".collapse { visibility: collapse; }".into())
        );
    }

    #[test]
    fn box_border() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "box-border"),
            Some(".box-border { box-sizing: border-box; }".into())
        );
    }

    #[test]
    fn overflow_hidden() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "overflow-hidden"),
            Some(".overflow-hidden { overflow: hidden; }".into())
        );
    }

    #[test]
    fn overflow_x_auto() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "overflow-x-auto"),
            Some(".overflow-x-auto { overflow-x: auto; }".into())
        );
    }

    #[test]
    fn float_left() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "float-left"),
            Some(".float-left { float: left; }".into())
        );
    }

    #[test]
    fn float_start() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "float-start"),
            Some(".float-start { float: inline-start; }".into())
        );
    }

    #[test]
    fn clear_both() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "clear-both"),
            Some(".clear-both { clear: both; }".into())
        );
    }

    #[test]
    fn isolate() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "isolate"),
            Some(".isolate { isolation: isolate; }".into())
        );
    }

    #[test]
    fn isolation_auto() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "isolation-auto"),
            Some(".isolation-auto { isolation: auto; }".into())
        );
    }

    #[test]
    fn object_cover() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "object-cover"),
            Some(".object-cover { object-fit: cover; }".into())
        );
    }

    #[test]
    fn object_top_left() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "object-top-left"),
            Some(".object-top-left { object-position: left top; }".into())
        );
    }

    #[test]
    fn object_center() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "object-center"),
            Some(".object-center { object-position: center; }".into())
        );
    }

    #[test]
    fn overscroll_contain() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "overscroll-contain"),
            Some(".overscroll-contain { overscroll-behavior: contain; }".into())
        );
    }

    #[test]
    fn overscroll_y_none() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "overscroll-y-none"),
            Some(".overscroll-y-none { overscroll-behavior-y: none; }".into())
        );
    }

    #[test]
    fn columns_3() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "columns-3"),
            Some(".columns-3 { columns: 3; }".into())
        );
    }

    #[test]
    fn columns_lg() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "columns-lg"),
            Some(".columns-lg { columns: 32rem; }".into())
        );
    }

    #[test]
    fn columns_auto() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "columns-auto"),
            Some(".columns-auto { columns: auto; }".into())
        );
    }

    #[test]
    fn break_before_page() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "break-before-page"),
            Some(".break-before-page { break-before: page; }".into())
        );
    }

    #[test]
    fn break_inside_avoid() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "break-inside-avoid"),
            Some(".break-inside-avoid { break-inside: avoid; }".into())
        );
    }

    #[test]
    fn z_10() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "z-10"),
            Some(".z-10 { z-index: 10; }".into())
        );
    }

    #[test]
    fn z_auto() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "z-auto"),
            Some(".z-auto { z-index: auto; }".into())
        );
    }

    #[test]
    fn z_arbitrary() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "z-99"),
            Some(".z-99 { z-index: 99; }".into())
        );
    }

    #[test]
    fn box_decoration_clone() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "box-decoration-clone"),
            Some(".box-decoration-clone { box-decoration-break: clone; }".into())
        );
    }

    #[test]
    fn box_decoration_slice() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "box-decoration-slice"),
            Some(".box-decoration-slice { box-decoration-break: slice; }".into())
        );
    }
}
