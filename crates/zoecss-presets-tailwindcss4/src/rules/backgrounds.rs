use zoecss_config::{Preset, Rule};
use zoecss_core::{CssEntries, CssEntry};

/// Registers background utility rules (attachment, clip, origin, position,
/// repeat, size).
pub fn register(preset: &mut Preset) {
    for (token, value) in [
        ("bg-fixed", "fixed"),
        ("bg-local", "local"),
        ("bg-scroll", "scroll"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("background-attachment", value)]),
        });
    }

    for (token, value) in [
        ("bg-clip-border", "border-box"),
        ("bg-clip-padding", "padding-box"),
        ("bg-clip-content", "content-box"),
        ("bg-clip-text", "text"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("background-clip", value)]),
        });
    }

    for (token, value) in [
        ("bg-origin-border", "border-box"),
        ("bg-origin-padding", "padding-box"),
        ("bg-origin-content", "content-box"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("background-origin", value)]),
        });
    }

    for (token, value) in [
        ("bg-bottom", "bottom"),
        ("bg-center", "center"),
        ("bg-left", "left"),
        ("bg-left-bottom", "left bottom"),
        ("bg-left-top", "left top"),
        ("bg-right", "right"),
        ("bg-right-bottom", "right bottom"),
        ("bg-right-top", "right top"),
        ("bg-top", "top"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("background-position", value)]),
        });
    }

    for (token, value) in [
        ("bg-repeat", "repeat"),
        ("bg-no-repeat", "no-repeat"),
        ("bg-repeat-x", "repeat-x"),
        ("bg-repeat-y", "repeat-y"),
        ("bg-repeat-round", "round"),
        ("bg-repeat-space", "space"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("background-repeat", value)]),
        });
    }

    for (token, value) in [
        ("bg-auto", "auto"),
        ("bg-cover", "cover"),
        ("bg-contain", "contain"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("background-size", value)]),
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
    fn bg_fixed() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "bg-fixed"),
            Some(".bg-fixed { background-attachment: fixed; }".into())
        );
    }

    #[test]
    fn bg_clip_text() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "bg-clip-text"),
            Some(".bg-clip-text { background-clip: text; }".into())
        );
    }

    #[test]
    fn bg_origin_padding() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "bg-origin-padding"),
            Some(".bg-origin-padding { background-origin: padding-box; }".into())
        );
    }

    #[test]
    fn bg_center() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "bg-center"),
            Some(".bg-center { background-position: center; }".into())
        );
    }

    #[test]
    fn bg_no_repeat() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "bg-no-repeat"),
            Some(".bg-no-repeat { background-repeat: no-repeat; }".into())
        );
    }

    #[test]
    fn bg_cover() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "bg-cover"),
            Some(".bg-cover { background-size: cover; }".into())
        );
    }
}
