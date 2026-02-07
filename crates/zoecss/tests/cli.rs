use std::process::Command;

fn zoecss() -> Command {
    Command::new(env!("CARGO_BIN_EXE_zoecss"))
}

fn fixtures(name: &str) -> String {
    let manifest = env!("CARGO_MANIFEST_DIR");
    format!("{manifest}/../../fixtures/{name}")
}

#[test]
fn sample_html_produces_expected_css() {
    let output = zoecss()
        .arg(fixtures("sample.html"))
        .output()
        .expect("failed to run zoecss");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.starts_with("@layer base, utilities;\n"));

    assert!(
        stdout.contains("@layer base {"),
        "base layer block must be present"
    );
    assert!(
        stdout.contains(":root {"),
        "theme :root block must be present"
    );
    assert!(
        stdout.contains("--spacing-1: 0.25rem;"),
        "spacing custom properties must be emitted"
    );
    assert!(
        stdout.contains("--colors-red: #ef4444;"),
        "color custom properties must be emitted"
    );

    let pos_base = stdout
        .find("@layer base {")
        .expect("base layer block must be present");
    let pos_utilities = stdout
        .find("@layer utilities {")
        .expect("utilities layer block must be present");
    assert!(
        pos_base < pos_utilities,
        "base layer must appear before utilities layer"
    );

    assert!(stdout.contains("  .flex { display: flex; }"));
    assert!(stdout.contains("  .p-4 { padding: 1rem; }"));
    assert!(stdout.contains("  .hover\\:block:hover { display: block; }"));
    assert!(stdout.contains("  .text-\\[\\#ff0000\\] { color: #ff0000; }"));
    assert!(stdout.contains("  @media (min-width: 640px) { .sm\\:grid { display: grid; } }"));
    assert!(stdout.contains("  .hidden { display: none; }"));
    assert!(stdout.contains("  .m-2 { margin: 0.5rem; }"));

    // Rules must appear in first-occurrence order from the source HTML
    let pos_flex = stdout.find("  .flex { display: flex; }").unwrap();
    let pos_p4 = stdout.find("  .p-4 { padding: 1rem; }").unwrap();
    let pos_hover_block = stdout
        .find("  .hover\\:block:hover { display: block; }")
        .unwrap();
    let pos_text = stdout
        .find("  .text-\\[\\#ff0000\\] { color: #ff0000; }")
        .unwrap();
    let pos_sm_grid = stdout
        .find("  @media (min-width: 640px) { .sm\\:grid { display: grid; } }")
        .unwrap();
    let pos_hidden = stdout.find("  .hidden { display: none; }").unwrap();
    let pos_m2 = stdout.find("  .m-2 { margin: 0.5rem; }").unwrap();

    assert!(pos_flex < pos_p4, "flex must come before p-4");
    assert!(pos_p4 < pos_hover_block, "p-4 must come before hover:block");
    assert!(
        pos_hover_block < pos_text,
        "hover:block must come before text-[#ff0000]"
    );
    assert!(
        pos_text < pos_sm_grid,
        "text-[#ff0000] must come before sm:grid"
    );
    assert!(pos_sm_grid < pos_hidden, "sm:grid must come before hidden");
    assert!(pos_hidden < pos_m2, "hidden must come before m-2");
}

#[test]
fn empty_file_produces_no_output() {
    let output = zoecss()
        .arg(fixtures("empty.html"))
        .output()
        .expect("failed to run zoecss");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    // @layer structure is always emitted, even without matches
    assert!(stdout.contains("@layer base, utilities;"));
    assert!(stdout.contains("@layer utilities {}"));

    assert!(
        stdout.contains("@layer base {"),
        "base layer block must be present"
    );
    assert!(
        stdout.contains(":root {"),
        "theme :root block must be present"
    );
}

#[test]
fn missing_file_exits_with_error() {
    let output = zoecss()
        .arg("nonexistent.html")
        .output()
        .expect("failed to run zoecss");

    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("nonexistent.html"));
}

#[test]
fn no_args_exits_with_code_2() {
    let output = zoecss().output().expect("failed to run zoecss");

    assert!(!output.status.success());
    // clap exits with code 2 on missing required arguments
    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn deduplication_across_files() {
    let output = zoecss()
        .arg(fixtures("sample.html"))
        .arg(fixtures("duplicate.html"))
        .output()
        .expect("failed to run zoecss");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.starts_with("@layer base, utilities;"));

    assert!(
        stdout.contains("@layer base {"),
        "base layer block must be present"
    );

    // "flex" in both files, deduplicated to one rule
    let flex_count = stdout.matches(".flex { display: flex; }").count();
    assert_eq!(flex_count, 1, "flex should appear exactly once");

    let p4_count = stdout.matches(".p-4 { padding: 1rem; }").count();
    assert_eq!(p4_count, 1, "p-4 should appear exactly once");

    // "block" only in duplicate.html (hover:block in sample.html is a different token)
    assert!(stdout.contains(".block { display: block; }"));
}
