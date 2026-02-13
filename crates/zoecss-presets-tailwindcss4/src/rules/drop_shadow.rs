use std::sync::LazyLock;

use regex::Regex;
use zoecss_config::{CssEntries, CssEntry, Preset, Rule};
use zoecss_core::Theme;

/// CSS `filter` requires individual `drop-shadow()` calls, unlike `box-shadow`'s comma-separated list.
fn wrap_drop_shadow(value: &str) -> String {
    value
        .split(", ")
        .map(|part| format!("drop-shadow({part})"))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Produces the colored counterpart used by the drop-shadow composition pattern.
fn drop_shadow_to_colored(wrapped: &str) -> String {
    static RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"rgb\([^)]+\)|oklch\([^)]+\)|#[0-9a-fA-F]{3,8}\b").expect("valid regex")
    });
    RE.replace_all(wrapped, "var(--tw-drop-shadow-color)")
        .into_owned()
}

/// Builds CSS entries for a drop-shadow **size** utility (composition pattern).
fn drop_shadow_size_entries(raw_value: &str) -> CssEntries {
    let wrapped = wrap_drop_shadow(raw_value);
    let colored = drop_shadow_to_colored(&wrapped);
    CssEntries::new(vec![
        CssEntry::new("--tw-drop-shadow", wrapped),
        CssEntry::new("--tw-drop-shadow-colored", colored),
        CssEntry::new("filter", "var(--tw-drop-shadow)"),
    ])
}

/// Builds CSS entries for a drop-shadow **color** utility.
fn drop_shadow_color_entries(color: &str) -> CssEntries {
    CssEntries::new(vec![
        CssEntry::new("--tw-drop-shadow-color", color.to_owned()),
        CssEntry::new("--tw-drop-shadow", "var(--tw-drop-shadow-colored)"),
    ])
}

/// Registers drop-shadow filter utility rules (sizes + colors) using the CSS
/// custom property composition pattern.
pub fn register(preset: &mut Preset) {
    preset
        .property_defaults
        .push(CssEntry::new("--tw-drop-shadow", "drop-shadow(0 0 #0000)"));
    preset.property_defaults.push(CssEntry::new(
        "--tw-drop-shadow-colored",
        "drop-shadow(0 0 #0000)",
    ));

    preset.rules.push(Rule::Static {
        token: "drop-shadow".into(),
        entries: drop_shadow_size_entries(
            "0 1px 2px rgb(0 0 0 / 0.1), 0 1px 1px rgb(0 0 0 / 0.06)",
        ),
    });

    // `drop-shadow-none` keeps both default and colored at transparent zero
    // so a subsequent color utility cannot reveal a filter.
    preset.rules.push(Rule::Static {
        token: "drop-shadow-none".into(),
        entries: CssEntries::new(vec![
            CssEntry::new("--tw-drop-shadow", "drop-shadow(0 0 #0000)"),
            CssEntry::new("--tw-drop-shadow-colored", "drop-shadow(0 0 #0000)"),
            CssEntry::new("filter", "var(--tw-drop-shadow)"),
        ]),
    });

    preset.rules.push(Rule::Static {
        token: "drop-shadow-inherit".into(),
        entries: drop_shadow_color_entries("inherit"),
    });

    preset.rules.push(Rule::Static {
        token: "drop-shadow-current".into(),
        entries: drop_shadow_color_entries("currentColor"),
    });

    preset.rules.push(Rule::Static {
        token: "drop-shadow-transparent".into(),
        entries: drop_shadow_color_entries("transparent"),
    });

    preset.rules.push(Rule::Dynamic {
        pattern: r"^drop-shadow-(.+)$".into(),
        handler: handle_drop_shadow,
    });
}

/// Tries theme `drop-shadow` section first (size), then `color` section (color).
fn handle_drop_shadow(token: &str, theme: &Theme) -> Option<CssEntries> {
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^drop-shadow-(.+)$").expect("valid regex"));
    let caps = RE.captures(token)?;
    let key = caps.get(1)?.as_str();

    if let Some(value) = theme.get("drop-shadow", key) {
        return Some(drop_shadow_size_entries(value));
    }

    if let Some(color) = theme.get("color", key) {
        return Some(drop_shadow_color_entries(color));
    }

    None
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
    fn drop_shadow_none() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "drop-shadow-none"),
            Some(
                ".drop-shadow-none { \
                 --tw-drop-shadow: drop-shadow(0 0 #0000); \
                 --tw-drop-shadow-colored: drop-shadow(0 0 #0000); \
                 filter: var(--tw-drop-shadow); }"
                    .into()
            )
        );
    }

    #[test]
    fn drop_shadow_md() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "drop-shadow-md"),
            Some(
                ".drop-shadow-md { \
                 --tw-drop-shadow: drop-shadow(0 3px 3px rgb(0 0 0 / 0.12)); \
                 --tw-drop-shadow-colored: drop-shadow(0 3px 3px var(--tw-drop-shadow-color)); \
                 filter: var(--tw-drop-shadow); }"
                    .into()
            )
        );
    }

    #[test]
    fn drop_shadow_bare() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "drop-shadow"),
            Some(
                ".drop-shadow { \
                 --tw-drop-shadow: drop-shadow(0 1px 2px rgb(0 0 0 / 0.1)) drop-shadow(0 1px 1px rgb(0 0 0 / 0.06)); \
                 --tw-drop-shadow-colored: drop-shadow(0 1px 2px var(--tw-drop-shadow-color)) drop-shadow(0 1px 1px var(--tw-drop-shadow-color)); \
                 filter: var(--tw-drop-shadow); }"
                    .into()
            )
        );
    }

    #[test]
    fn unknown_drop_shadow_key() {
        let compiled = compile_tailwindcss4();
        assert_eq!(generate(&compiled, "drop-shadow-99"), None);
    }

    #[test]
    fn drop_shadow_red_500() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "drop-shadow-red-500"),
            Some(
                ".drop-shadow-red-500 { \
                 --tw-drop-shadow-color: oklch(63.7% 0.237 25.331); \
                 --tw-drop-shadow: var(--tw-drop-shadow-colored); }"
                    .into()
            )
        );
    }

    #[test]
    fn drop_shadow_transparent() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "drop-shadow-transparent"),
            Some(
                ".drop-shadow-transparent { \
                 --tw-drop-shadow-color: transparent; \
                 --tw-drop-shadow: var(--tw-drop-shadow-colored); }"
                    .into()
            )
        );
    }

    #[test]
    fn drop_shadow_current() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "drop-shadow-current"),
            Some(
                ".drop-shadow-current { \
                 --tw-drop-shadow-color: currentColor; \
                 --tw-drop-shadow: var(--tw-drop-shadow-colored); }"
                    .into()
            )
        );
    }

    #[test]
    fn drop_shadow_inherit() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "drop-shadow-inherit"),
            Some(
                ".drop-shadow-inherit { \
                 --tw-drop-shadow-color: inherit; \
                 --tw-drop-shadow: var(--tw-drop-shadow-colored); }"
                    .into()
            )
        );
    }
}
