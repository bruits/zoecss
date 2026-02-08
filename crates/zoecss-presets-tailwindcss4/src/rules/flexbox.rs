use regex::Regex;
use zoecss_config::{Preset, Rule};
use zoecss_core::{CssEntries, CssEntry, Theme};

/// Registers all flexbox utility rules (flex-direction, flex-wrap, flex shorthand,
/// grow, shrink, justify-content, align-items, align-self, align-content).
pub fn register(preset: &mut Preset) {
    // Flex-direction utilities
    for (token, value) in [
        ("flex-row", "row"),
        ("flex-row-reverse", "row-reverse"),
        ("flex-col", "column"),
        ("flex-col-reverse", "column-reverse"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("flex-direction", value)]),
        });
    }

    // Flex-wrap utilities
    for (token, value) in [
        ("flex-wrap", "wrap"),
        ("flex-wrap-reverse", "wrap-reverse"),
        ("flex-nowrap", "nowrap"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("flex-wrap", value)]),
        });
    }

    // Flex shorthand utilities
    for (token, value) in [
        ("flex-1", "1"),
        ("flex-auto", "auto"),
        ("flex-initial", "0 auto"),
        ("flex-none", "none"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("flex", value)]),
        });
    }

    // Flex-grow utilities
    for (token, value) in [("grow", "1"), ("grow-0", "0")] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("flex-grow", value)]),
        });
    }

    // Flex-shrink utilities
    for (token, value) in [("shrink", "1"), ("shrink-0", "0")] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("flex-shrink", value)]),
        });
    }

    // Justify-content utilities
    for (token, value) in [
        ("justify-normal", "normal"),
        ("justify-start", "flex-start"),
        ("justify-end", "flex-end"),
        ("justify-center", "center"),
        ("justify-between", "space-between"),
        ("justify-around", "space-around"),
        ("justify-evenly", "space-evenly"),
        ("justify-stretch", "stretch"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("justify-content", value)]),
        });
    }

    // Align-items utilities
    for (token, value) in [
        ("items-start", "flex-start"),
        ("items-end", "flex-end"),
        ("items-center", "center"),
        ("items-baseline", "baseline"),
        ("items-stretch", "stretch"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("align-items", value)]),
        });
    }

    // Align-self utilities
    for (token, value) in [
        ("self-auto", "auto"),
        ("self-start", "flex-start"),
        ("self-end", "flex-end"),
        ("self-center", "center"),
        ("self-stretch", "stretch"),
        ("self-baseline", "baseline"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("align-self", value)]),
        });
    }

    // Align-content utilities
    for (token, value) in [
        ("content-normal", "normal"),
        ("content-center", "center"),
        ("content-start", "flex-start"),
        ("content-end", "flex-end"),
        ("content-between", "space-between"),
        ("content-around", "space-around"),
        ("content-evenly", "space-evenly"),
        ("content-baseline", "baseline"),
        ("content-stretch", "stretch"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("align-content", value)]),
        });
    }

    // Order utilities (common static values)
    for (token, value) in [
        ("order-first", "-9999"),
        ("order-last", "9999"),
        ("order-none", "0"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("order", value)]),
        });
    }

    // Order (dynamic for any arbitrary integer)
    preset.rules.push(Rule::Dynamic {
        pattern: r"^order-(\d+)$".into(),
        handler: handle_order,
    });
}

fn handle_order(token: &str, _theme: &Theme) -> Option<CssEntries> {
    use std::sync::LazyLock;
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^order-(\d+)$").expect("valid regex"));
    let caps = RE.captures(token)?;
    let value = caps.get(1)?.as_str();
    Some(CssEntries::new(vec![CssEntry::new(
        "order",
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
    fn flex_direction_row() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "flex-row"),
            Some(".flex-row { flex-direction: row; }".into())
        );
    }

    #[test]
    fn flex_direction_col() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "flex-col"),
            Some(".flex-col { flex-direction: column; }".into())
        );
    }

    #[test]
    fn flex_direction_col_reverse() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "flex-col-reverse"),
            Some(".flex-col-reverse { flex-direction: column-reverse; }".into())
        );
    }

    #[test]
    fn flex_wrap() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "flex-wrap"),
            Some(".flex-wrap { flex-wrap: wrap; }".into())
        );
    }

    #[test]
    fn flex_nowrap() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "flex-nowrap"),
            Some(".flex-nowrap { flex-wrap: nowrap; }".into())
        );
    }

    #[test]
    fn flex_1() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "flex-1"),
            Some(".flex-1 { flex: 1; }".into())
        );
    }

    #[test]
    fn flex_initial() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "flex-initial"),
            Some(".flex-initial { flex: 0 auto; }".into())
        );
    }

    #[test]
    fn flex_none() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "flex-none"),
            Some(".flex-none { flex: none; }".into())
        );
    }

    #[test]
    fn grow() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "grow"),
            Some(".grow { flex-grow: 1; }".into())
        );
    }

    #[test]
    fn grow_0() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "grow-0"),
            Some(".grow-0 { flex-grow: 0; }".into())
        );
    }

    #[test]
    fn shrink() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "shrink"),
            Some(".shrink { flex-shrink: 1; }".into())
        );
    }

    #[test]
    fn shrink_0() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "shrink-0"),
            Some(".shrink-0 { flex-shrink: 0; }".into())
        );
    }

    #[test]
    fn justify_center() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "justify-center"),
            Some(".justify-center { justify-content: center; }".into())
        );
    }

    #[test]
    fn justify_between() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "justify-between"),
            Some(".justify-between { justify-content: space-between; }".into())
        );
    }

    #[test]
    fn justify_stretch() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "justify-stretch"),
            Some(".justify-stretch { justify-content: stretch; }".into())
        );
    }

    #[test]
    fn items_center() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "items-center"),
            Some(".items-center { align-items: center; }".into())
        );
    }

    #[test]
    fn items_baseline() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "items-baseline"),
            Some(".items-baseline { align-items: baseline; }".into())
        );
    }

    #[test]
    fn self_auto() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "self-auto"),
            Some(".self-auto { align-self: auto; }".into())
        );
    }

    #[test]
    fn self_center() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "self-center"),
            Some(".self-center { align-self: center; }".into())
        );
    }

    #[test]
    fn content_center() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "content-center"),
            Some(".content-center { align-content: center; }".into())
        );
    }

    #[test]
    fn content_between() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "content-between"),
            Some(".content-between { align-content: space-between; }".into())
        );
    }

    #[test]
    fn content_stretch() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "content-stretch"),
            Some(".content-stretch { align-content: stretch; }".into())
        );
    }

    #[test]
    fn order_first() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "order-first"),
            Some(".order-first { order: -9999; }".into())
        );
    }

    #[test]
    fn order_last() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "order-last"),
            Some(".order-last { order: 9999; }".into())
        );
    }

    #[test]
    fn order_none() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "order-none"),
            Some(".order-none { order: 0; }".into())
        );
    }

    #[test]
    fn order_3() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "order-3"),
            Some(".order-3 { order: 3; }".into())
        );
    }

    #[test]
    fn order_unknown() {
        let compiled = compile_tailwindcss4();
        assert_eq!(generate(&compiled, "order-abc"), None);
    }
}
