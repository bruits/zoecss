//! Compiled configuration optimized for fast token matching.
//!
//! Built once from a merged [`Config`], then queried immutably. Static rules use
//! O(1) hash-map lookup; pattern and dynamic rules share a single-pass `RegexSet`
//! DFA with a parallel `Vec` for capture extraction on match.

use std::borrow::Cow;

use rustc_hash::FxHashMap;

use regex::Regex;
use regex::RegexSet;

use zoecss_core::{CssEngine, CssEntries, CssEntry, Theme, Variant};

use crate::config::Config;
use crate::rule::Rule;

/// A compiled regex rule, ready for capture extraction on match.
#[derive(Debug, Clone)]
pub enum CompiledRegexRule {
    /// Pattern rule: regex + CSS entry templates containing `$1`/`$2` capture placeholders.
    Pattern { regex: Regex, template: CssEntries },
    /// Dynamic rule: regex + handler function pointer.
    Dynamic {
        regex: Regex,
        handler: fn(&str, &Theme) -> Option<CssEntries>,
    },
}

/// Compiled configuration optimized for fast token matching.
///
/// Static rules live in an `FxHashMap` for O(1) exact-match lookup.
/// Pattern and dynamic rules are indexed by a `RegexSet` for single-pass DFA matching,
/// backed by a parallel `Vec<CompiledRegexRule>` for per-match capture extraction.
#[derive(Debug, Clone)]
pub struct CompiledConfig {
    static_rules: FxHashMap<Cow<'static, str>, CssEntries>,
    regex_set: RegexSet,
    regex_rules: Vec<CompiledRegexRule>,
    variants: FxHashMap<Cow<'static, str>, Variant>,
    theme: Theme,
    base_css: String,
}

impl CompiledConfig {
    /// Compiles a merged [`Config`] into an optimized runtime form.
    ///
    /// Returns an error if any rule contains an invalid regex pattern.
    pub fn compile(config: Config) -> crate::error::Result<Self> {
        use crate::error::ConfigError;

        let mut static_rules = FxHashMap::default();
        let mut patterns: Vec<Cow<'static, str>> = Vec::new();
        let mut regex_rules = Vec::new();

        for rule in config.rules {
            match rule {
                Rule::Static { token, entries } => {
                    static_rules.insert(token, entries);
                }
                Rule::Pattern { pattern, template } => {
                    let regex = Regex::new(&pattern).map_err(|e| ConfigError::InvalidRegex {
                        pattern: pattern.to_string(),
                        message: e.to_string(),
                    })?;
                    patterns.push(pattern);
                    regex_rules.push(CompiledRegexRule::Pattern { regex, template });
                }
                Rule::Dynamic { pattern, handler } => {
                    let regex = Regex::new(&pattern).map_err(|e| ConfigError::InvalidRegex {
                        pattern: pattern.to_string(),
                        message: e.to_string(),
                    })?;
                    patterns.push(pattern);
                    regex_rules.push(CompiledRegexRule::Dynamic { regex, handler });
                }
            }
        }

        let regex_set = RegexSet::new(&patterns).map_err(|e| ConfigError::InvalidRegex {
            pattern: patterns
                .iter()
                .map(|p| p.as_ref())
                .collect::<Vec<_>>()
                .join(", "),
            message: e.to_string(),
        })?;

        let mut variants = FxHashMap::default();
        for variant in config.variants {
            let name = match &variant {
                Variant::Selector { name, .. } | Variant::AtRule { name, .. } => name.clone(),
            };
            variants.insert(name, variant);
        }

        let theme_props = config.theme.to_custom_properties();
        let user_base = config.base_css.join("\n");

        // Deduplicate property defaults with last-wins semantics
        let deduped_defaults = {
            let mut seen = Vec::<CssEntry>::new();
            for entry in config.property_defaults {
                if let Some(pos) = seen.iter().position(|e| e.property == entry.property) {
                    seen[pos].value = entry.value;
                } else {
                    seen.push(entry);
                }
            }
            seen
        };

        let defaults_block = if deduped_defaults.is_empty() {
            String::new()
        } else {
            let declarations: Vec<String> = deduped_defaults
                .iter()
                .map(|entry| format!("{}: {}", entry.property, entry.value))
                .collect();
            format!(
                "*, ::after, ::before, ::backdrop, ::file-selector-button {{ {} }}",
                declarations.join("; ")
            )
        };

        let base_css = [user_base, defaults_block, theme_props]
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("\n");

        Ok(Self {
            static_rules,
            regex_set,
            regex_rules,
            variants,
            theme: config.theme,
            base_css,
        })
    }

