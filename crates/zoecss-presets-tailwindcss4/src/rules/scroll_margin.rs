use zoecss_config::{Preset, Rule};
use zoecss_core::{CssEntries, CssEntry};

/// Registers scroll-margin utility rules consuming `{theme.spacing.$1}`.
///
/// Directional rules are registered before the shorthand so the engine's
/// first-match-wins strategy hits the specific pattern first.
pub fn register(preset: &mut Preset) {
    for (pattern, properties) in [
        (
            r"^scroll-mx-(.+)$",
            vec!["scroll-margin-left", "scroll-margin-right"],
        ),
        (
            r"^scroll-my-(.+)$",
            vec!["scroll-margin-top", "scroll-margin-bottom"],
        ),
        (r"^scroll-mt-(.+)$", vec!["scroll-margin-top"]),
        (r"^scroll-mr-(.+)$", vec!["scroll-margin-right"]),
        (r"^scroll-mb-(.+)$", vec!["scroll-margin-bottom"]),
        (r"^scroll-ml-(.+)$", vec!["scroll-margin-left"]),
        (r"^scroll-ms-(.+)$", vec!["scroll-margin-inline-start"]),
        (r"^scroll-me-(.+)$", vec!["scroll-margin-inline-end"]),
        (r"^scroll-m-(.+)$", vec!["scroll-margin"]),
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
    fn scroll_margin_shorthand() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "scroll-m-4"),
            Some(".scroll-m-4 { scroll-margin: 1rem; }".into())
        );
    }

    #[test]
    fn scroll_margin_x() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "scroll-mx-2"),
            Some(
                ".scroll-mx-2 { scroll-margin-left: 0.5rem; scroll-margin-right: 0.5rem; }".into()
            )
        );
    }

    #[test]
    fn scroll_margin_top() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "scroll-mt-4"),
            Some(".scroll-mt-4 { scroll-margin-top: 1rem; }".into())
        );
    }

    #[test]
    fn scroll_margin_inline_start() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "scroll-ms-4"),
            Some(".scroll-ms-4 { scroll-margin-inline-start: 1rem; }".into())
        );
    }

    #[test]
    fn unknown_spacing_key() {
        let compiled = compile_tailwindcss4();
        assert_eq!(generate(&compiled, "scroll-m-99"), None);
    }
}
