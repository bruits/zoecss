use zoecss_config::{CompiledConfig, Config};
use zoecss_core::{extract_tokens, generate};
use zoecss_preset_tailwindcss4::tailwindcss4;

fn fixtures(name: &str) -> String {
    let manifest = env!("CARGO_MANIFEST_DIR");
    format!("{manifest}/../../fixtures/{name}")
}

fn run_pipeline(fixture_name: &str) -> String {
    let path = fixtures(fixture_name);
    let content = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("failed to read fixture {path}: {e}"));
    let tokens = extract_tokens(&content);

    let mut config = Config::new();
    config.presets.push(tailwindcss4());
    let compiled = CompiledConfig::compile(config.merge()).expect("tailwindcss4 preset compiles");

    tokens
        .iter()
        .filter_map(|token| generate(&compiled, token))
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn basic_html_static_rules_only() {
    let output = run_pipeline("basic.html");
    let expected = "\
.flex { display: flex; }\n\
.block { display: block; }\n\
.inline { display: inline; }\n\
.grid { display: grid; }\n\
.hidden { display: none; }";
    assert_eq!(output, expected);
}

#[test]
fn spacing_html_pattern_rules_with_dedup() {
    let output = run_pipeline("spacing.html");
    let expected = "\
.p-1 { padding: 0.25rem; }\n\
.m-2 { margin: 0.5rem; }\n\
.p-4 { padding: 1rem; }\n\
.m-8 { margin: 2rem; }\n\
.px-4 { padding-left: 1rem; padding-right: 1rem; }\n\
.py-2 { padding-top: 0.5rem; padding-bottom: 0.5rem; }\n\
.pt-1 { padding-top: 0.25rem; }\n\
.gap-2 { gap: 0.5rem; }\n\
.mx-8 { margin-left: 2rem; margin-right: 2rem; }\n\
.gap-x-4 { column-gap: 1rem; }";
    assert_eq!(output, expected);
}

#[test]
fn variants_html_selector_and_at_rule() {
    let output = run_pipeline("variants.html");
    let expected = "\
.hover\\:flex:hover { display: flex; }\n\
@media (min-width: 640px) { .sm\\:block { display: block; } }\n\
@media (min-width: 768px) { .md\\:grid { display: grid; } }\n\
@media (min-width: 640px) { .sm\\:hover\\:flex:hover { display: flex; } }";
    assert_eq!(output, expected);
}

#[test]
fn dynamic_html_bracket_syntax() {
    let output = run_pipeline("dynamic.html");
    let expected = "\
.text-\\[\\#ff0000\\] { color: #ff0000; }\n\
.text-\\[rgb\\(0\\,0\\,0\\)\\] { color: rgb(0,0,0); }";
    assert_eq!(output, expected);
}

#[test]
fn mixed_html_full_pipeline() {
    let output = run_pipeline("mixed.html");
    let expected = "\
.flex { display: flex; }\n\
.p-4 { padding: 1rem; }\n\
.hover\\:block:hover { display: block; }\n\
.text-\\[\\#ff0000\\] { color: #ff0000; }\n\
@media (min-width: 640px) { .sm\\:grid { display: grid; } }\n\
.block { display: block; }\n\
.m-2 { margin: 0.5rem; }\n\
@media (min-width: 640px) { .sm\\:hover\\:flex:hover { display: flex; } }\n\
.p-1 { padding: 0.25rem; }\n\
.hidden { display: none; }";
    assert_eq!(output, expected);
}

#[test]
fn no_matches_html_empty_output() {
    let output = run_pipeline("no_matches.html");
    assert_eq!(output, "");
}
