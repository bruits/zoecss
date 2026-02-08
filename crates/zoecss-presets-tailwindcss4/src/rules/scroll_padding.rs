use zoecss_config::{Preset, Rule};
use zoecss_core::{CssEntries, CssEntry};

/// Registers scroll-padding utility rules consuming `{theme.spacing.$1}`.
///
/// Directional rules are registered before the shorthand so the engine's
/// first-match-wins strategy hits the specific pattern first.
pub fn register(preset: &mut Preset) {
    for (pattern, properties) in [
        (
            r"^scroll-px-(.+)$",
            vec!["scroll-padding-left", "scroll-padding-right"],
        ),
        (
            r"^scroll-py-(.+)$",
            vec!["scroll-padding-top", "scroll-padding-bottom"],
        ),
        (r"^scroll-pt-(.+)$", vec!["scroll-padding-top"]),
        (r"^scroll-pr-(.+)$", vec!["scroll-padding-right"]),
        (r"^scroll-pb-(.+)$", vec!["scroll-padding-bottom"]),
        (r"^scroll-pl-(.+)$", vec!["scroll-padding-left"]),
        (r"^scroll-ps-(.+)$", vec!["scroll-padding-inline-start"]),
        (r"^scroll-pe-(.+)$", vec!["scroll-padding-inline-end"]),
        (r"^scroll-p-(.+)$", vec!["scroll-padding"]),
    ] {
        preset.rules.push(Rule::Pattern {
            pattern: pattern.into(),
            template: CssEntries::new(
                properties
                    .into_iter()
                    .map(|prop| CssEntry::new(prop, "{theme.spacing.$1}"))
                    .collect(),
            ),
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
    fn scroll_padding_shorthand() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "scroll-p-4"),
            Some(".scroll-p-4 { scroll-padding: 1rem; }".into())
        );
    }

    #[test]
    fn scroll_padding_x() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "scroll-px-2"),
            Some(
                ".scroll-px-2 { scroll-padding-left: 0.5rem; scroll-padding-right: 0.5rem; }"
                    .into()
            )
        );
    }

    #[test]
    fn scroll_padding_bottom() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "scroll-pb-4"),
            Some(".scroll-pb-4 { scroll-padding-bottom: 1rem; }".into())
        );
    }

    #[test]
    fn scroll_padding_inline_end() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "scroll-pe-4"),
            Some(".scroll-pe-4 { scroll-padding-inline-end: 1rem; }".into())
        );
    }

    #[test]
    fn unknown_spacing_key() {
        let compiled = compile_tailwindcss4();
        assert_eq!(generate(&compiled, "scroll-p-99"), None);
    }
}
