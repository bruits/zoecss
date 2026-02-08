use zoecss_core::Theme;

pub fn register(theme: &mut Theme) {
    for (key, value) in [
        ("in", "cubic-bezier(0.4, 0, 1, 1)"),
        ("out", "cubic-bezier(0, 0, 0.2, 1)"),
        ("in-out", "cubic-bezier(0.4, 0, 0.2, 1)"),
    ] {
        theme.insert("ease", key, value);
    }

    for (key, value) in [
        ("spin", "spin 1s linear infinite"),
        ("ping", "ping 1s cubic-bezier(0, 0, 0.2, 1) infinite"),
        ("pulse", "pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite"),
        ("bounce", "bounce 1s infinite"),
    ] {
        theme.insert("animate", key, value);
    }
}
