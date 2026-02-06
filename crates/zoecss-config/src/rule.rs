//! Rules mapping utility tokens to CSS declarations.

use std::borrow::Cow;

use crate::entries::CssEntries;
use crate::theme::Theme;

/// A rule that maps utility tokens to CSS declarations.
///
/// `PartialEq`/`Eq` intentionally omitted: the `Dynamic` variant holds a `fn` pointer
/// whose address comparison is unpredictable across codegen units.
#[derive(Debug, Clone)]
pub enum Rule {
    /// Exact token match → CSS entries. Designed for O(1) HashMap lookup.
    Static {
        token: Cow<'static, str>,
        entries: CssEntries,
    },
    /// Regex pattern + template with captures and theme lookups.
    /// Template syntax: `$1`, `$2` for captures; `{theme.section.key}` for theme values.
    Pattern {
        pattern: Cow<'static, str>,
        template: CssEntries,
    },
    /// Regex pattern + Rust handler function. For logic that templates can't express.
    /// Lives in presets, not user TOML config.
    Dynamic {
        pattern: Cow<'static, str>,
        handler: fn(&str, &Theme) -> Option<CssEntries>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entries::CssEntry;

    #[test]
    fn static_rule() {
        let rule = Rule::Static {
            token: "flex".into(),
            entries: CssEntries::new(vec![CssEntry::new("display", "flex")]),
        };
        match &rule {
            Rule::Static { token, entries } => {
                assert_eq!(token, "flex");
                assert_eq!(entries.len(), 1);
            }
            _ => panic!("expected Static"),
        }
    }

    #[test]
    fn pattern_rule() {
        let rule = Rule::Pattern {
            pattern: r"p-(\d+)".into(),
            template: CssEntries::new(vec![CssEntry::new("padding", "$1rem")]),
        };
        match &rule {
            Rule::Pattern { pattern, template } => {
                assert_eq!(pattern, r"p-(\d+)");
                assert_eq!(template.len(), 1);
            }
            _ => panic!("expected Pattern"),
        }
    }

    #[test]
    fn dynamic_rule() {
        fn handler(_token: &str, _theme: &Theme) -> Option<CssEntries> {
            Some(CssEntries::new(vec![CssEntry::new("color", "red")]))
        }

        let rule = Rule::Dynamic {
            pattern: r"text-(.+)".into(),
            handler,
        };
        match &rule {
            Rule::Dynamic {
                pattern,
                handler: h,
            } => {
                assert_eq!(pattern, r"text-(.+)");
                let result = h("text-red", &Theme::default());
                assert!(result.is_some());
            }
            _ => panic!("expected Dynamic"),
        }
    }

    #[test]
    fn debug_formats() {
        let rule = Rule::Static {
            token: "block".into(),
            entries: CssEntries::new(vec![CssEntry::new("display", "block")]),
        };
        let debug = format!("{rule:?}");
        assert!(debug.contains("Static"));
    }
}
