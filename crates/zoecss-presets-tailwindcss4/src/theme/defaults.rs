use zoecss_core::Theme;

pub fn register(theme: &mut Theme) {
    for (key, value) in [
        (
            "font-family",
            "ui-sans-serif, system-ui, sans-serif, \"Apple Color Emoji\", \"Segoe UI Emoji\", \"Segoe UI Symbol\", \"Noto Color Emoji\"",
        ),
        ("font-feature-settings", "normal"),
        ("font-variation-settings", "normal"),
        (
            "mono-font-family",
            "ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, \"Liberation Mono\", \"Courier New\", monospace",
        ),
        ("mono-font-feature-settings", "normal"),
        ("mono-font-variation-settings", "normal"),
        ("transition-duration", "150ms"),
        ("transition-timing-function", "cubic-bezier(0.4, 0, 0.2, 1)"),
    ] {
        theme.insert("default", key, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zoecss_core::Theme;

    #[test]
    fn transition_entries() {
        let mut theme = Theme::default();
        register(&mut theme);
        assert_eq!(theme.get("default", "transition-duration"), Some("150ms"));
        assert_eq!(
            theme.get("default", "transition-timing-function"),
            Some("cubic-bezier(0.4, 0, 0.2, 1)")
        );
    }
}
