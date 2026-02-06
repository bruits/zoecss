//! CSS declarations produced by rule matches.

use std::borrow::Cow;

/// A single CSS property-value declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CssEntry {
    pub property: Cow<'static, str>,
    pub value: Cow<'static, str>,
}

impl CssEntry {
    pub fn new(
        property: impl Into<Cow<'static, str>>,
        value: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self {
            property: property.into(),
            value: value.into(),
        }
    }
}

/// An ordered list of CSS declarations produced by a rule match.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CssEntries(pub Vec<CssEntry>);

impl CssEntries {
    pub fn new(entries: Vec<CssEntry>) -> Self {
        Self(entries)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entry_from_static_str() {
        let entry = CssEntry::new("display", "flex");
        assert_eq!(entry.property, "display");
        assert_eq!(entry.value, "flex");
    }

    #[test]
    fn entry_from_owned_string() {
        let entry = CssEntry::new(String::from("padding"), String::from("1rem"));
        assert_eq!(entry.property, "padding");
        assert_eq!(entry.value, "1rem");
    }

    #[test]
    fn entries_len_and_empty() {
        let empty = CssEntries::default();
        assert!(empty.is_empty());
        assert_eq!(empty.len(), 0);

        let entries = CssEntries::new(vec![
            CssEntry::new("display", "flex"),
            CssEntry::new("align-items", "center"),
        ]);
        assert!(!entries.is_empty());
        assert_eq!(entries.len(), 2);
    }

    #[test]
    fn default_is_empty() {
        let entries = CssEntries::default();
        assert_eq!(entries.0, Vec::new());
    }
}
