//! Core CSS generation engine — matches tokens to CSS output.
//!
//! This crate owns the fundamental CSS types (`CssEntries`, `Variant`, `Theme`)
//! and the `CssEngine` trait. The `generate` function is generic over any
//! `CssEngine` implementation, decoupling the generation pipeline from any
//! specific configuration backend.

pub mod engine;
pub mod entries;
pub mod extractor;
pub mod theme;
pub mod variant;

pub use engine::CssEngine;
pub use entries::{CssEntries, CssEntry};
pub use extractor::extract_tokens;
pub use theme::Theme;
pub use variant::Variant;

/// Resolves a raw utility token into a complete CSS rule string.
///
/// Parses variant prefixes, delegates base-token resolution to the engine,
/// and formats the result as minimal CSS with variant wrappers.
///
/// Returns `None` when the token doesn't match any rule (or the engine
/// returns `None` during resolution).
pub fn generate(engine: &impl CssEngine, token: &str) -> Option<String> {
    let (variant_names, base_token) = parse_token(token, engine);

    let entries = engine.resolve_token(base_token)?;

    let variants: Vec<&Variant> = variant_names
        .iter()
        .map(|name| engine.get_variant(name))
        .collect::<Option<Vec<_>>>()?;

    Some(format_css(token, &entries, &variants))
}

/// Splits variant prefixes from the base utility token.
///
/// Greedily consumes colon-separated prefixes left-to-right as long as they
/// are recognised variant names. The remainder (including any unrecognised
/// prefix and everything after) is the base utility token.
fn parse_token<'a>(token: &'a str, engine: &impl CssEngine) -> (Vec<&'a str>, &'a str) {
    let mut variants = Vec::new();
    let mut rest = token;

    while let Some(colon_pos) = rest.find(':') {
        let candidate = &rest[..colon_pos];
        if engine.get_variant(candidate).is_some() {
            variants.push(candidate);
            rest = &rest[colon_pos + 1..];
        } else {
            break;
        }
    }

    (variants, rest)
}

/// Escapes special CSS characters in a class name so it can be used in a selector.
pub fn escape_css_class(name: &str) -> String {
    let mut escaped = String::with_capacity(name.len() * 2);
    for ch in name.chars() {
        match ch {
            ':' | '\\' | '/' | '.' | '#' | '!' | '[' | ']' | '(' | ')' | '{' | '}' | ',' | '>'
            | '+' | '~' | '=' | '%' | '@' => {
                escaped.push('\\');
                escaped.push(ch);
            }
            _ => escaped.push(ch),
        }
    }
    escaped
}

/// Formats the final CSS string with variant wrappers applied.
///
/// Variants are applied in reverse order (rightmost/innermost first) so the
/// leftmost variant ends up as the outermost wrapper.
fn format_css(token: &str, entries: &CssEntries, variants: &[&Variant]) -> String {
    let declarations: String = entries
        .0
        .iter()
        .map(|e| format!("{}: {};", e.property, e.value))
        .collect::<Vec<_>>()
        .join(" ");

    let class = escape_css_class(token);
    let mut selector = format!(".{class}");
    let mut css = format!("{selector} {{ {declarations} }}");

    // Apply variants in reverse (innermost first → leftmost becomes outermost).
    for variant in variants.iter().rev() {
        match variant {
            Variant::Selector { template, .. } => {
                let new_selector = template.replace('&', &selector);
                css = css.replacen(&selector, &new_selector, 1);
                selector = new_selector;
            }
            Variant::AtRule { rule, .. } => {
                css = format!("{rule} {{ {css} }}");
            }
        }
    }

    css
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_css_class_special_chars() {
        assert_eq!(escape_css_class("hover:flex"), "hover\\:flex");
        assert_eq!(escape_css_class("sm:hover:flex"), "sm\\:hover\\:flex");
        assert_eq!(escape_css_class("w-1/2"), "w-1\\/2");
        assert_eq!(escape_css_class("simple"), "simple");
    }
}
