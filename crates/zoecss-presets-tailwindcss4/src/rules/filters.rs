use zoecss_config::{Preset, Rule};
use zoecss_core::{CssEntries, CssEntry};

/// Registers grayscale, invert, and sepia filter utility rules.
pub fn register(preset: &mut Preset) {
    preset.rules.push(Rule::Static {
        token: "grayscale".into(),
        entries: CssEntries::new(vec![CssEntry::new("filter", "grayscale(100%)")]),
    });

    preset.rules.push(Rule::Static {
        token: "grayscale-0".into(),
        entries: CssEntries::new(vec![CssEntry::new("filter", "grayscale(0%)")]),
    });

    preset.rules.push(Rule::Static {
        token: "invert".into(),
        entries: CssEntries::new(vec![CssEntry::new("filter", "invert(100%)")]),
    });

    preset.rules.push(Rule::Static {
        token: "invert-0".into(),
        entries: CssEntries::new(vec![CssEntry::new("filter", "invert(0%)")]),
    });

    preset.rules.push(Rule::Static {
        token: "sepia".into(),
        entries: CssEntries::new(vec![CssEntry::new("filter", "sepia(100%)")]),
    });

    preset.rules.push(Rule::Static {
        token: "sepia-0".into(),
        entries: CssEntries::new(vec![CssEntry::new("filter", "sepia(0%)")]),
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
    fn grayscale() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "grayscale"),
            Some(".grayscale { filter: grayscale(100%); }".into())
        );
    }

    #[test]
    fn grayscale_0() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "grayscale-0"),
            Some(".grayscale-0 { filter: grayscale(0%); }".into())
        );
    }

    #[test]
    fn invert() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "invert"),
            Some(".invert { filter: invert(100%); }".into())
        );
    }

    #[test]
    fn invert_0() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "invert-0"),
            Some(".invert-0 { filter: invert(0%); }".into())
        );
    }

    #[test]
    fn sepia() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "sepia"),
            Some(".sepia { filter: sepia(100%); }".into())
        );
    }

    #[test]
    fn sepia_0() {
        let compiled = compile_tailwindcss4();
        assert_eq!(
            generate(&compiled, "sepia-0"),
            Some(".sepia-0 { filter: sepia(0%); }".into())
        );
    }
}