    /// O(1) lookup for a static rule by exact token.
    pub fn get_static(&self, token: &str) -> Option<&CssEntries> {
        self.static_rules.get(token)
    }

    /// Matches `token` against all pattern/dynamic regexes in a single DFA pass,
    /// returning the corresponding [`CompiledRegexRule`]s for capture extraction.
    pub fn match_regex(&self, token: &str) -> Vec<&CompiledRegexRule> {
        self.regex_set
            .matches(token)
            .iter()
            .map(|i| &self.regex_rules[i])
            .collect()
    }

    /// Returns the theme, needed for `{theme.section.key}` substitution and dynamic handlers.
    pub fn theme(&self) -> &Theme {
        &self.theme
    }
}

impl CssEngine for CompiledConfig {
    fn resolve_token(&self, token: &str) -> Option<CssEntries> {
        if let Some(entries) = self.get_static(token) {
            return Some(entries.clone());
        }

        let matches = self.match_regex(token);
        for rule in matches {
            match rule {
                CompiledRegexRule::Pattern { regex, template } => {
                    if let Some(entries) = substitute_captures(template, regex, token, &self.theme)
                    {
                        return Some(entries);
                    }
                }
                CompiledRegexRule::Dynamic { regex, handler } => {
                    if regex.is_match(token)
                        && let Some(entries) = handler(token, &self.theme)
                    {
                        return Some(entries);
                    }
                }
            }
        }

        None
    }

    fn get_variant(&self, name: &str) -> Option<&Variant> {
        self.variants.get(name)
    }

    fn base_css(&self) -> &str {
        &self.base_css
    }
}

/// Replaces `$1`, `$2`… with regex capture groups and `{theme.section.key}`
/// with theme lookups. Returns `None` if a required capture is missing or a
/// theme lookup fails.
fn substitute_captures(
    template: &CssEntries,
    regex: &Regex,
    token: &str,
    theme: &Theme,
) -> Option<CssEntries> {
    let caps = regex.captures(token)?;

    let entries = template
        .0
        .iter()
        .map(|entry| {
            let property = substitute_str(&entry.property, &caps, theme)?;
            let value = substitute_str(&entry.value, &caps, theme)?;
            Some(CssEntry::new(property, value))
        })
        .collect::<Option<Vec<_>>>()?;

    Some(CssEntries::new(entries))
}

