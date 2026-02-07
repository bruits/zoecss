//! Token extraction — scans source files for utility class candidates.
//!
//! The `extract_tokens` function is content-type-agnostic: it works on HTML,
//! JSX, Vue, Svelte, or any textual source by splitting on delimiter characters
//! and keeping sequences that look like plausible CSS utility tokens.

use std::collections::HashSet;

/// Characters that may appear inside a utility token.
fn is_token_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric()
        || matches!(
            ch,
            '-' | '_' | ':' | '/' | '.' | '[' | ']' | '#' | '%' | '!' | '@' | ','
        )
}

/// Scans `content` for plausible CSS utility tokens.
///
/// Iterates character-by-character, collecting maximal sequences of "token
/// characters" (alphanumerics plus `-_:/.[]#%!@,`). Each candidate must
/// contain at least one ASCII letter to filter out pure numbers and
/// punctuation. Results are deduplicated in first-occurrence order.
pub fn extract_tokens(content: &str) -> Vec<&str> {
    let mut seen = HashSet::new();
    let mut tokens: Vec<&str> = Vec::new();

    let bytes = content.as_bytes();
    let len = bytes.len();
    let mut i = 0;

    while i < len {
        // All token characters are ASCII, so byte-level scanning is safe.
        if is_token_char(bytes[i] as char) {
            let start = i;
            let mut bracket_depth: u32 = 0;
            while i < len {
                let ch = bytes[i] as char;
                if ch == '[' {
                    bracket_depth += 1;
                    i += 1;
                } else if ch == ']' && bracket_depth > 0 {
                    bracket_depth -= 1;
                    i += 1;
                } else if bracket_depth > 0 && !ch.is_ascii_whitespace() {
                    i += 1;
                } else if is_token_char(ch) {
                    i += 1;
                } else {
                    break;
                }
            }
            let candidate = &content[start..i];
            let has_letter = candidate.bytes().any(|b| b.is_ascii_alphabetic());
            if has_letter && seen.insert(candidate) {
                tokens.push(candidate);
            }
        } else {
            i += 1;
        }
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn html_class_attribute() {
        let input = r#"<div class="flex p-4 text-red-500">"#;
        let tokens = extract_tokens(input);
        assert!(tokens.contains(&"flex"));
        assert!(tokens.contains(&"p-4"));
        assert!(tokens.contains(&"text-red-500"));
    }

    #[test]
    fn jsx_classname_with_variants_and_brackets() {
        let input = r#"<Component className="sm:p-4 text-[#fff]" />"#;
        let tokens = extract_tokens(input);
        assert!(tokens.contains(&"sm:p-4"));
        assert!(tokens.contains(&"text-[#fff]"));
    }

    #[test]
    fn multiline_attributes() {
        let input = "class=\"\n  flex\n  items-center\n  p-4\n\"";
        let tokens = extract_tokens(input);
        assert!(tokens.contains(&"flex"));
        assert!(tokens.contains(&"items-center"));
        assert!(tokens.contains(&"p-4"));
    }

    #[test]
    fn deduplicates_preserving_order() {
        let input = r#"class="flex p-4 flex p-4 mt-2""#;
        let tokens = extract_tokens(input);
        let flex_positions: Vec<_> = tokens
            .iter()
            .enumerate()
            .filter(|(_, t)| **t == "flex")
            .map(|(i, _)| i)
            .collect();
        assert_eq!(flex_positions.len(), 1, "flex should appear exactly once");

        // first-occurrence order: flex before mt-2
        let flex_idx = tokens.iter().position(|t| *t == "flex").unwrap();
        let mt_idx = tokens.iter().position(|t| *t == "mt-2").unwrap();
        assert!(flex_idx < mt_idx);
    }

    #[test]
    fn variant_prefixed_tokens() {
        let input = r#"class="hover:flex sm:p-4 dark:bg-black""#;
        let tokens = extract_tokens(input);
        assert!(tokens.contains(&"hover:flex"));
        assert!(tokens.contains(&"sm:p-4"));
        assert!(tokens.contains(&"dark:bg-black"));
    }

    #[test]
    fn bracket_notation() {
        let input = r#"class="text-[#fff] bg-[rgb(0,0,0)] w-[calc(100%-2rem)]""#;
        let tokens = extract_tokens(input);
        assert!(tokens.contains(&"text-[#fff]"));
        assert!(tokens.contains(&"bg-[rgb(0,0,0)]"));
        assert!(tokens.contains(&"w-[calc(100%-2rem)]"));
    }

    #[test]
    fn extract_tokens_bracket_notation_special_chars() {
        let content =
            r#"class="data-[state=open]:bg-red-500 w-[calc(100%+2rem)] min-w-[calc(100%*2)]""#;
        let result = extract_tokens(content);
        assert!(result.contains(&"data-[state=open]:bg-red-500"));
        assert!(result.contains(&"w-[calc(100%+2rem)]"));
        assert!(result.contains(&"min-w-[calc(100%*2)]"));
    }

    #[test]
    fn empty_input() {
        assert!(extract_tokens("").is_empty());
    }

    #[test]
    fn no_extractable_tokens() {
        // Pure tags with no class-like content yield only tag names
        let tokens = extract_tokens("<> </>");
        assert!(tokens.is_empty());
    }

    #[test]
    fn pure_numbers_filtered_out() {
        let input = "42 3.14 100%";
        let tokens = extract_tokens(input);
        // None of these contain a letter
        assert!(tokens.is_empty());
    }

    #[test]
    fn mixed_content_types() {
        let input = r#"
            <div class="flex p-4">
                <span className="text-lg font-bold">Hello</span>
                <style>.custom { color: red; }</style>
            </div>
        "#;
        let tokens = extract_tokens(input);
        assert!(tokens.contains(&"flex"));
        assert!(tokens.contains(&"p-4"));
        assert!(tokens.contains(&"text-lg"));
        assert!(tokens.contains(&"font-bold"));
    }
}
