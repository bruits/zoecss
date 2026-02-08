use regex::Regex;
use zoecss_config::{Preset, Rule};
use zoecss_core::{CssEntries, CssEntry, Theme};

/// Registers all grid utility rules (grid-auto-flow, grid-auto-columns,
/// grid-auto-rows, grid-template-columns, grid-template-rows, grid-column,
/// grid-row, place-content, place-items, place-self, justify-items,
/// justify-self) including arbitrary bracket-value variants.
pub fn register(preset: &mut Preset) {
    // Grid-auto-flow utilities
    for (token, value) in [
        ("grid-flow-row", "row"),
        ("grid-flow-col", "column"),
        ("grid-flow-dense", "dense"),
        ("grid-flow-row-dense", "row dense"),
        ("grid-flow-col-dense", "column dense"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("grid-auto-flow", value)]),
        });
    }

    // Grid-auto-columns utilities
    for (token, value) in [
        ("auto-cols-auto", "auto"),
        ("auto-cols-min", "min-content"),
        ("auto-cols-max", "max-content"),
        ("auto-cols-fr", "minmax(0, 1fr)"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("grid-auto-columns", value)]),
        });
    }

    // Grid-auto-rows utilities
    for (token, value) in [
        ("auto-rows-auto", "auto"),
        ("auto-rows-min", "min-content"),
        ("auto-rows-max", "max-content"),
        ("auto-rows-fr", "minmax(0, 1fr)"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("grid-auto-rows", value)]),
        });
    }

    // Place-content utilities
    for (token, value) in [
        ("place-content-center", "center"),
        ("place-content-start", "start"),
        ("place-content-end", "end"),
        ("place-content-between", "space-between"),
        ("place-content-around", "space-around"),
        ("place-content-evenly", "space-evenly"),
        ("place-content-baseline", "baseline"),
        ("place-content-stretch", "stretch"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("place-content", value)]),
        });
    }

    // Place-items utilities
    for (token, value) in [
        ("place-items-start", "start"),
        ("place-items-end", "end"),
        ("place-items-center", "center"),
        ("place-items-baseline", "baseline"),
        ("place-items-stretch", "stretch"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("place-items", value)]),
        });
    }

    // Place-self utilities
    for (token, value) in [
        ("place-self-auto", "auto"),
        ("place-self-start", "start"),
        ("place-self-end", "end"),
        ("place-self-center", "center"),
        ("place-self-stretch", "stretch"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("place-self", value)]),
        });
    }

    // Justify-items utilities
    for (token, value) in [
        ("justify-items-start", "start"),
        ("justify-items-end", "end"),
        ("justify-items-center", "center"),
        ("justify-items-stretch", "stretch"),
        ("justify-items-normal", "normal"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("justify-items", value)]),
        });
    }

    // Justify-self utilities
    for (token, value) in [
        ("justify-self-auto", "auto"),
        ("justify-self-start", "start"),
        ("justify-self-end", "end"),
        ("justify-self-center", "center"),
        ("justify-self-stretch", "stretch"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("justify-self", value)]),
        });
    }

    // Grid template columns (grid-cols-1…12)
    for i in 1..=12 {
        preset.rules.push(Rule::Static {
            token: format!("grid-cols-{i}").into(),
            entries: CssEntries::new(vec![CssEntry::new(
                "grid-template-columns",
                format!("repeat({i}, minmax(0, 1fr))"),
            )]),
        });
    }

    // Grid template columns keyword utilities
    for (token, value) in [("grid-cols-none", "none"), ("grid-cols-subgrid", "subgrid")] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("grid-template-columns", value)]),
        });
    }

    // Grid template rows (grid-rows-1…12)
    for i in 1..=12 {
        preset.rules.push(Rule::Static {
            token: format!("grid-rows-{i}").into(),
            entries: CssEntries::new(vec![CssEntry::new(
                "grid-template-rows",
                format!("repeat({i}, minmax(0, 1fr))"),
            )]),
        });
    }

    // Grid template rows keyword utilities
    for (token, value) in [("grid-rows-none", "none"), ("grid-rows-subgrid", "subgrid")] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("grid-template-rows", value)]),
        });
    }

    // Grid column span (col-span-1…12)
    for i in 1..=12 {
        preset.rules.push(Rule::Static {
            token: format!("col-span-{i}").into(),
            entries: CssEntries::new(vec![CssEntry::new(
                "grid-column",
                format!("span {i} / span {i}"),
            )]),
        });
    }

    // Grid column keyword utilities
    for (token, property, value) in [
        ("col-span-full", "grid-column", "1 / -1"),
        ("col-auto", "grid-column", "auto"),
        ("col-start-auto", "grid-column-start", "auto"),
        ("col-end-auto", "grid-column-end", "auto"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new(property, value)]),
        });
    }

    // Grid column start (col-start-1…13)
    for i in 1..=13 {
        preset.rules.push(Rule::Static {
            token: format!("col-start-{i}").into(),
            entries: CssEntries::new(vec![CssEntry::new("grid-column-start", format!("{i}"))]),
        });
    }

    // Grid column end (col-end-1…13)
    for i in 1..=13 {
        preset.rules.push(Rule::Static {
            token: format!("col-end-{i}").into(),
            entries: CssEntries::new(vec![CssEntry::new("grid-column-end", format!("{i}"))]),
        });
    }

    // Grid row span (row-span-1…12)
    for i in 1..=12 {
        preset.rules.push(Rule::Static {
            token: format!("row-span-{i}").into(),
            entries: CssEntries::new(vec![CssEntry::new(
                "grid-row",
                format!("span {i} / span {i}"),
            )]),
        });
    }

    // Grid row keyword utilities
    for (token, property, value) in [
        ("row-span-full", "grid-row", "1 / -1"),
        ("row-auto", "grid-row", "auto"),
        ("row-start-auto", "grid-row-start", "auto"),
        ("row-end-auto", "grid-row-end", "auto"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new(property, value)]),
        });
    }

    // Grid row start (row-start-1…13)
    for i in 1..=13 {
        preset.rules.push(Rule::Static {
            token: format!("row-start-{i}").into(),
            entries: CssEntries::new(vec![CssEntry::new("grid-row-start", format!("{i}"))]),
        });
    }

    // Grid row end (row-end-1…13)
    for i in 1..=13 {
        preset.rules.push(Rule::Static {
            token: format!("row-end-{i}").into(),
            entries: CssEntries::new(vec![CssEntry::new("grid-row-end", format!("{i}"))]),
        });
    }

    // Arbitrary bracket-value rules
    preset.rules.push(Rule::Dynamic {
        pattern: r"^grid-cols-\[(.+)\]$".into(),
        handler: handle_arbitrary_grid_cols,
    });
    preset.rules.push(Rule::Dynamic {
        pattern: r"^grid-rows-\[(.+)\]$".into(),
        handler: handle_arbitrary_grid_rows,
    });
    preset.rules.push(Rule::Dynamic {
        pattern: r"^col-\[(.+)\]$".into(),
        handler: handle_arbitrary_col,
    });
    preset.rules.push(Rule::Dynamic {
        pattern: r"^col-start-\[(.+)\]$".into(),
        handler: handle_arbitrary_col_start,
    });
    preset.rules.push(Rule::Dynamic {
        pattern: r"^col-end-\[(.+)\]$".into(),
        handler: handle_arbitrary_col_end,
    });
    preset.rules.push(Rule::Dynamic {
        pattern: r"^row-\[(.+)\]$".into(),
        handler: handle_arbitrary_row,
    });
    preset.rules.push(Rule::Dynamic {
        pattern: r"^row-start-\[(.+)\]$".into(),
        handler: handle_arbitrary_row_start,
    });
    preset.rules.push(Rule::Dynamic {
        pattern: r"^row-end-\[(.+)\]$".into(),
        handler: handle_arbitrary_row_end,
    });
}

