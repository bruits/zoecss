use zoecss_config::{Preset, Rule};
use zoecss_core::{CssEntries, CssEntry};

/// Registers stroke-width utility rules.
///
/// Static rules resolve before pattern rules, so these take priority over the
/// `^stroke-(.+)$` color pattern declared in `color.rs`.
pub fn register(preset: &mut Preset) {
    for (token, value) in [("stroke-0", "0"), ("stroke-1", "1"), ("stroke-2", "2")] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("stroke-width", value)]),
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
    fn stroke_0() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "stroke-0"),
            Some(".stroke-0 { stroke-width: 0; }".into())
        );
    }

    #[test]
    fn stroke_1() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "stroke-1"),
            Some(".stroke-1 { stroke-width: 1; }".into())
        );
    }

    #[test]
    fn stroke_2() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "stroke-2"),
            Some(".stroke-2 { stroke-width: 2; }".into())
        );
    }
}
