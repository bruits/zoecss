mod spacing;

use regex::Regex;
use zoecss_config::{Preset, Rule};
use zoecss_core::{CssEntries, CssEntry, Theme};

/// Registers all utility rules for the Tailwind CSS v4 preset.
pub fn register_rules(preset: &mut Preset) {
    // Static display utilities
    for (token, value) in [
        ("flex", "flex"),
        ("block", "block"),
        ("inline", "inline"),
        ("grid", "grid"),
        ("hidden", "none"),
    ] {
        preset.rules.push(Rule::Static {
            token: token.into(),
            entries: CssEntries::new(vec![CssEntry::new("display", value)]),
        });
    }

    // Spacing pattern rules (padding, margin, gap)
    spacing::register(preset);

    // Dynamic rule — arbitrary color via bracket syntax
    preset.rules.push(Rule::Dynamic {
        pattern: r"^text-\[(.+)\]$".into(),
        handler: handle_arbitrary_color,
    });
}

fn handle_arbitrary_color(token: &str, _theme: &Theme) -> Option<CssEntries> {
    use std::sync::LazyLock;
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^text-\[(.+)\]$").expect("valid regex"));
    let caps = RE.captures(token)?;
    let color = caps.get(1)?.as_str();
    Some(CssEntries::new(vec![CssEntry::new(
        "color",
        color.to_owned(),
    )]))
}
