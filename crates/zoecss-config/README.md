# zoecss-config

This crate provides the types that describe a [ZoeCSS](../../README.md) configuration and a two-phase
pipeline to turn it into a query-ready runtime form:

```text
Config (build) ──► Config (merge) ──► CompiledConfig (query)
```

1. **Build** — assemble `Preset`s, user rules/variants/theme into a `Config`.
2. **Merge** — `Config::merge()` flattens presets (first = lowest priority) then
   applies user overrides on top. Theme is deep-merged per key; rules and variants
   are concatenated in order.
3. **Compile** — `CompiledConfig::compile(config)` produces an immutable, optimized
   form: `HashMap` for O(1) static lookups, `RegexSet` for single-pass pattern
   matching.

## Types

| Type                | Role                                                                        |
| ------------------- | --------------------------------------------------------------------------- |
| `Config`            | Top-level configuration: presets + user overrides                           |
| `Preset`            | Reusable bundle of rules, variants, and theme values                        |
| `Rule`              | Maps a utility token to CSS — `Static`, `Pattern`, or `Dynamic`             |
| `Variant`           | Output modifier — `Selector` (e.g. `&:hover`) or `AtRule` (e.g. `@media …`) |
| `Theme`             | Two-level `section → key → value` store for design tokens                   |
| `CssEntries`        | Ordered list of `CssEntry` property/value pairs                             |
| `CompiledConfig`    | Immutable runtime form, built once, queried many times                      |
| `CompiledRegexRule` | A compiled regex rule ready for capture extraction                          |

## Rule kinds

**Static** — exact token match, O(1) lookup at query time.

```rust
Rule::Static {
    token: "flex".into(),
    entries: CssEntries::new(vec![CssEntry::new("display", "flex")]),
}
```

**Pattern** — regex match with capture-based template substitution (`$1`, `$2`…).

```rust
Rule::Pattern {
    pattern: r"^p-(\d+)$".into(),
    template: CssEntries::new(vec![CssEntry::new("padding", "$1rem")]),
}
```

**Dynamic** — regex match dispatched to a handler function.

```rust
Rule::Dynamic {
    pattern: r"^text-(.+)$".into(),
    handler: |token, theme| { /* … */ },
}
```
