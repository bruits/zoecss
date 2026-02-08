use zoecss_core::Theme;

pub fn register(theme: &mut Theme) {
    for (key, value) in [
        ("xs", "0.125rem"),
        ("sm", "0.25rem"),
        ("md", "0.375rem"),
        ("lg", "0.5rem"),
        ("xl", "0.75rem"),
        ("2xl", "1rem"),
        ("3xl", "1.5rem"),
        ("4xl", "2rem"),
    ] {
        theme.insert("radius", key, value);
    }

    for (key, value) in [
        ("2xs", "0 1px rgb(0 0 0 / 0.05)"),
        ("xs", "0 1px 2px 0 rgb(0 0 0 / 0.05)"),
        (
            "sm",
            "0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)",
        ),
        (
            "md",
            "0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)",
        ),
        (
            "lg",
            "0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)",
        ),
        (
            "xl",
            "0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)",
        ),
        ("2xl", "0 25px 50px -12px rgb(0 0 0 / 0.25)"),
    ] {
        theme.insert("shadow", key, value);
    }

    for (key, value) in [
        ("2xs", "inset 0 1px rgb(0 0 0 / 0.05)"),
        ("xs", "inset 0 1px 1px rgb(0 0 0 / 0.05)"),
        ("sm", "inset 0 2px 4px rgb(0 0 0 / 0.05)"),
    ] {
        theme.insert("inset-shadow", key, value);
    }

    for (key, value) in [
        ("xs", "0 1px 1px rgb(0 0 0 / 0.05)"),
        ("sm", "0 1px 2px rgb(0 0 0 / 0.15)"),
        ("md", "0 3px 3px rgb(0 0 0 / 0.12)"),
        ("lg", "0 4px 4px rgb(0 0 0 / 0.15)"),
        ("xl", "0 9px 7px rgb(0 0 0 / 0.1)"),
        ("2xl", "0 25px 25px rgb(0 0 0 / 0.15)"),
    ] {
        theme.insert("drop-shadow", key, value);
    }

    for (key, value) in [
        ("2xs", "0px 1px 0px rgb(0 0 0 / 0.15)"),
        ("xs", "0px 1px 1px rgb(0 0 0 / 0.2)"),
        (
            "sm",
            "0px 1px 0px rgb(0 0 0 / 0.075), 0px 1px 1px rgb(0 0 0 / 0.075), 0px 2px 2px rgb(0 0 0 / 0.075)",
        ),
        (
            "md",
            "0px 1px 1px rgb(0 0 0 / 0.1), 0px 1px 2px rgb(0 0 0 / 0.1), 0px 2px 4px rgb(0 0 0 / 0.1)",
        ),
        (
            "lg",
            "0px 1px 2px rgb(0 0 0 / 0.1), 0px 3px 2px rgb(0 0 0 / 0.1), 0px 4px 8px rgb(0 0 0 / 0.1)",
        ),
    ] {
        theme.insert("text-shadow", key, value);
    }

    for (key, value) in [
        ("xs", "4px"),
        ("sm", "8px"),
        ("md", "12px"),
        ("lg", "16px"),
        ("xl", "24px"),
        ("2xl", "40px"),
        ("3xl", "64px"),
    ] {
        theme.insert("blur", key, value);
    }

    for (key, value) in [
        ("dramatic", "100px"),
        ("near", "300px"),
        ("normal", "500px"),
        ("midrange", "800px"),
        ("distant", "1200px"),
    ] {
        theme.insert("perspective", key, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zoecss_core::Theme;

    #[test]
    fn shadows() {
        let mut theme = Theme::default();
        register(&mut theme);
        assert!(theme.get("shadow", "md").unwrap().contains("4px 6px -1px"));
        assert!(
            theme
                .get("inset-shadow", "xs")
                .unwrap()
                .starts_with("inset")
        );
        assert!(theme.get("drop-shadow", "sm").is_some());
    }
}
