mod colors;
mod defaults;
mod effects;
mod layout;
mod spacing;
mod transitions;
mod typography;

use zoecss_core::Theme;

/// Registers all Tailwind CSS v4 default theme values.
pub fn register_theme(theme: &mut Theme) {
    typography::register(theme);
    colors::register(theme);
    layout::register(theme);
    spacing::register(theme);
    effects::register(theme);
    transitions::register(theme);
    defaults::register(theme);
}
