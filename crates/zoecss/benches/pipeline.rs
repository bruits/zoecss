use codspeed_criterion_compat::{Criterion, black_box, criterion_group, criterion_main};

use zoecss_config::{CompiledConfig, Config};
use zoecss_core::generate;
use zoecss_presets::base;

fn compile_base() -> CompiledConfig {
    let mut config = Config::new();
    config.presets.push(base());
    CompiledConfig::compile(config.merge()).expect("base preset compiles")
}

fn bench_compile(c: &mut Criterion) {
    c.bench_function("compile", |b| {
        b.iter(|| {
            let mut config = Config::new();
            config.presets.push(base());
            black_box(CompiledConfig::compile(config.merge()).expect("base preset compiles"))
        });
    });
}

fn bench_generate_tokens(c: &mut Criterion) {
    let compiled = compile_base();

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

fn bench_batch_generate(c: &mut Criterion) {
    let compiled = compile_base();

    let tokens: Vec<&str> = vec![
        // Static
        "flex",
        "block",
        "inline",
        "grid",
        "hidden",
        // Pattern — padding
        "p-1",
        "p-2",
        "p-4",
        "p-8",
        // Pattern — margin
        "m-1",
        "m-2",
        "m-4",
        "m-8",
        // Dynamic bracket syntax
        "text-[#ff0000]",
        "text-[rgb(0,0,0)]",
        "text-[#3b82f6]",
        // Selector variant
        "hover:flex",
        "hover:block",
        "hover:p-4",
        "hover:m-2",
        // At-rule variants
        "sm:flex",
        "sm:block",
        "sm:grid",
        "sm:p-4",
        "md:flex",
        "md:grid",
        "md:p-4",
        // Composed variants
        "sm:hover:flex",
        "sm:hover:p-4",
        "md:hover:flex",
        "md:hover:block",
        // Non-matching
        "nonexistent",
        "p-99",
        "unknown-class",
        "foo:flex",
        // Duplicates for realistic throughput
        "flex",
        "block",
        "grid",
        "hidden",
        "inline",
        "p-1",
        "p-2",
        "p-4",
        "p-8",
        "m-1",
        "m-2",
        "m-4",
        "m-8",
        "text-[#ff0000]",
        "text-[#3b82f6]",
        "hover:flex",
        "hover:block",
        "sm:flex",
        "sm:block",
        "md:flex",
        "md:grid",
        "sm:hover:flex",
        "md:hover:flex",
        "nonexistent",
        "unknown-class",
    ];

    c.bench_function("batch generate", |b| {
        b.iter(|| {
            for &token in &tokens {
                black_box(generate(&compiled, black_box(token)));
            }
        });
    });
}

criterion_group!(
    benches,
    bench_compile,
    bench_generate_tokens,
    bench_batch_generate
);
criterion_main!(benches);
