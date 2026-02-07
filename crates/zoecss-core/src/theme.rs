//! Theme values organized in two-level sections.

use std::borrow::Cow;

use rustc_hash::FxHashMap;

/// Theme values organized in two-level sections.
///
/// Sections map to nested keys and values, e.g., `"colors" → "red" → "#ef4444"`.
/// Values use `Cow<'static, str>` for zero-copy preset defaults.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Theme {
    pub sections: FxHashMap<Cow<'static, str>, FxHashMap<Cow<'static, str>, Cow<'static, str>>>,
}

impl Theme {
    pub fn new() -> Self {
        Self {
            sections: FxHashMap::default(),
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

    /// Renders all theme entries as CSS custom properties inside a `:root` block.
    ///
    /// Sections and keys are sorted alphabetically for deterministic output.
    /// Returns an empty string when the theme has no entries.
    pub fn to_custom_properties(&self) -> String {
        if self.sections.is_empty() {
            return String::new();
        }

        let mut sections: Vec<_> = self.sections.iter().collect();
        sections.sort_by(|(a, _), (b, _)| a.as_ref().cmp(b.as_ref()));

        let mut lines = Vec::new();
        for (section, entries) in sections {
            let mut keys: Vec<_> = entries.iter().collect();
            keys.sort_by(|(a, _), (b, _)| a.as_ref().cmp(b.as_ref()));
            for (key, value) in keys {
                lines.push(format!("  --{section}-{key}: {value};"));
            }
        }

        format!(":root {{\n{}\n}}", lines.join("\n"))
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

    #[test]
    fn to_custom_properties_empty_theme() {
        let theme = Theme::new();
        assert_eq!(theme.to_custom_properties(), "");
    }

    #[test]
    fn to_custom_properties_single_section() {
        let mut theme = Theme::new();
        theme.insert("colors", "blue", "#3b82f6");
        theme.insert("colors", "red", "#ef4444");
        let output = theme.to_custom_properties();
        let expected = ":root {\n  --colors-blue: #3b82f6;\n  --colors-red: #ef4444;\n}";
        assert_eq!(output, expected);
    }

    #[test]
    fn to_custom_properties_multiple_sections() {
        let mut theme = Theme::new();
        theme.insert("spacing", "2", "0.5rem");
        theme.insert("spacing", "1", "0.25rem");
        theme.insert("colors", "red", "#ef4444");
        theme.insert("colors", "blue", "#3b82f6");
        let output = theme.to_custom_properties();
        let expected = ":root {\n  --colors-blue: #3b82f6;\n  --colors-red: #ef4444;\n  --spacing-1: 0.25rem;\n  --spacing-2: 0.5rem;\n}";
        assert_eq!(output, expected);
    }

    #[test]
    fn to_custom_properties_deterministic_ordering() {
        let mut theme1 = Theme::new();
        theme1.insert("b", "z", "1");
        theme1.insert("a", "y", "2");
        theme1.insert("a", "x", "3");

        let mut theme2 = Theme::new();
        theme2.insert("a", "x", "3");
        theme2.insert("a", "y", "2");
        theme2.insert("b", "z", "1");

        assert_eq!(theme1.to_custom_properties(), theme2.to_custom_properties());
    }
}
