# zoecss

Command-line interface for [ZoeCSS](../../README.md) — scan source files, extract utility tokens, generate CSS.

## Usage

Pass one or more source files as arguments. Generated CSS is written to stdout.

```sh
zoecss index.html
zoecss src/**/*.html > output.css
zoecss header.html footer.html
```

The CLI is content-type-agnostic — HTML, JSX, Vue, Svelte, or any textual source works.

## Exit codes

| Code | Meaning                                     |
| ---- | ------------------------------------------- |
| `0`  | Success (even if no tokens matched)         |
| `1`  | Runtime error (file not found, I/O failure) |
| `2`  | Argument validation failure                 |
