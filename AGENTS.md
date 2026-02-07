# Agents Guide

Principles for how automated agents and contributors generate code and docs here. Favor clarity, small testable units, and the project’s existing conventions.

## Core Engineering Values

- Clarity over cleverness: write idiomatic, expressive, statically typed code.
- Leverage immutability and pattern matching where available; avoid hidden state.
- Prefer small, focused modules and explicit types over “magic”.

## Testing

- Unit-test pure functions and isolated modules.
- Assert observable behavior (inputs/outputs, effects), not internal details.
- Keep tests deterministic and independent of global state.

## Errors

- Typed error enums in library crates (`error.rs`), derived with `thiserror`.
- `anyhow` in the binary crate only, for CLI-level error propagation with context.
- Per-crate `pub type Result<T>` aliases for ergonomic signatures.
- Prefer `?` propagation over `.expect()` / `.unwrap()` in production code. Reserve `.expect()` for cases where failure is a programmer bug (e.g. hardcoded regex literals, test helpers).
- Add context at the boundary (CLI) rather than deep in core, keep library error messages concise.

## Documentation

- Self-documenting code first: expressive names and straightforward logic.
- Comments explain why (intent, invariants, trade‑offs), not how.
- All code, comments, documentation, commit messages, and user-facing output (CLI prompts, logs, errors) must be in English.

## Repository Conventions

- Before generating new code or docs, parse repository to inherit existing conventions and avoid duplication.
- Match the current project structure, naming, and style; do not create parallel patterns.
- Explicit `use` imports for standard library types.

## Changes & Dependencies

- Do not alter CI/CD configuration unless explicitly instructed.
- Only add external dependencies with strong justification and prior discussion. Prefer the standard library and existing utilities.
