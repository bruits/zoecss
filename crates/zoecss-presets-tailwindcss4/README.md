# zoecss-preset-tailwindcss4

Tailwind CSS v4 preset for [ZoeCSS](../../README.md).

A ready-made bundle of rules, variants, theme values, and a modern CSS reset (preflight) that mirrors Tailwind CSS v4. Refer to Tailwind CSS [documentation](https://tailwindcss.com/docs).

## Usage

`tailwindcss4()` returns the preset:

```rust
use zoecss_preset_tailwindcss4::tailwindcss4;
use zoecss_config::{Config, CompiledConfig};
use zoecss_core::generate;

let mut config = Config::new();
config.presets.push(tailwindcss4());
let engine = CompiledConfig::compile(config.merge()).unwrap();

generate(&engine, "flex");          // → ".flex { display: flex; }"
generate(&engine, "p-4");           // → ".p-4 { padding: 1rem; }"
generate(&engine, "text-[#fff]");   // → ".text-\[\#fff\] { color: #fff; }"
generate(&engine, "hover:flex");    // → ".hover\:flex:hover { display: flex; }"
generate(&engine, "sm:flex");       // → "@media (min-width: 640px) { .sm\:flex { display: flex; } }"
```
