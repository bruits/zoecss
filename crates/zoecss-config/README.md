# zoecss-config

Configuration model, merging, and compilation for [ZoeCSS](../../README.md), with a three-phase pipeline:

```text
Config (build) ──► Config (merge) ──► CompiledConfig (query)
```

1. **Build** — assemble `Preset`s, user rules/variants/theme into a `Config`.
2. **Merge** — `Config::merge()` flattens presets (first = lowest priority) then applies user overrides on top. Theme is deep-merged per key; rules and variants are concatenated in order.
1. **Compile** — `CompiledConfig::compile(config)` produces an immutable, optimized form: `HashMap` for O(1) static lookups, `RegexSet` for single-pass pattern matching.

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

## Types

| Type                | Role                                                         |
| ------------------- | ------------------------------------------------------------ |
| `Config`            | Top-level configuration: presets + user overrides            |
| `Preset`            | Reusable bundle of rules, variants, and theme values         |
| `Rule`              | Maps a utility token to CSS — `Static`, `Pattern`, `Dynamic` |
| `CompiledConfig`    | Immutable runtime form implementing `CssEngine`              |
| `CompiledRegexRule` | A compiled regex rule ready for capture extraction           |
