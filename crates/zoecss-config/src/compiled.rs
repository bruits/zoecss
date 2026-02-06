//! Compiled configuration optimized for fast token matching.
//!
//! Built once from a merged [`Config`], then queried immutably. Static rules use
//! O(1) `HashMap` lookup; pattern and dynamic rules share a single-pass `RegexSet`
//! DFA with a parallel `Vec` for capture extraction on match.

use std::borrow::Cow;
use std::collections::HashMap;

use regex::Regex;
use regex::RegexSet;

use crate::config::Config;
use crate::entries::CssEntries;
use crate::rule::Rule;
use crate::theme::Theme;
use crate::variant::Variant;

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
/// Static rules live in a `HashMap` for O(1) exact-match lookup.
/// Pattern and dynamic rules are indexed by a `RegexSet` for single-pass DFA matching,
/// backed by a parallel `Vec<CompiledRegexRule>` for per-match capture extraction.
#[derive(Debug, Clone)]
pub struct CompiledConfig {
    static_rules: HashMap<Cow<'static, str>, CssEntries>,
    regex_set: RegexSet,
    regex_rules: Vec<CompiledRegexRule>,
    variants: HashMap<Cow<'static, str>, Variant>,
    theme: Theme,
}

impl CompiledConfig {
    /// Compiles a merged [`Config`] into an optimized runtime form.
    pub fn compile(config: Config) -> Self {
        let mut static_rules = HashMap::new();
        let mut patterns: Vec<Cow<'static, str>> = Vec::new();
        let mut regex_rules = Vec::new();

        for rule in config.rules {
            match rule {
                Rule::Static { token, entries } => {
                    static_rules.insert(token, entries);
                }
                Rule::Pattern { pattern, template } => {
                    let regex = Regex::new(&pattern).expect("invalid pattern regex");
                    patterns.push(pattern);
                    regex_rules.push(CompiledRegexRule::Pattern { regex, template });
                }
                Rule::Dynamic { pattern, handler } => {
                    let regex = Regex::new(&pattern).expect("invalid dynamic regex");
                    patterns.push(pattern);
                    regex_rules.push(CompiledRegexRule::Dynamic { regex, handler });
                }
            }
        }

        let regex_set = RegexSet::new(&patterns).expect("invalid regex set");

        let mut variants = HashMap::new();
        for variant in config.variants {
            let name = match &variant {
                Variant::Selector { name, .. } | Variant::AtRule { name, .. } => name.clone(),
            };
            variants.insert(name, variant);
        }

        Self {
            static_rules,
            regex_set,
            regex_rules,
            variants,
            theme: config.theme,
        }
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

    /// O(1) lookup for a variant by name.
    pub fn get_variant(&self, name: &str) -> Option<&Variant> {
        self.variants.get(name)
    }

    /// Returns the theme, needed for `$theme(section, key)` substitution and dynamic handlers.
    pub fn theme(&self) -> &Theme {
        &self.theme
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entries::CssEntry;

    #[test]
    fn compile_empty_config() {
        let compiled = CompiledConfig::compile(Config::new());
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

        let compiled = CompiledConfig::compile(config);
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

        let compiled = CompiledConfig::compile(config);
        assert!(compiled.get_static("block").is_none());
    }

    #[test]
    fn compile_pattern_rule_match() {
        let mut config = Config::new();
        config.rules.push(Rule::Pattern {
            pattern: r"^p-(\d+)$".into(),
            template: CssEntries::new(vec![CssEntry::new("padding", "$1rem")]),
        });

        let compiled = CompiledConfig::compile(config);
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

        let compiled = CompiledConfig::compile(config);
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

        let compiled = CompiledConfig::compile(config);
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
        let compiled = CompiledConfig::compile(Config::new());
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

        let compiled = CompiledConfig::compile(config);

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

        let compiled = CompiledConfig::compile(config);
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
}