fn handle_arbitrary_grid_cols(token: &str, _theme: &Theme) -> Option<CssEntries> {
    use std::sync::LazyLock;
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^grid-cols-\[(.+)\]$").expect("valid regex"));
    let caps = RE.captures(token)?;
    let value = caps.get(1)?.as_str();
    Some(CssEntries::new(vec![CssEntry::new(
        "grid-template-columns",
        value.to_owned(),
    )]))
}

fn handle_arbitrary_grid_rows(token: &str, _theme: &Theme) -> Option<CssEntries> {
    use std::sync::LazyLock;
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^grid-rows-\[(.+)\]$").expect("valid regex"));
    let caps = RE.captures(token)?;
    let value = caps.get(1)?.as_str();
    Some(CssEntries::new(vec![CssEntry::new(
        "grid-template-rows",
        value.to_owned(),
    )]))
}

fn handle_arbitrary_col(token: &str, _theme: &Theme) -> Option<CssEntries> {
    use std::sync::LazyLock;
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^col-\[(.+)\]$").expect("valid regex"));
    let caps = RE.captures(token)?;
    let value = caps.get(1)?.as_str();
    Some(CssEntries::new(vec![CssEntry::new(
        "grid-column",
        value.to_owned(),
    )]))
}

fn handle_arbitrary_col_start(token: &str, _theme: &Theme) -> Option<CssEntries> {
    use std::sync::LazyLock;
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^col-start-\[(.+)\]$").expect("valid regex"));
    let caps = RE.captures(token)?;
    let value = caps.get(1)?.as_str();
    Some(CssEntries::new(vec![CssEntry::new(
        "grid-column-start",
        value.to_owned(),
    )]))
}

