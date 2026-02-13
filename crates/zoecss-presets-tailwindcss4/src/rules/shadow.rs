use std::sync::LazyLock;

use regex::Regex;
use zoecss_config::{CssEntries, CssEntry, Preset, Rule};
use zoecss_core::Theme;

/// Produces the colored counterpart used by the shadow composition pattern.
fn shadow_to_colored(shadow: &str) -> String {
    static RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"rgb\([^)]+\)|oklch\([^)]+\)|#[0-9a-fA-F]{3,8}\b").expect("valid regex")
    });
    RE.replace_all(shadow, "var(--tw-shadow-color)")
        .into_owned()
}

/// Builds CSS entries for a shadow **size** utility (composition pattern).
fn shadow_size_entries(value: &str) -> CssEntries {
    CssEntries::new(vec![
        CssEntry::new("--tw-shadow", value.to_owned()),
        CssEntry::new("--tw-shadow-colored", shadow_to_colored(value)),
        CssEntry::new("box-shadow", "var(--tw-inset-shadow), var(--tw-shadow)"),
    ])
}

/// Builds CSS entries for a shadow **color** utility.
fn shadow_color_entries(color: &str) -> CssEntries {
    CssEntries::new(vec![
        CssEntry::new("--tw-shadow-color", color.to_owned()),
        CssEntry::new("--tw-shadow", "var(--tw-shadow-colored)"),
    ])
}

/// Registers box-shadow utility rules (sizes + colors) using the CSS custom
/// property composition pattern.
pub fn register(preset: &mut Preset) {
    preset
        .property_defaults
        .push(CssEntry::new("--tw-shadow", "0 0 #0000"));
    preset
        .property_defaults
        .push(CssEntry::new("--tw-shadow-colored", "0 0 #0000"));
    preset
        .property_defaults
        .push(CssEntry::new("--tw-inset-shadow", "0 0 #0000"));
    preset
        .property_defaults
        .push(CssEntry::new("--tw-inset-shadow-colored", "0 0 #0000"));

    preset.rules.push(Rule::Static {
        token: "shadow".into(),
        entries: shadow_size_entries(
            "0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)",
        ),
    });

    // `shadow-none` keeps both default and colored at transparent zero-shadow
    // so a subsequent color utility cannot accidentally reveal a shadow.
    preset.rules.push(Rule::Static {
        token: "shadow-none".into(),
        entries: CssEntries::new(vec![
            CssEntry::new("--tw-shadow", "0 0 #0000"),
            CssEntry::new("--tw-shadow-colored", "0 0 #0000"),
            CssEntry::new("box-shadow", "var(--tw-inset-shadow), var(--tw-shadow)"),
        ]),
    });

    preset.rules.push(Rule::Static {
        token: "shadow-inner".into(),
        entries: shadow_size_entries("inset 0 2px 4px 0 rgb(0 0 0 / 0.05)"),
    });

    preset.rules.push(Rule::Static {
        token: "shadow-inherit".into(),
        entries: shadow_color_entries("inherit"),
    });

    preset.rules.push(Rule::Static {
        token: "shadow-current".into(),
        entries: shadow_color_entries("currentColor"),
    });

    preset.rules.push(Rule::Static {
        token: "shadow-transparent".into(),
        entries: shadow_color_entries("transparent"),
    });

    preset.rules.push(Rule::Dynamic {
        pattern: r"^shadow-(.+)$".into(),
        handler: handle_shadow,
    });
}

/// Tries theme `shadow` section first (size), then `color` section (color).
fn handle_shadow(token: &str, theme: &Theme) -> Option<CssEntries> {
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^shadow-(.+)$").expect("valid regex"));
    let caps = RE.captures(token)?;
    let key = caps.get(1)?.as_str();

    if let Some(value) = theme.get("shadow", key) {
        return Some(shadow_size_entries(value));
    }

    if let Some(color) = theme.get("color", key) {
        return Some(shadow_color_entries(color));
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
    fn shadow_none() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "shadow-none"),
            Some(
                ".shadow-none { \
                 --tw-shadow: 0 0 #0000; \
                 --tw-shadow-colored: 0 0 #0000; \
                 box-shadow: var(--tw-inset-shadow), var(--tw-shadow); }"
                    .into()
            )
        );
    }

    #[test]
    fn shadow_md() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "shadow-md"),
            Some(
                ".shadow-md { \
                 --tw-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1); \
                 --tw-shadow-colored: 0 4px 6px -1px var(--tw-shadow-color), 0 2px 4px -2px var(--tw-shadow-color); \
                 box-shadow: var(--tw-inset-shadow), var(--tw-shadow); }"
                    .into()
            )
        );
    }

    #[test]
    fn shadow_bare() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "shadow"),
            Some(
                ".shadow { \
                 --tw-shadow: 0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1); \
                 --tw-shadow-colored: 0 1px 3px 0 var(--tw-shadow-color), 0 1px 2px -1px var(--tw-shadow-color); \
                 box-shadow: var(--tw-inset-shadow), var(--tw-shadow); }"
                    .into()
            )
        );
    }

    #[test]
    fn shadow_inner() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "shadow-inner"),
            Some(
                ".shadow-inner { \
                 --tw-shadow: inset 0 2px 4px 0 rgb(0 0 0 / 0.05); \
                 --tw-shadow-colored: inset 0 2px 4px 0 var(--tw-shadow-color); \
                 box-shadow: var(--tw-inset-shadow), var(--tw-shadow); }"
                    .into()
            )
        );
    }

    #[test]
    fn unknown_shadow_key() {
        let compiled = compile_tailwindcss4();
        assert_eq!(generate(&compiled, "shadow-99"), None);
    }

    #[test]
    fn shadow_red_500() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "shadow-red-500"),
            Some(
                ".shadow-red-500 { \
                 --tw-shadow-color: oklch(63.7% 0.237 25.331); \
                 --tw-shadow: var(--tw-shadow-colored); }"
                    .into()
            )
        );
    }

    #[test]
    fn shadow_transparent() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "shadow-transparent"),
            Some(
                ".shadow-transparent { \
                 --tw-shadow-color: transparent; \
                 --tw-shadow: var(--tw-shadow-colored); }"
                    .into()
            )
        );
    }

    #[test]
    fn shadow_current() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "shadow-current"),
            Some(
                ".shadow-current { \
                 --tw-shadow-color: currentColor; \
                 --tw-shadow: var(--tw-shadow-colored); }"
                    .into()
            )
        );
    }

    #[test]
    fn shadow_inherit() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "shadow-inherit"),
            Some(
                ".shadow-inherit { \
                 --tw-shadow-color: inherit; \
                 --tw-shadow: var(--tw-shadow-colored); }"
                    .into()
            )
        );
    }
}
