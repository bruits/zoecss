//! Variants that modify generated CSS output (selector or at-rule wrapping).

use std::borrow::Cow;

/// A variant modifies generated CSS output (selector or at-rule wrapping).
///
/// Examples: `hover:` wraps in `:hover` pseudo-class, `sm:` wraps in a media query.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Variant {
    /// Modifies the CSS selector. `template` uses `&` as placeholder for the original selector.
    Selector {
        name: Cow<'static, str>,
        template: Cow<'static, str>,
    },
    /// Wraps CSS output in an at-rule (media query, `@supports`, etc.).
    AtRule {
        name: Cow<'static, str>,
        rule: Cow<'static, str>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selector_variant() {
        let variant = Variant::Selector {
            name: "hover".into(),
            template: "&:hover".into(),
        };
        match &variant {
            Variant::Selector { name, template } => {
                assert_eq!(name, "hover");
                assert_eq!(template, "&:hover");
            }
            _ => panic!("expected Selector"),
        }
    }

    #[test]
    fn at_rule_variant() {
        let variant = Variant::AtRule {
            name: "sm".into(),
            rule: "@media (min-width: 640px)".into(),
        };
        match &variant {
            Variant::AtRule { name, rule } => {
                assert_eq!(name, "sm");
                assert_eq!(rule, "@media (min-width: 640px)");
            }
            _ => panic!("expected AtRule"),
        }
    }
}