fn handle_arbitrary_col_end(token: &str, _theme: &Theme) -> Option<CssEntries> {
    use std::sync::LazyLock;
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^col-end-\[(.+)\]$").expect("valid regex"));
    let caps = RE.captures(token)?;
    let value = caps.get(1)?.as_str();
    Some(CssEntries::new(vec![CssEntry::new(
        "grid-column-end",
        value.to_owned(),
    )]))
}

fn handle_arbitrary_row(token: &str, _theme: &Theme) -> Option<CssEntries> {
    use std::sync::LazyLock;
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^row-\[(.+)\]$").expect("valid regex"));
    let caps = RE.captures(token)?;
    let value = caps.get(1)?.as_str();
    Some(CssEntries::new(vec![CssEntry::new(
        "grid-row",
        value.to_owned(),
    )]))
}

fn handle_arbitrary_row_start(token: &str, _theme: &Theme) -> Option<CssEntries> {
    use std::sync::LazyLock;
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^row-start-\[(.+)\]$").expect("valid regex"));
    let caps = RE.captures(token)?;
    let value = caps.get(1)?.as_str();
    Some(CssEntries::new(vec![CssEntry::new(
        "grid-row-start",
        value.to_owned(),
    )]))
}

fn handle_arbitrary_row_end(token: &str, _theme: &Theme) -> Option<CssEntries> {
    use std::sync::LazyLock;
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^row-end-\[(.+)\]$").expect("valid regex"));
    let caps = RE.captures(token)?;
    let value = caps.get(1)?.as_str();
    Some(CssEntries::new(vec![CssEntry::new(
        "grid-row-end",
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
    fn grid_flow_row() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "grid-flow-row"),
            Some(".grid-flow-row { grid-auto-flow: row; }".into())
        );
    }

    #[test]
    fn grid_flow_col() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "grid-flow-col"),
            Some(".grid-flow-col { grid-auto-flow: column; }".into())
        );
    }

    #[test]
    fn grid_flow_row_dense() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "grid-flow-row-dense"),
            Some(".grid-flow-row-dense { grid-auto-flow: row dense; }".into())
        );
    }

    #[test]
    fn auto_cols_auto() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "auto-cols-auto"),
            Some(".auto-cols-auto { grid-auto-columns: auto; }".into())
        );
    }

    #[test]
    fn auto_cols_fr() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "auto-cols-fr"),
            Some(".auto-cols-fr { grid-auto-columns: minmax(0, 1fr); }".into())
        );
    }

    #[test]
    fn auto_rows_min() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "auto-rows-min"),
            Some(".auto-rows-min { grid-auto-rows: min-content; }".into())
        );
    }

    #[test]
    fn auto_rows_fr() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "auto-rows-fr"),
            Some(".auto-rows-fr { grid-auto-rows: minmax(0, 1fr); }".into())
        );
    }

    #[test]
    fn place_content_center() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "place-content-center"),
            Some(".place-content-center { place-content: center; }".into())
        );
    }

    #[test]
    fn place_content_start() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "place-content-start"),
            Some(".place-content-start { place-content: start; }".into())
        );
    }

    #[test]
    fn place_content_end() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "place-content-end"),
            Some(".place-content-end { place-content: end; }".into())
        );
    }

    #[test]
    fn place_content_between() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "place-content-between"),
            Some(".place-content-between { place-content: space-between; }".into())
        );
    }

    #[test]
    fn place_items_center() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "place-items-center"),
            Some(".place-items-center { place-items: center; }".into())
        );
    }

    #[test]
    fn place_items_stretch() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "place-items-stretch"),
            Some(".place-items-stretch { place-items: stretch; }".into())
        );
    }

    #[test]
    fn place_self_auto() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "place-self-auto"),
            Some(".place-self-auto { place-self: auto; }".into())
        );
    }

    #[test]
    fn place_self_center() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "place-self-center"),
            Some(".place-self-center { place-self: center; }".into())
        );
    }

    #[test]
    fn justify_items_start() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "justify-items-start"),
            Some(".justify-items-start { justify-items: start; }".into())
        );
    }

    #[test]
    fn justify_items_center() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "justify-items-center"),
            Some(".justify-items-center { justify-items: center; }".into())
        );
    }

    #[test]
    fn justify_items_stretch() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "justify-items-stretch"),
            Some(".justify-items-stretch { justify-items: stretch; }".into())
        );
    }

    #[test]
    fn justify_items_normal() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "justify-items-normal"),
            Some(".justify-items-normal { justify-items: normal; }".into())
        );
    }

    #[test]
    fn justify_self_auto() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "justify-self-auto"),
            Some(".justify-self-auto { justify-self: auto; }".into())
        );
    }

    #[test]
    fn justify_self_center() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "justify-self-center"),
            Some(".justify-self-center { justify-self: center; }".into())
        );
    }

    #[test]
    fn justify_self_stretch() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "justify-self-stretch"),
            Some(".justify-self-stretch { justify-self: stretch; }".into())
        );
    }

    #[test]
    fn grid_cols_1() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "grid-cols-1"),
            Some(".grid-cols-1 { grid-template-columns: repeat(1, minmax(0, 1fr)); }".into())
        );
    }

    #[test]
    fn grid_cols_6() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "grid-cols-6"),
            Some(".grid-cols-6 { grid-template-columns: repeat(6, minmax(0, 1fr)); }".into())
        );
    }

    #[test]
    fn grid_cols_12() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "grid-cols-12"),
            Some(".grid-cols-12 { grid-template-columns: repeat(12, minmax(0, 1fr)); }".into())
        );
    }

    #[test]
    fn grid_cols_none() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "grid-cols-none"),
            Some(".grid-cols-none { grid-template-columns: none; }".into())
        );
    }

    #[test]
    fn grid_cols_subgrid() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "grid-cols-subgrid"),
            Some(".grid-cols-subgrid { grid-template-columns: subgrid; }".into())
        );
    }

    #[test]
    fn grid_rows_1() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "grid-rows-1"),
            Some(".grid-rows-1 { grid-template-rows: repeat(1, minmax(0, 1fr)); }".into())
        );
    }

    #[test]
    fn grid_rows_6() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "grid-rows-6"),
            Some(".grid-rows-6 { grid-template-rows: repeat(6, minmax(0, 1fr)); }".into())
        );
    }

    #[test]
    fn grid_rows_12() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "grid-rows-12"),
            Some(".grid-rows-12 { grid-template-rows: repeat(12, minmax(0, 1fr)); }".into())
        );
    }

    #[test]
    fn grid_rows_none() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "grid-rows-none"),
            Some(".grid-rows-none { grid-template-rows: none; }".into())
        );
    }

    #[test]
    fn grid_rows_subgrid() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "grid-rows-subgrid"),
            Some(".grid-rows-subgrid { grid-template-rows: subgrid; }".into())
        );
    }

    #[test]
    fn col_span_1() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "col-span-1"),
            Some(".col-span-1 { grid-column: span 1 / span 1; }".into())
        );
    }

    #[test]
    fn col_span_6() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "col-span-6"),
            Some(".col-span-6 { grid-column: span 6 / span 6; }".into())
        );
    }

    #[test]
    fn col_span_12() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "col-span-12"),
            Some(".col-span-12 { grid-column: span 12 / span 12; }".into())
        );
    }

    #[test]
    fn col_span_full() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "col-span-full"),
            Some(".col-span-full { grid-column: 1 / -1; }".into())
        );
    }

    #[test]
    fn col_auto() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "col-auto"),
            Some(".col-auto { grid-column: auto; }".into())
        );
    }

    #[test]
    fn col_start_1() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "col-start-1"),
            Some(".col-start-1 { grid-column-start: 1; }".into())
        );
    }

    #[test]
    fn col_start_13() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "col-start-13"),
            Some(".col-start-13 { grid-column-start: 13; }".into())
        );
    }

    #[test]
    fn col_start_auto() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "col-start-auto"),
            Some(".col-start-auto { grid-column-start: auto; }".into())
        );
    }

    #[test]
    fn col_end_1() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "col-end-1"),
            Some(".col-end-1 { grid-column-end: 1; }".into())
        );
    }

    #[test]
    fn col_end_13() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "col-end-13"),
            Some(".col-end-13 { grid-column-end: 13; }".into())
        );
    }

    #[test]
    fn col_end_auto() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "col-end-auto"),
            Some(".col-end-auto { grid-column-end: auto; }".into())
        );
    }

    #[test]
    fn row_span_1() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "row-span-1"),
            Some(".row-span-1 { grid-row: span 1 / span 1; }".into())
        );
    }

    #[test]
    fn row_span_6() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "row-span-6"),
            Some(".row-span-6 { grid-row: span 6 / span 6; }".into())
        );
    }

    #[test]
    fn row_span_12() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "row-span-12"),
            Some(".row-span-12 { grid-row: span 12 / span 12; }".into())
        );
    }

    #[test]
    fn row_span_full() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "row-span-full"),
            Some(".row-span-full { grid-row: 1 / -1; }".into())
        );
    }

    #[test]
    fn row_auto() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "row-auto"),
            Some(".row-auto { grid-row: auto; }".into())
        );
    }

    #[test]
    fn row_start_1() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "row-start-1"),
            Some(".row-start-1 { grid-row-start: 1; }".into())
        );
    }

    #[test]
    fn row_start_13() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "row-start-13"),
            Some(".row-start-13 { grid-row-start: 13; }".into())
        );
    }

    #[test]
    fn row_start_auto() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "row-start-auto"),
            Some(".row-start-auto { grid-row-start: auto; }".into())
        );
    }

    #[test]
    fn row_end_1() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "row-end-1"),
            Some(".row-end-1 { grid-row-end: 1; }".into())
        );
    }

    #[test]
    fn row_end_13() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "row-end-13"),
            Some(".row-end-13 { grid-row-end: 13; }".into())
        );
    }

    #[test]
    fn row_end_auto() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "row-end-auto"),
            Some(".row-end-auto { grid-row-end: auto; }".into())
        );
    }

    #[test]
    fn grid_cols_arbitrary() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "grid-cols-[200px_1fr_2fr]"),
            Some(".grid-cols-\\[200px_1fr_2fr\\] { grid-template-columns: 200px_1fr_2fr; }".into())
        );
    }

    #[test]
    fn grid_rows_arbitrary() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "grid-rows-[auto_1fr_auto]"),
            Some(".grid-rows-\\[auto_1fr_auto\\] { grid-template-rows: auto_1fr_auto; }".into())
        );
    }

    #[test]
    fn col_arbitrary() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "col-[span_2/4]"),
            Some(".col-\\[span_2\\/4\\] { grid-column: span_2/4; }".into())
        );
    }

    #[test]
    fn col_start_arbitrary() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "col-start-[3]"),
            Some(".col-start-\\[3\\] { grid-column-start: 3; }".into())
        );
    }

    #[test]
    fn col_end_arbitrary() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "col-end-[5]"),
            Some(".col-end-\\[5\\] { grid-column-end: 5; }".into())
        );
    }

    #[test]
    fn row_arbitrary() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "row-[span_3/span_3]"),
            Some(".row-\\[span_3\\/span_3\\] { grid-row: span_3/span_3; }".into())
        );
    }

    #[test]
    fn row_start_arbitrary() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "row-start-[2]"),
            Some(".row-start-\\[2\\] { grid-row-start: 2; }".into())
        );
    }

    #[test]
    fn row_end_arbitrary() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "row-end-[4]"),
            Some(".row-end-\\[4\\] { grid-row-end: 4; }".into())
        );
    }
}
