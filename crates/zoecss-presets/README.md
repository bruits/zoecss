# zoecss-presets

Composable configuration presets for [ZoeCSS](../../README.md).

A preset is a reusable bundle of rules, variants, and theme values that can be plugged into a `Config` before merging. This crate ships ready-made presets, and users can build their own with `zoecss-config::Preset`.

Each preset lives in its own submodule under `src/`, making it straightforward to add new ones alongside the existing ones.

## Tailwind CSS preset

`tailwindcss()` returns a Tailwind CSS compatible preset with utility rules, responsive variants, spacing/color theme values, and a modern CSS reset (preflight):

```rust
use zoecss_presets::tailwindcss;
use zoecss_config::{Config, CompiledConfig};
use zoecss_core::generate;

let mut config = Config::new();
config.presets.push(tailwindcss());
let engine = CompiledConfig::compile(config.merge());

generate(&engine, "flex");          // → ".flex { display: flex; }"
generate(&engine, "p-4");           // → ".p-4 { padding: 1rem; }"
generate(&engine, "text-[#fff]");   // → ".text-\[\#fff\] { color: #fff; }"
generate(&engine, "hover:flex");    // → ".hover\:flex:hover { display: flex; }"
generate(&engine, "sm:flex");       // → "@media (min-width: 640px) { .sm\:flex { display: flex; } }"
```
