use regex::Regex;
use zoecss_config::{Preset, Rule};
use zoecss_core::{CssEntries, CssEntry, Theme};

/// Registers effects utility rules (mix-blend-mode, background-blend-mode, opacity).
pub fn register(preset: &mut Preset) {
    for (token, value) in [
        ("mix-blend-normal", "normal"),
        ("mix-blend-multiply", "multiply"),
        ("mix-blend-screen", "screen"),
        ("mix-blend-overlay", "overlay"),
        ("mix-blend-darken", "darken"),
        ("mix-blend-lighten", "lighten"),
        ("mix-blend-color-dodge", "color-dodge"),
        ("mix-blend-color-burn", "color-burn"),
        ("mix-blend-hard-light", "hard-light"),
        ("mix-blend-soft-light", "soft-light"),
        ("mix-blend-difference", "difference"),
        ("mix-blend-exclusion", "exclusion"),
        ("mix-blend-hue", "hue"),
        ("mix-blend-saturation", "saturation"),
        ("mix-blend-color", "color"),
        ("mix-blend-luminosity", "luminosity"),
        ("mix-blend-plus-darker", "plus-darker"),
        ("mix-blend-plus-lighter", "plus-lighter"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("mix-blend-mode", value)]),
        });
    }

    for (token, value) in [
        ("bg-blend-normal", "normal"),
        ("bg-blend-multiply", "multiply"),
        ("bg-blend-screen", "screen"),
        ("bg-blend-overlay", "overlay"),
        ("bg-blend-darken", "darken"),
        ("bg-blend-lighten", "lighten"),
        ("bg-blend-color-dodge", "color-dodge"),
        ("bg-blend-color-burn", "color-burn"),
        ("bg-blend-hard-light", "hard-light"),
        ("bg-blend-soft-light", "soft-light"),
        ("bg-blend-difference", "difference"),
        ("bg-blend-exclusion", "exclusion"),
        ("bg-blend-hue", "hue"),
        ("bg-blend-saturation", "saturation"),
        ("bg-blend-color", "color"),
        ("bg-blend-luminosity", "luminosity"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("background-blend-mode", value)]),
        });
    }

    // Numeric values
    preset.rules.push(Rule::Pattern {
        pattern: r"^opacity-(\d+)$".into(),
        template: CssEntries::new(vec![CssEntry::new("opacity", "$1%")]),
    });

    // Arbitrary values
    preset.rules.push(Rule::Dynamic {
        pattern: r"^opacity-\[(.+)\]$".into(),
        handler: handle_arbitrary_opacity,
    });
}

fn handle_arbitrary_opacity(token: &str, _theme: &Theme) -> Option<CssEntries> {
    use std::sync::LazyLock;
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^opacity-\[(.+)\]$").expect("valid regex"));
    let caps = RE.captures(token)?;
    let value = caps.get(1)?.as_str();
    Some(CssEntries::new(vec![CssEntry::new(
        "opacity",
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
    fn mix_blend_multiply() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "mix-blend-multiply"),
            Some(".mix-blend-multiply { mix-blend-mode: multiply; }".into())
        );
    }

    #[test]
    fn bg_blend_screen() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "bg-blend-screen"),
            Some(".bg-blend-screen { background-blend-mode: screen; }".into())
        );
    }

    #[test]
    fn opacity_50() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "opacity-50"),
            Some(".opacity-50 { opacity: 50%; }".into())
        );
    }

    #[test]
    fn opacity_100() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "opacity-100"),
            Some(".opacity-100 { opacity: 100%; }".into())
        );
    }

    #[test]
    fn opacity_0() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "opacity-0"),
            Some(".opacity-0 { opacity: 0%; }".into())
        );
    }

    #[test]
    fn opacity_arbitrary() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "opacity-[0.5]"),
            Some(".opacity-\\[0\\.5\\] { opacity: 0.5; }".into())
        );
    }
}
