# zoecss-core

Core CSS generation engine and foundational types for [ZoeCSS](../../README.md).

## Generation

The `generate` function resolves a utility token (e.g. `hover:flex`) into CSS:

```rust
zoecss_core::generate(&engine, "hover:flex");
// → Some(".hover\\:flex:hover { display: flex; }")
```

It is generic over `CssEngine`, a trait that any configuration backend can implement.
`zoecss-config` provides `CompiledConfig`, the default implementation.

## Types

| Type         | Role                                                                        |
| ------------ | --------------------------------------------------------------------------- |
| `CssEngine`  | Trait — resolves tokens to CSS entries and looks up variants                |
| `CssEntries` | Ordered list of `CssEntry` property/value pairs                             |
| `CssEntry`   | A single CSS declaration (`property: value`)                                |
| `Variant`    | Output modifier — `Selector` (e.g. `&:hover`) or `AtRule` (e.g. `@media …`) |
| `Theme`      | Two-level `section → key → value` store for design tokens                   |