/// Performs `$N` capture and `{theme.section.key}` substitution on a single string.
fn substitute_str(input: &str, caps: &regex::Captures<'_>, theme: &Theme) -> Option<String> {
    let mut result = input.to_owned();

    // Replace $1, $2, … capture placeholders (descending order avoids $1 shadowing $10).
    for i in (1..caps.len()).rev() {
        let placeholder = format!("${i}");
        if result.contains(&placeholder) {
            let value = caps
                .get(i)
                .map(|m: regex::Match<'_>| m.as_str())
                .unwrap_or("");
            result = result.replace(&placeholder, value);
        }
    }

    // Replace {theme.section.key} placeholders.
    while let Some(start) = result.find("{theme.") {
        let end = result[start..].find('}')? + start;
        let path = &result[start + 7..end]; // skip "{theme."
        let (section, key) = path.split_once('.')?;
        let value = theme.get(section, key)?;
        result = format!("{}{value}{}", &result[..start], &result[end + 1..]);
    }

    // Reject if any $N placeholder survived (out-of-range capture reference).
    if has_unresolved_placeholder(&result) {
        return None;
    }

    Some(result)
}

/// Returns `true` when `s` contains a `$` immediately followed by an ASCII digit.
fn has_unresolved_placeholder(s: &str) -> bool {
    let bytes = s.as_bytes();
    bytes
        .windows(2)
        .any(|w| w[0] == b'$' && w[1].is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;
    use zoecss_core::generate;

    #[test]
    fn compile_empty_config() {
        let compiled = CompiledConfig::compile(Config::new()).expect("valid test config");
        assert!(compiled.get_static("anything").is_none());
        assert!(compiled.match_regex("anything").is_empty());
        assert!(compiled.get_variant("hover").is_none());
        assert!(compiled.theme().sections.is_empty());
    }

    #[test]
    fn compile_static_rule_lookup() {
        let mut config = Config::new();
        config.rules.push(Rule::Static {
            token: "flex".into(),
            entries: CssEntries::new(vec![CssEntry::new("display", "flex")]),
        });

        let compiled = CompiledConfig::compile(config).expect("valid test config");
        let entries = compiled.get_static("flex").expect("should find 'flex'");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries.0[0].property, "display");
        assert_eq!(entries.0[0].value, "flex");
    }

    #[test]
    fn compile_static_rule_miss() {
        let mut config = Config::new();
        config.rules.push(Rule::Static {
            token: "flex".into(),
            entries: CssEntries::new(vec![CssEntry::new("display", "flex")]),
        });

        let compiled = CompiledConfig::compile(config).expect("valid test config");
        assert!(compiled.get_static("block").is_none());
    }

    #[test]
    fn compile_pattern_rule_match() {
        let mut config = Config::new();
        config.rules.push(Rule::Pattern {
            pattern: r"^p-(\d+)$".into(),
            template: CssEntries::new(vec![CssEntry::new("padding", "$1rem")]),
        });

        let compiled = CompiledConfig::compile(config).expect("valid test config");
        let matches = compiled.match_regex("p-4");
        assert_eq!(matches.len(), 1);
        match &matches[0] {
            CompiledRegexRule::Pattern { regex, template } => {
                assert!(regex.is_match("p-4"));
                assert_eq!(template.len(), 1);
            }
            _ => panic!("expected Pattern"),
        }
    }

    #[test]
    fn compile_dynamic_rule_match() {
        fn handler(_token: &str, _theme: &Theme) -> Option<CssEntries> {
            Some(CssEntries::new(vec![CssEntry::new("color", "red")]))
        }

        let mut config = Config::new();
        config.rules.push(Rule::Dynamic {
            pattern: r"^text-(.+)$".into(),
            handler,
        });

        let compiled = CompiledConfig::compile(config).expect("valid test config");
        let matches = compiled.match_regex("text-red");
        assert_eq!(matches.len(), 1);
        match &matches[0] {
            CompiledRegexRule::Dynamic { regex, handler: h } => {
                assert!(regex.is_match("text-red"));
                let result = h("text-red", compiled.theme());
                assert!(result.is_some());
            }
            _ => panic!("expected Dynamic"),
        }
    }

    #[test]
    fn compile_variant_lookup() {
        let mut config = Config::new();
        config.variants.push(Variant::Selector {
            name: "hover".into(),
            template: "&:hover".into(),
        });

        let compiled = CompiledConfig::compile(config).expect("valid test config");
        let variant = compiled.get_variant("hover").expect("should find 'hover'");
        assert_eq!(
            variant,
            &Variant::Selector {
                name: "hover".into(),
                template: "&:hover".into(),
            }
        );
    }

    #[test]
    fn compile_variant_miss() {
        let compiled = CompiledConfig::compile(Config::new()).expect("valid test config");
        assert!(compiled.get_variant("hover").is_none());
    }

    #[test]
    fn compile_mixed_rules() {
        fn handler(_token: &str, _theme: &Theme) -> Option<CssEntries> {
            Some(CssEntries::new(vec![CssEntry::new("color", "red")]))
        }

        let mut config = Config::new();
        config.rules.push(Rule::Static {
            token: "flex".into(),
            entries: CssEntries::new(vec![CssEntry::new("display", "flex")]),
        });
        config.rules.push(Rule::Pattern {
            pattern: r"^p-(\d+)$".into(),
            template: CssEntries::new(vec![CssEntry::new("padding", "$1rem")]),
        });
        config.rules.push(Rule::Dynamic {
            pattern: r"^text-(.+)$".into(),
            handler,
        });
        config.variants.push(Variant::Selector {
            name: "hover".into(),
            template: "&:hover".into(),
        });
        config.theme.insert("colors", "red", "#ef4444");

        let compiled = CompiledConfig::compile(config).expect("valid test config");

        assert!(compiled.get_static("flex").is_some());
        assert!(compiled.get_static("p-4").is_none());

        assert_eq!(compiled.match_regex("p-4").len(), 1);
        assert_eq!(compiled.match_regex("text-red").len(), 1);
        assert!(compiled.match_regex("flex").is_empty());

        assert!(compiled.get_variant("hover").is_some());
        assert_eq!(compiled.theme().get("colors", "red"), Some("#ef4444"));
    }

    #[test]
    fn compile_multiple_regex_matches() {
        let mut config = Config::new();
        config.rules.push(Rule::Pattern {
            pattern: r"^p-.+$".into(),
            template: CssEntries::new(vec![CssEntry::new("padding", "match-1")]),
        });
        config.rules.push(Rule::Pattern {
            pattern: r"^p-\d+$".into(),
            template: CssEntries::new(vec![CssEntry::new("padding", "match-2")]),
        });

        let compiled = CompiledConfig::compile(config).expect("valid test config");
        let matches = compiled.match_regex("p-4");
        assert_eq!(matches.len(), 2);

        match &matches[0] {
            CompiledRegexRule::Pattern { template, .. } => {
                assert_eq!(template.0[0].value, "match-1");
            }
            _ => panic!("expected Pattern"),
        }
        match &matches[1] {
            CompiledRegexRule::Pattern { template, .. } => {
                assert_eq!(template.0[0].value, "match-2");
            }
            _ => panic!("expected Pattern"),
        }
    }

    fn compile(config: Config) -> CompiledConfig {
        CompiledConfig::compile(config.merge()).expect("valid test config")
    }

    #[test]
    fn generate_static_rule() {
        let mut config = Config::new();
        config.rules.push(Rule::Static {
            token: "flex".into(),
            entries: CssEntries::new(vec![CssEntry::new("display", "flex")]),
        });
        let compiled = compile(config);
        assert_eq!(
            generate(&compiled, "flex").unwrap(),
            ".flex { display: flex; }"
        );
    }

    #[test]
    fn generate_pattern_rule_with_captures() {
        let mut config = Config::new();
        config.rules.push(Rule::Pattern {
            pattern: r"^p-(\d+)$".into(),
            template: CssEntries::new(vec![CssEntry::new("padding", "$1rem")]),
        });
        let compiled = compile(config);
        assert_eq!(
            generate(&compiled, "p-4").unwrap(),
            ".p-4 { padding: 4rem; }"
        );
    }

    #[test]
    fn generate_pattern_rule_with_theme_substitution() {
        let mut config = Config::new();
        config.theme.insert("colors", "red", "#ef4444");
        config.rules.push(Rule::Pattern {
            pattern: r"^text-(.+)$".into(),
            template: CssEntries::new(vec![CssEntry::new("color", "{theme.colors.$1}")]),
        });
        let compiled = compile(config);
        assert_eq!(
            generate(&compiled, "text-red").unwrap(),
            ".text-red { color: #ef4444; }"
        );
    }

    #[test]
    fn generate_dynamic_rule() {
        fn handler(_token: &str, _theme: &Theme) -> Option<CssEntries> {
            Some(CssEntries::new(vec![CssEntry::new("color", "red")]))
        }

        let mut config = Config::new();
        config.rules.push(Rule::Dynamic {
            pattern: r"^custom-(.+)$".into(),
            handler,
        });
        let compiled = compile(config);
        assert_eq!(
            generate(&compiled, "custom-foo").unwrap(),
            ".custom-foo { color: red; }"
        );
    }

    #[test]
    fn generate_unknown_token_returns_none() {
        let compiled = compile(Config::new());
        assert!(generate(&compiled, "does-not-exist").is_none());
    }

    #[test]
    fn generate_selector_variant() {
        let mut config = Config::new();
        config.rules.push(Rule::Static {
            token: "flex".into(),
            entries: CssEntries::new(vec![CssEntry::new("display", "flex")]),
        });
        config.variants.push(Variant::Selector {
            name: "hover".into(),
            template: "&:hover".into(),
        });
        let compiled = compile(config);
        assert_eq!(
            generate(&compiled, "hover:flex").unwrap(),
            ".hover\\:flex:hover { display: flex; }"
        );
    }

    #[test]
    fn generate_at_rule_variant() {
        let mut config = Config::new();
        config.rules.push(Rule::Static {
            token: "flex".into(),
            entries: CssEntries::new(vec![CssEntry::new("display", "flex")]),
        });
        config.variants.push(Variant::AtRule {
            name: "sm".into(),
            rule: "@media (min-width: 640px)".into(),
        });
        let compiled = compile(config);
        assert_eq!(
            generate(&compiled, "sm:flex").unwrap(),
            "@media (min-width: 640px) { .sm\\:flex { display: flex; } }"
        );
    }

    #[test]
    fn generate_composed_variants_selector_and_at_rule() {
        let mut config = Config::new();
        config.rules.push(Rule::Static {
            token: "flex".into(),
            entries: CssEntries::new(vec![CssEntry::new("display", "flex")]),
        });
        config.variants.push(Variant::Selector {
            name: "hover".into(),
            template: "&:hover".into(),
        });
        config.variants.push(Variant::AtRule {
            name: "sm".into(),
            rule: "@media (min-width: 640px)".into(),
        });
        let compiled = compile(config);

        let result = generate(&compiled, "hover:sm:flex").unwrap();
        assert_eq!(
            result,
            "@media (min-width: 640px) { .hover\\:sm\\:flex:hover { display: flex; } }"
        );
    }

    #[test]
    fn generate_unknown_variant_falls_through_to_base_token() {
        let mut config = Config::new();
        config.rules.push(Rule::Static {
            token: "unknown:flex".into(),
            entries: CssEntries::new(vec![CssEntry::new("display", "flex")]),
        });
        let compiled = compile(config);
        assert_eq!(
            generate(&compiled, "unknown:flex").unwrap(),
            ".unknown\\:flex { display: flex; }"
        );
    }

    #[test]
    fn generate_unknown_variant_no_match() {
        let mut config = Config::new();
        config.rules.push(Rule::Static {
            token: "flex".into(),
            entries: CssEntries::new(vec![CssEntry::new("display", "flex")]),
        });
        let compiled = compile(config);
        assert!(generate(&compiled, "fake:flex").is_none());
    }

    #[test]
    fn generate_multiple_entries() {
        let mut config = Config::new();
        config.rules.push(Rule::Static {
            token: "inset-0".into(),
            entries: CssEntries::new(vec![
                CssEntry::new("top", "0"),
                CssEntry::new("right", "0"),
                CssEntry::new("bottom", "0"),
                CssEntry::new("left", "0"),
            ]),
        });
        let compiled = compile(config);
        assert_eq!(
            generate(&compiled, "inset-0").unwrap(),
            ".inset-0 { top: 0; right: 0; bottom: 0; left: 0; }"
        );
    }

    #[test]
    fn generate_pattern_with_only_capture_substitution() {
        let mut config = Config::new();
        config.rules.push(Rule::Pattern {
            pattern: r"^m-(\d+)$".into(),
            template: CssEntries::new(vec![CssEntry::new("margin", "$1px")]),
        });
        let compiled = compile(config);
        assert_eq!(generate(&compiled, "m-8").unwrap(), ".m-8 { margin: 8px; }");
    }

    #[test]
    fn generate_dynamic_rule_returning_none() {
        fn handler(_token: &str, _theme: &Theme) -> Option<CssEntries> {
            None
        }

        let mut config = Config::new();
        config.rules.push(Rule::Dynamic {
            pattern: r"^fail-(.+)$".into(),
            handler,
        });
        let compiled = compile(config);
        assert!(generate(&compiled, "fail-something").is_none());
    }

    #[test]
    fn generate_theme_lookup_failure_returns_none() {
        let mut config = Config::new();
        config.rules.push(Rule::Pattern {
            pattern: r"^bg-(.+)$".into(),
            template: CssEntries::new(vec![CssEntry::new("background-color", "{theme.colors.$1}")]),
        });
        let compiled = compile(config);
        assert!(generate(&compiled, "bg-missing").is_none());
    }

    #[test]
    fn generate_static_rule_takes_priority_over_regex() {
        let mut config = Config::new();
        config.rules.push(Rule::Static {
            token: "p-4".into(),
            entries: CssEntries::new(vec![CssEntry::new("padding", "1rem")]),
        });
        config.rules.push(Rule::Pattern {
            pattern: r"^p-(\d+)$".into(),
            template: CssEntries::new(vec![CssEntry::new("padding", "$1px")]),
        });
        let compiled = compile(config);
        assert_eq!(
            generate(&compiled, "p-4").unwrap(),
            ".p-4 { padding: 1rem; }"
        );
    }

    #[test]
    fn generate_pattern_with_multiple_captures() {
        let mut config = Config::new();
        config.rules.push(Rule::Pattern {
            pattern: r"^p-(\d+)-(\d+)$".into(),
            template: CssEntries::new(vec![CssEntry::new("padding", "$1rem $2rem")]),
        });
        let compiled = compile(config);
        assert_eq!(
            generate(&compiled, "p-2-4").unwrap(),
            ".p-2-4 { padding: 2rem 4rem; }"
        );
    }

    #[test]
    fn generate_out_of_range_capture_returns_none() {
        let mut config = Config::new();
        config.rules.push(Rule::Pattern {
            pattern: r"^p-(\d+)$".into(),
            template: CssEntries::new(vec![CssEntry::new("padding", "$1rem $3rem")]),
        });
        let compiled = compile(config);
        assert!(generate(&compiled, "p-4").is_none());
    }

    #[test]
    fn generate_variant_with_pattern_rule() {
        let mut config = Config::new();
        config.rules.push(Rule::Pattern {
            pattern: r"^p-(\d+)$".into(),
            template: CssEntries::new(vec![CssEntry::new("padding", "$1rem")]),
        });
        config.variants.push(Variant::Selector {
            name: "hover".into(),
            template: "&:hover".into(),
        });
        let compiled = compile(config);
        assert_eq!(
            generate(&compiled, "hover:p-4").unwrap(),
            ".hover\\:p-4:hover { padding: 4rem; }"
        );
    }

    #[test]
    fn compile_invalid_regex_returns_error() {
        let mut config = Config::new();
        config.rules.push(Rule::Pattern {
            pattern: r"^p-[invalid".into(),
            template: CssEntries::new(vec![CssEntry::new("padding", "$1rem")]),
        });

        let result = CompiledConfig::compile(config);
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert!(err.to_string().contains("p-[invalid"));
    }

    #[test]
    fn property_defaults_dedup_last_wins() {
        let mut config = Config::new();
        config
            .property_defaults
            .push(CssEntry::new("--tw-foo", "first"));
        config
            .property_defaults
            .push(CssEntry::new("--tw-foo", "second"));

        let compiled = compile(config);
        let base = compiled.base_css();

        assert!(base.contains("second"), "last value should survive dedup");
        assert!(!base.contains("first"), "first value should be overwritten");
    }
}
