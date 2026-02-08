use zoecss_core::Theme;

pub fn register(theme: &mut Theme) {
    for (key, value) in [
        ("sm", "40rem"),
        ("md", "48rem"),
        ("lg", "64rem"),
        ("xl", "80rem"),
        ("2xl", "96rem"),
    ] {
        theme.insert("breakpoint", key, value);
    }

    for (key, value) in [
        ("3xs", "16rem"),
        ("2xs", "18rem"),
        ("xs", "20rem"),
        ("sm", "24rem"),
        ("md", "28rem"),
        ("lg", "32rem"),
        ("xl", "36rem"),
        ("2xl", "42rem"),
        ("3xl", "48rem"),
        ("4xl", "56rem"),
        ("5xl", "64rem"),
        ("6xl", "72rem"),
        ("7xl", "80rem"),
    ] {
        theme.insert("container", key, value);
    }

    theme.insert("aspect", "square", "1 / 1");
    theme.insert("aspect", "video", "16 / 9");
}

#[cfg(test)]
mod tests {
    use super::*;
    use zoecss_core::Theme;

    #[test]
    fn breakpoints() {
        let mut theme = Theme::default();
        register(&mut theme);
        assert_eq!(theme.get("breakpoint", "sm"), Some("40rem"));
        assert_eq!(theme.get("breakpoint", "2xl"), Some("96rem"));
    }
}
