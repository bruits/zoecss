# zoecss-presets

Composable configuration presets for [ZoeCSS](../../README.md).

A preset is a reusable bundle of rules, variants, and theme values that can be plugged into a `Config` before merging. This crate ships ready-made presets, and users can build their own with `zoecss-config::Preset`.

## Base preset

`base()` returns a minimal preset that exercises every rule type and variant kind:

```rust
use zoecss_presets::base;
use zoecss_config::{Config, CompiledConfig};
use zoecss_core::generate;

let mut config = Config::new();
config.presets.push(base());
let engine = CompiledConfig::compile(config.merge());

generate(&engine, "flex");          // → ".flex { display: flex; }"
generate(&engine, "p-4");           // → ".p-4 { padding: 1rem; }"
generate(&engine, "text-[#fff]");   // → ".text-\[\#fff\] { color: #fff; }"
generate(&engine, "hover:flex");    // → ".hover\:flex:hover { display: flex; }"
generate(&engine, "sm:flex");       // → "@media (min-width: 640px) { .sm\:flex { display: flex; } }"
```

This is only intended as a development tool and test fixture. Real presets will be more focused and opinionated, e.g. implementing the Tailwind CSS compatible preset.
