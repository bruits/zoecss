//! Theme values organized in two-level sections.

use std::borrow::Cow;
use std::collections::HashMap;

/// Theme values organized in two-level sections.
///
/// Sections map to nested keys and values, e.g., `"colors" → "red" → "#ef4444"`.
/// Values use `Cow<'static, str>` for zero-copy preset defaults.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Theme {
    pub sections: HashMap<Cow<'static, str>, HashMap<Cow<'static, str>, Cow<'static, str>>>,
}

impl Theme {
    pub fn new() -> Self {
        Self {
            sections: HashMap::new(),
        }
    }

    /// Looks up a value by section and key.
    pub fn get(&self, section: &str, key: &str) -> Option<&str> {
        self.sections.get(section)?.get(key).map(|v| v.as_ref())
    }

    /// Inserts a value into a section, creating the section if needed.
    pub fn insert(
        &mut self,
        section: impl Into<Cow<'static, str>>,
        key: impl Into<Cow<'static, str>>,
        value: impl Into<Cow<'static, str>>,
    ) {
        self.sections
            .entry(section.into())
            .or_default()
            .insert(key.into(), value.into());
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_and_get() {
        let mut theme = Theme::new();
        theme.insert("colors", "red", "#ef4444");
        assert_eq!(theme.get("colors", "red"), Some("#ef4444"));
    }

    #[test]
    fn missing_section_returns_none() {
        let theme = Theme::new();
        assert_eq!(theme.get("colors", "red"), None);
    }

    #[test]
    fn missing_key_returns_none() {
        let mut theme = Theme::new();
        theme.insert("colors", "red", "#ef4444");
        assert_eq!(theme.get("colors", "blue"), None);
    }

    #[test]
    fn default_is_empty() {
        let theme = Theme::default();
        assert!(theme.sections.is_empty());
    }

    #[test]
    fn insert_multiple_keys_in_section() {
        let mut theme = Theme::new();
        theme.insert("spacing", "1", "0.25rem");
        theme.insert("spacing", "2", "0.5rem");
        assert_eq!(theme.get("spacing", "1"), Some("0.25rem"));
        assert_eq!(theme.get("spacing", "2"), Some("0.5rem"));
    }
}
