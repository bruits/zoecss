use zoecss_core::Theme;

pub fn register(theme: &mut Theme) {
    for (key, value) in [
        (
            "sans",
            "ui-sans-serif, system-ui, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji'",
        ),
        (
            "serif",
            "ui-serif, Georgia, Cambria, 'Times New Roman', Times, serif",
        ),
        (
            "mono",
            "ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono', 'Courier New', monospace",
        ),
    ] {
        theme.insert("font", key, value);
    }

    for (key, value) in [
        ("xs", "0.75rem"),
        ("xs--line-height", "calc(1 / 0.75)"),
        ("sm", "0.875rem"),
        ("sm--line-height", "calc(1.25 / 0.875)"),
        ("base", "1rem"),
        ("base--line-height", "calc(1.5 / 1)"),
        ("lg", "1.125rem"),
        ("lg--line-height", "calc(1.75 / 1.125)"),
        ("xl", "1.25rem"),
        ("xl--line-height", "calc(1.75 / 1.25)"),
        ("2xl", "1.5rem"),
        ("2xl--line-height", "calc(2 / 1.5)"),
        ("3xl", "1.875rem"),
        ("3xl--line-height", "calc(2.25 / 1.875)"),
        ("4xl", "2.25rem"),
        ("4xl--line-height", "calc(2.5 / 2.25)"),
        ("5xl", "3rem"),
        ("5xl--line-height", "1"),
        ("6xl", "3.75rem"),
        ("6xl--line-height", "1"),
        ("7xl", "4.5rem"),
        ("7xl--line-height", "1"),
        ("8xl", "6rem"),
        ("8xl--line-height", "1"),
        ("9xl", "8rem"),
        ("9xl--line-height", "1"),
    ] {
        theme.insert("text", key, value);
    }

    for (key, value) in [
        ("thin", "100"),
        ("extralight", "200"),
        ("light", "300"),
        ("normal", "400"),
        ("medium", "500"),
        ("semibold", "600"),
        ("bold", "700"),
        ("extrabold", "800"),
        ("black", "900"),
    ] {
        theme.insert("font-weight", key, value);
    }

    for (key, value) in [
        ("tighter", "-0.05em"),
        ("tight", "-0.025em"),
        ("normal", "0em"),
        ("wide", "0.025em"),
        ("wider", "0.05em"),
        ("widest", "0.1em"),
    ] {
        theme.insert("tracking", key, value);
    }

    for (key, value) in [
        ("tight", "1.25"),
        ("snug", "1.375"),
        ("normal", "1.5"),
        ("relaxed", "1.625"),
        ("loose", "2"),
    ] {
        theme.insert("leading", key, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zoecss_core::Theme;

    #[test]
    fn font_families() {
        let mut theme = Theme::default();
        register(&mut theme);
        assert!(theme.get("font", "sans").unwrap().contains("ui-sans-serif"));
        assert!(theme.get("font", "serif").unwrap().contains("ui-serif"));
        assert!(theme.get("font", "mono").unwrap().contains("ui-monospace"));
    }

    #[test]
    fn text_sizes_with_line_heights() {
        let mut theme = Theme::default();
        register(&mut theme);
        assert_eq!(theme.get("text", "xs"), Some("0.75rem"));
        assert_eq!(theme.get("text", "xs--line-height"), Some("calc(1 / 0.75)"));
        assert_eq!(theme.get("text", "5xl--line-height"), Some("1"));
    }
}
