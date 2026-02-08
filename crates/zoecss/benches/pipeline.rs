use std::collections::HashSet;
use std::fs;

use codspeed_criterion_compat::{Criterion, black_box, criterion_group, criterion_main};

use zoecss_config::{CompiledConfig, Config};
use zoecss_core::{extract_tokens, generate};
use zoecss_presets::tailwindcss;

fn compile_tailwindcss() -> CompiledConfig {
    let mut config = Config::new();
    config.presets.push(tailwindcss());
    CompiledConfig::compile(config.merge()).expect("tailwindcss preset compiles")
}

fn bench_compile(c: &mut Criterion) {
    c.bench_function("compile", |b| {
        b.iter(|| {
            let mut config = Config::new();
            config.presets.push(tailwindcss());
            black_box(CompiledConfig::compile(config.merge()).expect("tailwindcss preset compiles"))
        });
    });
}

fn bench_generate_tokens(c: &mut Criterion) {
    let compiled = compile_tailwindcss();

    let tokens: &[(&str, &str)] = &[
        ("flex", "static"),
        ("p-4", "pattern with theme lookup"),
        ("text-[#ff0000]", "dynamic bracket syntax"),
        ("hover:flex", "selector variant"),
        ("sm:flex", "at-rule variant"),
        ("sm:hover:flex", "composed variants"),
        ("nonexistent", "no match"),
    ];

    for &(token, label) in tokens {
        c.bench_function(&format!("generate {label}"), |b| {
            b.iter(|| black_box(generate(&compiled, black_box(token))));
        });
    }
}

fn fixtures(name: &str) -> String {
    let manifest = env!("CARGO_MANIFEST_DIR");
    format!("{manifest}/../../fixtures/{name}")
}

fn bench_extract_tokens(c: &mut Criterion) {
    let mut group = c.benchmark_group("extract");

    let files = ["mixed.html", "no_matches.html"];
    for name in files {
        let content = fs::read_to_string(fixtures(name)).expect("fixture exists");
        group.bench_function(name, |b| {
            b.iter(|| black_box(extract_tokens(black_box(&content))));
        });
    }

    let mixed = fs::read_to_string(fixtures("mixed.html")).expect("fixture exists");
    let scaled = mixed.repeat(100);
    group.bench_function("mixed.html x100", |b| {
        b.iter(|| black_box(extract_tokens(black_box(&scaled))));
    });

    group.finish();
}

fn bench_full_pipeline(c: &mut Criterion) {
    let content = fs::read_to_string(fixtures("mixed.html")).expect("fixture exists");

    c.bench_function("full pipeline", |b| {
        b.iter(|| {
            let mut seen = HashSet::new();
            let mut tokens: Vec<String> = Vec::new();
            for token in extract_tokens(&content) {
                if seen.insert(token.to_owned()) {
                    tokens.push(token.to_owned());
                }
            }

            let mut config = Config::new();
            config.presets.push(tailwindcss());
            let compiled = CompiledConfig::compile(config.merge()).expect("tailwindcss preset compiles");

            let css: Vec<String> = tokens
                .iter()
                .filter_map(|token| generate(&compiled, token))
                .collect();

            black_box(css.join("\n"))
        });
    });
}

criterion_group!(
    benches,
    bench_compile,
    bench_generate_tokens,
    bench_extract_tokens,
    bench_full_pipeline
);
criterion_main!(benches);
