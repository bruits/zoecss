use zoecss_config::{Preset, Rule};
use zoecss_core::{CssEntries, CssEntry};

/// Registers inset utility rules (top, right, bottom, left, inset, start, end)
/// consuming `{theme.spacing.$1}`.
///
/// Directional and axis rules are registered before the shorthand so the engine's
/// first-match-wins strategy hits the specific pattern first.
pub fn register(preset: &mut Preset) {
    for (pattern, properties) in [
        (r"^inset-x-(.+)$", vec!["left", "right"]),
        (r"^inset-y-(.+)$", vec!["top", "bottom"]),
        (r"^start-(.+)$", vec!["inset-inline-start"]),
        (r"^end-(.+)$", vec!["inset-inline-end"]),
        (r"^top-(.+)$", vec!["top"]),
        (r"^right-(.+)$", vec!["right"]),
        (r"^bottom-(.+)$", vec!["bottom"]),
        (r"^left-(.+)$", vec!["left"]),
        (r"^inset-(.+)$", vec!["inset"]),
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
    fn inset_shorthand() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "inset-4"),
            Some(".inset-4 { inset: 1rem; }".into())
        );
    }

    #[test]
    fn inset_x() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "inset-x-4"),
            Some(".inset-x-4 { left: 1rem; right: 1rem; }".into())
        );
    }

    #[test]
    fn inset_y() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "inset-y-2"),
            Some(".inset-y-2 { top: 0.5rem; bottom: 0.5rem; }".into())
        );
    }

    #[test]
    fn top() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "top-4"),
            Some(".top-4 { top: 1rem; }".into())
        );
    }

    #[test]
    fn right() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "right-2"),
            Some(".right-2 { right: 0.5rem; }".into())
        );
    }

    #[test]
    fn bottom() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "bottom-8"),
            Some(".bottom-8 { bottom: 2rem; }".into())
        );
    }

    #[test]
    fn left() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "left-4"),
            Some(".left-4 { left: 1rem; }".into())
        );
    }

    #[test]
    fn start() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "start-4"),
            Some(".start-4 { inset-inline-start: 1rem; }".into())
        );
    }

    #[test]
    fn end() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "end-2"),
            Some(".end-2 { inset-inline-end: 0.5rem; }".into())
        );
    }

    #[test]
    fn unknown_spacing_key() {
        let compiled = compile_tailwindcss4();
        assert_eq!(generate(&compiled, "inset-99"), None);
        assert_eq!(generate(&compiled, "top-99"), None);
    }
}
