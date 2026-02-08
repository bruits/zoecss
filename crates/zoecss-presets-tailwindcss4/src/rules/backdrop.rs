use zoecss_config::{Preset, Rule};
use zoecss_core::{CssEntries, CssEntry};

/// Registers all backdrop-filter utility rules: blur, brightness, contrast,
/// grayscale, hue-rotate, invert, opacity, saturate, and sepia.
pub fn register(preset: &mut Preset) {
    preset.rules.push(Rule::Static {
        token: "backdrop-blur".into(),
        entries: CssEntries::new(vec![
            CssEntry::new("-webkit-backdrop-filter", "blur(8px)"),
            CssEntry::new("backdrop-filter", "blur(8px)"),
        ]),
    });

    preset.rules.push(Rule::Static {
        token: "backdrop-blur-none".into(),
        entries: CssEntries::new(vec![
            CssEntry::new("-webkit-backdrop-filter", "blur(0px)"),
            CssEntry::new("backdrop-filter", "blur(0px)"),
        ]),
    });

    preset.rules.push(Rule::Pattern {
        pattern: r"^backdrop-blur-(.+)$".into(),
        template: CssEntries::new(vec![
            CssEntry::new("-webkit-backdrop-filter", "blur({theme.blur.$1})"),
            CssEntry::new("backdrop-filter", "blur({theme.blur.$1})"),
        ]),
    });

    preset.rules.push(Rule::Pattern {
        pattern: r"^backdrop-brightness-(\d+)$".into(),
        template: CssEntries::new(vec![
            CssEntry::new("-webkit-backdrop-filter", "brightness($1%)"),
            CssEntry::new("backdrop-filter", "brightness($1%)"),
        ]),
    });

    preset.rules.push(Rule::Pattern {
        pattern: r"^backdrop-contrast-(\d+)$".into(),
        template: CssEntries::new(vec![
            CssEntry::new("-webkit-backdrop-filter", "contrast($1%)"),
            CssEntry::new("backdrop-filter", "contrast($1%)"),
        ]),
    });

    preset.rules.push(Rule::Static {
        token: "backdrop-grayscale".into(),
        entries: CssEntries::new(vec![
            CssEntry::new("-webkit-backdrop-filter", "grayscale(100%)"),
            CssEntry::new("backdrop-filter", "grayscale(100%)"),
        ]),
    });

    preset.rules.push(Rule::Static {
        token: "backdrop-grayscale-0".into(),
        entries: CssEntries::new(vec![
            CssEntry::new("-webkit-backdrop-filter", "grayscale(0%)"),
            CssEntry::new("backdrop-filter", "grayscale(0%)"),
        ]),
    });

    preset.rules.push(Rule::Pattern {
        pattern: r"^backdrop-hue-rotate-(\d+)$".into(),
        template: CssEntries::new(vec![
            CssEntry::new("-webkit-backdrop-filter", "hue-rotate($1deg)"),
            CssEntry::new("backdrop-filter", "hue-rotate($1deg)"),
        ]),
    });

    preset.rules.push(Rule::Static {
        token: "backdrop-invert".into(),
        entries: CssEntries::new(vec![
            CssEntry::new("-webkit-backdrop-filter", "invert(100%)"),
            CssEntry::new("backdrop-filter", "invert(100%)"),
        ]),
    });

    preset.rules.push(Rule::Static {
        token: "backdrop-invert-0".into(),
        entries: CssEntries::new(vec![
            CssEntry::new("-webkit-backdrop-filter", "invert(0%)"),
            CssEntry::new("backdrop-filter", "invert(0%)"),
        ]),
    });

    preset.rules.push(Rule::Pattern {
        pattern: r"^backdrop-opacity-(\d+)$".into(),
        template: CssEntries::new(vec![
            CssEntry::new("-webkit-backdrop-filter", "opacity($1%)"),
            CssEntry::new("backdrop-filter", "opacity($1%)"),
        ]),
    });

    preset.rules.push(Rule::Pattern {
        pattern: r"^backdrop-saturate-(\d+)$".into(),
        template: CssEntries::new(vec![
            CssEntry::new("-webkit-backdrop-filter", "saturate($1%)"),
            CssEntry::new("backdrop-filter", "saturate($1%)"),
        ]),
    });

    preset.rules.push(Rule::Static {
        token: "backdrop-sepia".into(),
        entries: CssEntries::new(vec![
            CssEntry::new("-webkit-backdrop-filter", "sepia(100%)"),
            CssEntry::new("backdrop-filter", "sepia(100%)"),
        ]),
    });

    preset.rules.push(Rule::Static {
        token: "backdrop-sepia-0".into(),
        entries: CssEntries::new(vec![
            CssEntry::new("-webkit-backdrop-filter", "sepia(0%)"),
            CssEntry::new("backdrop-filter", "sepia(0%)"),
        ]),
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
    fn backdrop_blur_bare() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "backdrop-blur"),
            Some(".backdrop-blur { -webkit-backdrop-filter: blur(8px); backdrop-filter: blur(8px); }".into())
        );
    }

    #[test]
    fn backdrop_blur_none() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "backdrop-blur-none"),
            Some(".backdrop-blur-none { -webkit-backdrop-filter: blur(0px); backdrop-filter: blur(0px); }".into())
        );
    }

    #[test]
    fn backdrop_blur_md() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "backdrop-blur-md"),
            Some(".backdrop-blur-md { -webkit-backdrop-filter: blur(12px); backdrop-filter: blur(12px); }".into())
        );
    }

    #[test]
    fn backdrop_blur_unknown_key() {
        let compiled = compile_tailwindcss4();
        assert_eq!(generate(&compiled, "backdrop-blur-99"), None);
    }

    #[test]
    fn backdrop_brightness_50() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "backdrop-brightness-50"),
            Some(".backdrop-brightness-50 { -webkit-backdrop-filter: brightness(50%); backdrop-filter: brightness(50%); }".into())
        );
    }

    #[test]
    fn backdrop_contrast_75() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "backdrop-contrast-75"),
            Some(".backdrop-contrast-75 { -webkit-backdrop-filter: contrast(75%); backdrop-filter: contrast(75%); }".into())
        );
    }

    #[test]
    fn backdrop_saturate_150() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "backdrop-saturate-150"),
            Some(".backdrop-saturate-150 { -webkit-backdrop-filter: saturate(150%); backdrop-filter: saturate(150%); }".into())
        );
    }

    #[test]
    fn backdrop_opacity_50() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "backdrop-opacity-50"),
            Some(".backdrop-opacity-50 { -webkit-backdrop-filter: opacity(50%); backdrop-filter: opacity(50%); }".into())
        );
    }

    #[test]
    fn backdrop_grayscale() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "backdrop-grayscale"),
            Some(".backdrop-grayscale { -webkit-backdrop-filter: grayscale(100%); backdrop-filter: grayscale(100%); }".into())
        );
    }

    #[test]
    fn backdrop_grayscale_0() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "backdrop-grayscale-0"),
            Some(".backdrop-grayscale-0 { -webkit-backdrop-filter: grayscale(0%); backdrop-filter: grayscale(0%); }".into())
        );
    }

    #[test]
    fn backdrop_invert() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "backdrop-invert"),
            Some(".backdrop-invert { -webkit-backdrop-filter: invert(100%); backdrop-filter: invert(100%); }".into())
        );
    }

    #[test]
    fn backdrop_invert_0() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "backdrop-invert-0"),
            Some(".backdrop-invert-0 { -webkit-backdrop-filter: invert(0%); backdrop-filter: invert(0%); }".into())
        );
    }

    #[test]
    fn backdrop_sepia() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "backdrop-sepia"),
            Some(".backdrop-sepia { -webkit-backdrop-filter: sepia(100%); backdrop-filter: sepia(100%); }".into())
        );
    }

    #[test]
    fn backdrop_sepia_0() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "backdrop-sepia-0"),
            Some(".backdrop-sepia-0 { -webkit-backdrop-filter: sepia(0%); backdrop-filter: sepia(0%); }".into())
        );
    }

    #[test]
    fn backdrop_hue_rotate_90() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "backdrop-hue-rotate-90"),
            Some(".backdrop-hue-rotate-90 { -webkit-backdrop-filter: hue-rotate(90deg); backdrop-filter: hue-rotate(90deg); }".into())
        );
    }

    #[test]
    fn backdrop_unknown_token() {
        let compiled = compile_tailwindcss4();
        assert_eq!(generate(&compiled, "backdrop-foobar"), None);
    }
}
